#!/usr/bin/env python3
"""Initialize and track file-guided metaGDB synthesis runs.

This script deliberately contains no model client.  A coding-agent harness reads
the repository workflow and edits the generated candidate; this script only
validates the fixed input, creates the candidate, and records phase progress.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

if __package__:
    from .workflow_graph import (
        PHASES as GRAPH_PHASES, ready_phase, return_affected, return_target,
    )
else:
    from workflow_graph import (
        PHASES as GRAPH_PHASES, ready_phase, return_affected, return_target,
    )


ROOT = Path(__file__).resolve().parent.parent
WORKFLOW_DIR = ROOT / "docs" / "synthesis"
GENERATED_DIR = ROOT / "metagdb_src" / "implementation" / "generated"
CONFIG_DIR = ROOT / "scripts" / "config"
SNAPSHOT_SCRIPT = ROOT / "scripts" / "snapshot.py"
SNAPSHOT_DIR = GENERATED_DIR / ".snapshots"

PHASES = list(GRAPH_PHASES)
LANGUAGES = {"Cypher", "GQL", "Gremlin"}
ENUMS = {
    "schema.mode": {"strict", "hybrid", "schemaless"},
    "transactions.atomicity_scope": {"multi_operation_transaction"},
    "transactions.concurrency_control": {"mvcc", "2pl", "occ", "single_writer"},
    "transactions.isolation_level": {
        "read_committed",
        "repeatable_read",
        "snapshot_isolation",
        "serializable",
    },
    "transactions.durability": {"crash_safe"},
    "execution.model": {"volcano", "vectorized", "push_based"},
    "performance.profile": {
        "point_read_optimized", "traversal_optimized", "scan_analytics",
        "write_optimized", "recovery_optimized", "balanced", "custom",
    },
}
EXPECTED_KEYS = {
    "": {
        "schema_version", "candidate", "language", "schema", "storage",
        "transactions", "execution", "algorithms", "capabilities", "quality",
        "performance", "boundaries", "budget",
    },
    "candidate": {"crate_name", "description"},
    "language": {"name", "grammar_path"},
    "schema": {"mode", "vertex_labels", "edge_types", "property_keys"},
    "storage": {"medium", "layout", "indexes", "persistence", "recovery"},
    "transactions": {
        "atomicity_scope", "concurrency_control", "isolation_level", "durability"
    },
    "execution": {"model"},
    "algorithms": {"required"},
    "capabilities": {"required", "optional", "unsupported_behavior"},
    "quality": {
        "benchmark_min_pass_rate", "system_benchmark_required",
        "required_tests", "zero_compiler_warnings"
    },
    "performance": {
        "profile", "threads", "datasets", "dataset_weights", "cache_modes", "repeat", "workload",
        "primary_metrics", "guardrails", "absolute_targets",
        "max_optimization_iterations", "no_improvement_patience",
    },
    "performance.guardrails": {
        "other_latency_regression_max", "disk_size_regression_max",
        "reopen_time_regression_max", "minimum_primary_improvement",
    },
}

#: Fields a design vector may carry without being required; the comparator
#: supplies the documented default when they are absent.
OPTIONAL_KEYS = {
    "performance": {"topology_downside_penalty"},
    "performance.guardrails": {
        "weighted_latency_regression_max",
        "unweighted_latency_regression_max",
        "minimum_overall_ratio",
        "minimum_topology_robust_gain",
    },
    "boundaries": {
        "allow_framework_changes", "allow_benchmark_changes", "allow_grammar_changes"
    },
    "budget": {"max_fix_iterations"},
}


class DesignError(ValueError):
    """A deterministic validation error in a design vector."""


def _load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise DesignError(f"design vector does not exist: {path}") from exc
    except json.JSONDecodeError as exc:
        raise DesignError(f"invalid JSON at line {exc.lineno}, column {exc.colno}: {exc.msg}") from exc
    if not isinstance(value, dict):
        raise DesignError("design vector root must be a JSON object")
    return value


def _at(value: dict[str, Any], dotted: str) -> Any:
    current: Any = value
    for part in dotted.split("."):
        if not isinstance(current, dict) or part not in current:
            raise DesignError(f"missing required field: {dotted}")
        current = current[part]
    return current


def _require_string(value: dict[str, Any], dotted: str, *, nonempty: bool = True) -> str:
    item = _at(value, dotted)
    if not isinstance(item, str) or (nonempty and not item.strip()):
        raise DesignError(f"{dotted} must be a{' non-empty' if nonempty else ''} string")
    return item


def _require_string_list(value: dict[str, Any], dotted: str) -> list[str]:
    item = _at(value, dotted)
    if not isinstance(item, list) or any(not isinstance(x, str) or not x.strip() for x in item):
        raise DesignError(f"{dotted} must be an array of non-empty strings")
    if len(set(item)) != len(item):
        raise DesignError(f"{dotted} must not contain duplicates")
    return item


def algorithm_specs(design: dict[str, Any]) -> list[dict[str, Any]]:
    """Normalize unrestricted user-requested algorithm descriptions."""
    raw = _at(design, "algorithms.required")
    if not isinstance(raw, list):
        raise DesignError("algorithms.required must be an array")
    normalized: list[dict[str, Any]] = []
    seen: set[str] = set()
    for index, item in enumerate(raw):
        dotted = f"algorithms.required[{index}]"
        if isinstance(item, str):
            name = item.strip()
            if not name:
                raise DesignError(f"{dotted} must not be empty")
            spec: dict[str, Any] = {
                "name": name,
                "description": f"Implement the user-requested {name} algorithm",
                "performance_weight": None,
                "legacy_shorthand": True,
            }
        elif isinstance(item, dict):
            unknown = set(item) - {"name", "description", "performance_weight"}
            missing = {"name", "description"} - set(item)
            if missing:
                raise DesignError(f"{dotted} is missing fields: {', '.join(sorted(missing))}")
            if unknown:
                raise DesignError(f"{dotted} has unknown fields: {', '.join(sorted(unknown))}")
            name = item.get("name")
            description = item.get("description")
            if not isinstance(name, str) or not name.strip():
                raise DesignError(f"{dotted}.name must be a non-empty string")
            if not isinstance(description, str) or not description.strip():
                raise DesignError(f"{dotted}.description must be a non-empty string")
            name = name.strip()
            if not re.fullmatch(r"[A-Za-z][A-Za-z0-9_.-]*", name):
                raise DesignError(
                    f"{dotted}.name may contain only letters, digits, '_', '-' and '.', "
                    "and must begin with a letter"
                )
            weight = item.get("performance_weight")
            if weight is not None and (
                isinstance(weight, bool) or not isinstance(weight, (int, float)) or weight < 0
            ):
                raise DesignError(f"{dotted}.performance_weight must be non-negative")
            spec = {
                "name": name,
                "description": description.strip(),
                "performance_weight": weight,
                "legacy_shorthand": False,
            }
        else:
            raise DesignError(f"{dotted} must be a string or object")
        folded = name.casefold()
        if folded in seen:
            raise DesignError("algorithms.required must not contain duplicate names")
        seen.add(folded)
        normalized.append(spec)

    explicit = [spec["performance_weight"] for spec in normalized]
    if any(weight is not None for weight in explicit):
        if any(weight is None for weight in explicit):
            raise DesignError(
                "either every required algorithm must define performance_weight or none of them"
            )
        if abs(sum(float(weight) for weight in explicit) - 1.0) > 1e-6:
            raise DesignError("algorithm performance_weight values must sum to 1.0")
    elif normalized:
        equal = 1.0 / len(normalized)
        for spec in normalized:
            spec["performance_weight"] = equal
    slugs = [re.sub(r"[^a-z0-9]+", "_", spec["name"].lower()).strip("_") for spec in normalized]
    if len(slugs) != len(set(slugs)):
        raise DesignError("algorithm names must produce distinct performance metric names")
    return normalized


#: Headings that may open the performance section of the final report.
PERFORMANCE_HEADING = re.compile(r"^#{1,3}\s*(?:\d+[.)]\s*)?performance\b.*$", re.IGNORECASE)
NEXT_HEADING = re.compile(r"^#{1,3}\s+\S")
#: A section made only of these carries no evidence.
PLACEHOLDER_LINES = {"pending", "tbd", "todo", "n/a", "none yet", "-", "..."}
#: The phase-13 gate asks the report to state these, whatever the wording.
PERFORMANCE_REQUIREMENTS = (
    ("the dataset and operation weights", ("weight",)),
    ("the accepted and rejected hypotheses", ("hypothes", "accepted", "rejected", "iteration")),
)


def performance_section(report: str) -> list[str]:
    """The body lines of the report's performance section."""
    lines = report.splitlines()
    start = None
    for index, line in enumerate(lines):
        if PERFORMANCE_HEADING.match(line.strip()):
            start = index + 1
            break
    if start is None:
        raise DesignError(
            "final_report.md has no performance section; phase 13 requires one that "
            "states the weights, the iterations, and the remaining limits"
        )
    body: list[str] = []
    for line in lines[start:]:
        if NEXT_HEADING.match(line.strip()):
            break
        body.append(line)
    return body


def check_performance_section(path: Path) -> None:
    try:
        report = path.read_text(encoding="utf-8")
    except OSError as error:
        raise DesignError(f"cannot read {path}: {error}") from error
    body = performance_section(report)
    content = [
        line.strip()
        for line in body
        if line.strip() and line.strip().lower() not in PLACEHOLDER_LINES
    ]
    if len(content) < 4:
        raise DesignError(
            "the performance section of final_report.md is empty or a placeholder; "
            "it must report the measured result, not a promise to measure"
        )
    lowered = "\n".join(content).lower()
    missing = [
        description
        for description, keywords in PERFORMANCE_REQUIREMENTS
        if not any(keyword in lowered for keyword in keywords)
    ]
    if missing:
        raise DesignError(
            "the performance section does not state " + " or ".join(missing)
        )


def check_performance_evidence(metadata: Path, crate: str) -> str:
    """Return the honest phase outcome for current, independently replicated evidence."""
    evidence_path = metadata / "performance_evidence.json"
    evidence = _load_json(evidence_path)
    if evidence.get("schema_version") != 1:
        raise DesignError("performance_evidence.json must use schema_version 1")
    comparison = evidence.get("comparison")
    if not isinstance(comparison, dict):
        raise DesignError("performance_evidence.json has no embedded comparison")
    comparison_path_value = evidence.get("comparison_path")
    comparison_digest = evidence.get("comparison_sha256")
    if not isinstance(comparison_path_value, str) or not comparison_path_value:
        raise DesignError("performance_evidence.json has no comparison_path")
    if not isinstance(comparison_digest, str) or not comparison_digest:
        raise DesignError("performance_evidence.json has no comparison_sha256")
    comparison_path = Path(comparison_path_value)
    if not comparison_path.is_absolute():
        comparison_path = ROOT / comparison_path
    try:
        comparison_bytes = comparison_path.read_bytes()
        recorded_comparison = json.loads(comparison_bytes)
    except (OSError, json.JSONDecodeError) as error:
        raise DesignError(f"cannot read recorded performance comparison: {error}") from error
    if hashlib.sha256(comparison_bytes).hexdigest() != comparison_digest:
        raise DesignError("recorded performance comparison hash does not match the evidence")
    if recorded_comparison != comparison:
        raise DesignError("embedded performance comparison differs from the recorded report")
    baseline = comparison.get("baseline_directories")
    candidate = comparison.get("candidate_directories")
    if not isinstance(baseline, list) or len(set(baseline)) < 2:
        raise DesignError("phase 13 requires at least two independent baseline measurements")
    if not isinstance(candidate, list) or len(set(candidate)) < 2:
        raise DesignError("phase 13 requires at least two independent candidate measurements")
    design_path = metadata / "design_vector.json"
    design_sha = hashlib.sha256(design_path.read_bytes()).hexdigest()
    if comparison.get("design_sha256") != design_sha:
        raise DesignError("performance evidence was measured with a different design vector")
    from build import source_revision
    current = source_revision(crate)
    if evidence.get("current_candidate_revision") != current:
        raise DesignError("performance evidence is stale for the current candidate source")
    if comparison.get("candidate_revisions") != [current]:
        raise DesignError("comparison candidate revision does not match the current source")
    status = comparison.get("status")
    if status == "accepted" and comparison.get("accepted") is True:
        if comparison.get("noise_limited_details"):
            raise DesignError("accepted evidence cannot contain noise-limited metrics")
        if evidence.get("conclusion_scope") != "performance_accepted":
            raise DesignError("accepted evidence has the wrong conclusion scope")
        return "passed"
    if status == "inconclusive" and comparison.get("accepted") is False:
        if not comparison.get("noise_limited_details"):
            raise DesignError("inconclusive evidence must name noise-limited metrics")
        if comparison.get("failure_details") or comparison.get("tentative_failure_details"):
            raise DesignError(
                "real or tentative performance failures cannot be closed as noise-limited"
            )
        if evidence.get("conclusion_scope") != "correctness_complete_performance_inconclusive":
            raise DesignError("inconclusive evidence has the wrong conclusion scope")
        report = "\n".join(performance_section(
            (metadata / "final_report.md").read_text(encoding="utf-8")
        )).lower()
        if "inconclusive" not in report or not any(
            phrase in report for phrase in ("cannot claim", "does not establish", "no performance claim")
        ):
            raise DesignError(
                "an inconclusive run must state in the Performance section that it cannot claim "
                "or does not establish a performance improvement"
            )
        return "completed_with_limitations"
    raise DesignError("canonical performance comparison is neither accepted nor solely noise-limited")


def validate_algorithm_plan(
    design: dict[str, Any], path: Path, *, require_complete: bool
) -> dict[str, Any]:
    """Validate the candidate-local, agent-authored algorithm evidence plan."""
    plan = _load_json(path)
    if set(plan) != {"schema_version", "algorithms"} or plan.get("schema_version") != 1:
        raise DesignError("algorithm plan must contain schema_version=1 and algorithms")
    rows = plan.get("algorithms")
    if not isinstance(rows, list):
        raise DesignError("algorithm plan algorithms must be an array")
    expected = {spec["name"].casefold(): spec for spec in algorithm_specs(design)}
    actual: dict[str, dict[str, Any]] = {}
    required_fields = {
        "name", "description", "core_test", "language_test", "correctness_summary",
        "arguments", "language_query_template", "allow_empty_result",
    }
    for index, row in enumerate(rows):
        dotted = f"algorithm plan algorithms[{index}]"
        if not isinstance(row, dict) or set(row) != required_fields:
            raise DesignError(f"{dotted} must contain exactly: {', '.join(sorted(required_fields))}")
        name = row.get("name")
        if not isinstance(name, str) or name.casefold() not in expected:
            raise DesignError(f"{dotted}.name is not declared by the design vector")
        folded = name.casefold()
        if folded in actual:
            raise DesignError(f"algorithm plan repeats {name}")
        if row.get("description") != expected[folded]["description"]:
            raise DesignError(f"{dotted}.description must match the frozen design vector")
        if not isinstance(row.get("arguments"), dict):
            raise DesignError(f"{dotted}.arguments must be an object")
        for key, value in row["arguments"].items():
            if not isinstance(key, str) or not key:
                raise DesignError(f"{dotted}.arguments keys must be non-empty strings")
            if not (value is None or isinstance(value, (str, int, float, bool))):
                raise DesignError(f"{dotted}.arguments.{key} must be a scalar or null")
            if isinstance(value, str) and value.startswith("$") and value not in {
                "$hotspot_vertex", "$first_vertex", "$vertex_count"
            }:
                raise DesignError(f"{dotted}.arguments.{key} uses an unknown placeholder")
        if not isinstance(row.get("allow_empty_result"), bool):
            raise DesignError(f"{dotted}.allow_empty_result must be boolean")
        if require_complete:
            for field in ("core_test", "language_test", "correctness_summary", "language_query_template"):
                value = row.get(field)
                if not isinstance(value, str) or not value.strip():
                    raise DesignError(f"{dotted}.{field} must be completed by the synthesis agent")
            if row["core_test"] == row["language_test"]:
                raise DesignError(f"{dotted} must use distinct core and language test names")
            if name.casefold() not in row["language_query_template"].casefold():
                raise DesignError(
                    f"{dotted}.language_query_template must visibly invoke the registered "
                    f"algorithm name {name!r}"
                )
        actual[folded] = row
    missing = set(expected) - set(actual)
    extra = set(actual) - set(expected)
    if missing or extra:
        details = []
        if missing:
            details.append("missing " + ", ".join(sorted(missing)))
        if extra:
            details.append("extra " + ", ".join(sorted(extra)))
        raise DesignError("algorithm plan does not match the design vector: " + "; ".join(details))
    return plan


def _inside_root(path_text: str, field: str) -> Path:
    candidate = (ROOT / path_text).resolve()
    try:
        candidate.relative_to(ROOT.resolve())
    except ValueError as exc:
        raise DesignError(f"{field} must stay inside the repository") from exc
    return candidate


def _require_exact_keys(design: dict[str, Any]) -> None:
    for dotted, expected in EXPECTED_KEYS.items():
        value: Any = design if not dotted else _at(design, dotted)
        if not isinstance(value, dict):
            raise DesignError(f"{dotted or 'root'} must be an object")
        allowed = expected | OPTIONAL_KEYS.get(dotted, set())
        missing = expected - set(value)
        extra = set(value) - allowed
        if missing:
            raise DesignError(f"{dotted or 'root'} is missing fields: {', '.join(sorted(missing))}")
        if extra:
            raise DesignError(f"{dotted or 'root'} has unknown fields: {', '.join(sorted(extra))}")


def validate_design(path: Path) -> dict[str, Any]:
    design = _load_json(path)
    _require_exact_keys(design)
    if design.get("schema_version") != 1:
        raise DesignError("schema_version must be 1")

    crate = _require_string(design, "candidate.crate_name")
    _require_string(design, "candidate.description")
    if not re.fullmatch(r"metagdb_[a-z0-9_]+", crate):
        raise DesignError("candidate.crate_name must be a lowercase Rust name beginning with metagdb_")

    language = _require_string(design, "language.name")
    if language not in LANGUAGES:
        raise DesignError(f"language.name must be one of: {', '.join(sorted(LANGUAGES))}")
    grammar_text = _require_string(design, "language.grammar_path")
    grammar = _inside_root(grammar_text, "language.grammar_path")
    if grammar.suffix != ".g4" or not grammar.is_file():
        raise DesignError(f"language.grammar_path must name an existing .g4 file: {grammar_text}")
    if grammar.stem.lower() != language.lower():
        raise DesignError("language.name must agree with the .g4 file name")
    parser_dir = ROOT / "metagdb_src" / "implementation" / "language" / language
    if not (parser_dir / "Cargo.toml").is_file():
        raise DesignError(f"generated parser crate is missing for {language}")

    for dotted, choices in ENUMS.items():
        selected = _require_string(design, dotted)
        if selected not in choices:
            raise DesignError(f"{dotted} must be one of: {', '.join(sorted(choices))}")

    _require_string_list(design, "schema.vertex_labels")
    _require_string_list(design, "schema.edge_types")
    _require_string_list(design, "schema.property_keys")
    _require_string(design, "storage.medium")
    _require_string(design, "storage.layout")
    _require_string_list(design, "storage.indexes")
    _require_string(design, "storage.persistence")
    _require_string(design, "storage.recovery")
    algorithms = algorithm_specs(design)
    required = _require_string_list(design, "capabilities.required")
    optional = _require_string_list(design, "capabilities.optional")
    if set(required) & set(optional):
        raise DesignError("capabilities.required and capabilities.optional must not overlap")
    _require_string(design, "capabilities.unsupported_behavior")
    _require_string_list(design, "quality.required_tests")

    pass_rate = _at(design, "quality.benchmark_min_pass_rate")
    if isinstance(pass_rate, bool) or not isinstance(pass_rate, (int, float)) or not 0 <= pass_rate <= 1:
        raise DesignError("quality.benchmark_min_pass_rate must be a number from 0 to 1")
    if pass_rate < 0.9:
        raise DesignError("quality.benchmark_min_pass_rate must be at least 0.9")
    warnings = _at(design, "quality.zero_compiler_warnings")
    if not isinstance(warnings, bool):
        raise DesignError("quality.zero_compiler_warnings must be boolean")
    if _at(design, "quality.system_benchmark_required") is not True:
        raise DesignError("quality.system_benchmark_required must be true for an ACID GDBMS")

    if _at(design, "performance.threads") != 1:
        raise DesignError("performance.threads must be 1 for the current single-thread experiments")
    repeat = _at(design, "performance.repeat")
    if isinstance(repeat, bool) or not isinstance(repeat, int) or not 3 <= repeat <= 20:
        raise DesignError("performance.repeat must be an integer from 3 to 20")
    max_optimization_iterations = _at(design, "performance.max_optimization_iterations")
    if (
        isinstance(max_optimization_iterations, bool)
        or not isinstance(max_optimization_iterations, int)
        or max_optimization_iterations < 1
    ):
        raise DesignError("performance.max_optimization_iterations must be a positive integer")
    patience = _at(design, "performance.no_improvement_patience")
    if isinstance(patience, bool) or not isinstance(patience, int) or patience < 1:
        raise DesignError("performance.no_improvement_patience must be a positive integer")
    if patience > max_optimization_iterations:
        raise DesignError("performance.no_improvement_patience cannot exceed max_optimization_iterations")
    datasets = _require_string_list(design, "performance.datasets")
    if not datasets:
        raise DesignError("performance.datasets must contain at least one dataset")
    manifest = _load_json(ROOT / "benchmark" / "performance" / "datasets.json")
    known_datasets = set(manifest.get("datasets", {}))
    unknown_datasets = set(datasets) - known_datasets
    if unknown_datasets:
        raise DesignError("performance.datasets contains unknown names: " + ", ".join(sorted(unknown_datasets)))
    dataset_weights = _at(design, "performance.dataset_weights")
    if not isinstance(dataset_weights, dict):
        raise DesignError("performance.dataset_weights must be an object")
    if set(dataset_weights) != set(datasets):
        raise DesignError("performance.dataset_weights must contain exactly the selected datasets")
    for name, weight in dataset_weights.items():
        if isinstance(weight, bool) or not isinstance(weight, (int, float)) or weight < 0:
            raise DesignError(f"performance.dataset_weights.{name} must be a non-negative number")
    if abs(sum(dataset_weights.values()) - 1.0) > 1e-6:
        raise DesignError("performance.dataset_weights must sum to 1.0")
    cache_modes = _require_string_list(design, "performance.cache_modes")
    if not cache_modes:
        raise DesignError("performance.cache_modes must contain at least one mode")
    unknown_cache_modes = set(cache_modes) - {"warm", "reopen"}
    if unknown_cache_modes:
        raise DesignError("performance.cache_modes contains unknown values: " + ", ".join(sorted(unknown_cache_modes)))
    weights = _at(design, "performance.workload")
    if not isinstance(weights, dict):
        raise DesignError("performance.workload must be an object")
    legacy_workload = {
        "storage_import", "storage_vertex_point_read", "storage_edge_point_read",
        "storage_out_adjacency", "storage_hotspot_out_adjacency", "storage_scan",
        "storage_reopen", "storage_disk_size", "query_point_read", "query_one_hop",
        "transaction_commit", "algorithm_projection_build", "algorithm_bfs",
    }
    open_workload = {
        "storage_import", "storage_vertex_point_read", "storage_edge_point_read",
        "storage_out_adjacency", "storage_hotspot_out_adjacency", "storage_scan",
        "storage_reopen", "storage_disk_size", "query_point_read", "query_one_hop",
        "transaction_commit", "algorithm_projection_build", "algorithm_kernel",
        "algorithm_e2e",
    }
    insert_workload = open_workload | {"transaction_edge_insert_window"}
    if set(weights) not in (legacy_workload, open_workload, insert_workload):
        missing = open_workload - set(weights)
        extra = set(weights) - insert_workload
        raise DesignError(
            "performance.workload must use the open algorithm profile; missing "
            + ", ".join(sorted(missing))
            + ("; unknown " + ", ".join(sorted(extra)) if extra else "")
        )
    if set(weights) == legacy_workload and any(not spec["legacy_shorthand"] for spec in algorithms):
        raise DesignError(
            "object-form algorithms require performance.workload.algorithm_kernel and "
            "algorithm_e2e instead of algorithm_bfs"
        )
    if not algorithms and (
        float(weights.get("algorithm_kernel", 0)) > 0
        or float(weights.get("algorithm_e2e", 0)) > 0
    ):
        raise DesignError("algorithm workload weights must be zero when no algorithms are required")
    for name, weight in weights.items():
        if isinstance(weight, bool) or not isinstance(weight, (int, float)) or weight < 0:
            raise DesignError(f"performance.workload.{name} must be a non-negative number")
    if abs(sum(weights.values()) - 1.0) > 1e-6:
        raise DesignError("performance.workload weights must sum to 1.0")
    metric_names = {
        "storage.import.us",
        "storage.vertex_point_read.p95", "storage.edge_point_read.p95",
        "storage.out_adjacency.p95", "storage.hotspot_out_adjacency.p95",
        "storage.scan.p95", "query.point_read.p95",
        "query.one_hop.p95", "transaction.commit.p95", "algorithm.bfs.p95",
        "transaction.edge_insert_window.us_per_edge",
        "algorithm.projection_build.us", "storage.disk_bytes", "storage.reopen.us",
    }
    for spec in algorithms:
        slug = re.sub(r"[^a-z0-9]+", "_", spec["name"].lower()).strip("_")
        metric_names.add(f"algorithm.{slug}.kernel.p95")
        metric_names.add(f"algorithm.{slug}.e2e.p95")
    primary_metrics = _require_string_list(design, "performance.primary_metrics")
    if not primary_metrics:
        raise DesignError("performance.primary_metrics must contain at least one metric")
    unknown_metrics = set(primary_metrics) - metric_names
    if unknown_metrics:
        raise DesignError("performance.primary_metrics contains unknown values: " + ", ".join(sorted(unknown_metrics)))
    for name, ratio in _at(design, "performance.guardrails").items():
        minimum = 0 if name in {"minimum_primary_improvement", "minimum_topology_robust_gain"} else 1
        if isinstance(ratio, bool) or not isinstance(ratio, (int, float)) or not math.isfinite(ratio) or ratio < minimum:
            raise DesignError(f"performance.guardrails.{name} must be at least {minimum}")
    penalty = _at(design, "performance").get("topology_downside_penalty", 0.0)
    if isinstance(penalty, bool) or not isinstance(penalty, (int, float)) or not math.isfinite(penalty) or penalty < 0:
        raise DesignError("performance.topology_downside_penalty must be a finite non-negative number")
    targets = _at(design, "performance.absolute_targets")
    if not isinstance(targets, dict):
        raise DesignError("performance.absolute_targets must be an object")
    for name, target in targets.items():
        if name not in metric_names:
            raise DesignError(f"performance.absolute_targets has unknown metric: {name}")
        if isinstance(target, bool) or not isinstance(target, (int, float)) or target <= 0:
            raise DesignError(f"performance.absolute_targets.{name} must be positive")

    framework_changes = _at(design, "boundaries.allow_framework_changes")
    benchmark_changes = _at(design, "boundaries.allow_benchmark_changes")
    grammar_changes = _at(design, "boundaries.allow_grammar_changes")
    if any(not isinstance(x, bool) for x in (framework_changes, benchmark_changes, grammar_changes)):
        raise DesignError("all boundaries.allow_* fields must be boolean")
    if framework_changes or benchmark_changes or grammar_changes:
        raise DesignError("this workflow requires all boundaries.allow_* fields to be false")

    max_iterations = _at(design, "budget.max_fix_iterations")
    if isinstance(max_iterations, bool) or not isinstance(max_iterations, int) or max_iterations < 0:
        raise DesignError("budget.max_fix_iterations must be a non-negative integer (0 means unlimited)")
    return design


def _metadata_dir(crate: str) -> Path:
    if not re.fullmatch(r"metagdb_[a-z0-9_]+", crate):
        raise DesignError("crate must be a lowercase Rust name beginning with metagdb_")
    return GENERATED_DIR / crate / ".synthesis"


def _now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _source_revision(crate: str) -> str:
    if __package__:
        from .build import source_revision
    else:
        from build import source_revision
    return source_revision(crate)


def _write_state(metadata: Path, state: dict[str, Any]) -> None:
    (metadata / "state.json").write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
    current = next((p["name"] for p in state["phases"] if p["status"] == "active"), "complete")
    lines = [
        "# Synthesis state",
        "",
        f"- Candidate: `{state['crate_name']}`",
        f"- Current phase: `{current}`",
        f"- Updated: `{state['updated_at']}`",
        "",
        "| Phase | Status | Evidence |",
        "|---|---|---|",
    ]
    for phase in state["phases"]:
        evidence = str(phase.get("evidence", "")).replace("|", "\\|").replace("\n", " ")
        lines.append(f"| `{phase['name']}` | {phase['status']} | {evidence} |")
    if state.get("schema_version") == 2:
        lines.extend([
            "",
            "Generation prerequisites and upstream return edges are defined in",
            "`scripts/workflow_graph.py`. Each transition needs a phase review and a",
            "fresh, tool-run validation gate. Failed gates leave the phase active.",
            "If the active stage finds an upstream incompatibility, `revisit` returns",
            "to its owner and invalidates affected passing evidence before re-generation.",
            "",
            "```bash",
            "python3 scripts/synthesis.py advance <crate> --phase <current-phase> --review <candidate>/.synthesis/reviews/<current-phase>.json",
            "python3 scripts/synthesis.py revisit <crate> --phase <current-phase> --to <earlier-phase> --review <candidate>/.synthesis/reviews/<current-phase>.json",
            "```",
            "",
        ])
    else:
        lines.extend([
        "",
        "`state.json` is authoritative. Advance only after the current phase gate passes.",
        "A successful advance automatically commits the candidate and state to its isolated Git:",
        "",
        "```bash",
        "python3 scripts/synthesis.py advance <crate> --phase <current-phase> --evidence \"command/result\"",
        "```",
        "",
        ])
    (metadata / "state.md").write_text("\n".join(lines), encoding="utf-8")


def _render_template(name: str, replacements: dict[str, str]) -> str:
    text = (WORKFLOW_DIR / "templates" / name).read_text(encoding="utf-8")
    for key, value in replacements.items():
        text = text.replace("{{" + key + "}}", value)
    return text


def cmd_check(args: argparse.Namespace) -> int:
    try:
        design = validate_design(Path(args.design).resolve())
    except DesignError as exc:
        print(f"Invalid design vector: {exc}", file=sys.stderr)
        return 2
    print(
        f"Valid design vector: {design['candidate']['crate_name']} / "
        f"{design['language']['name']} / threshold "
        f"{design['quality']['benchmark_min_pass_rate']:.1%}"
    )
    return 0


def cmd_init(args: argparse.Namespace) -> int:
    design_path = Path(args.design).resolve()
    try:
        design = validate_design(design_path)
    except DesignError as exc:
        print(f"Invalid design vector: {exc}", file=sys.stderr)
        return 2
    crate = design["candidate"]["crate_name"]
    candidate = GENERATED_DIR / crate
    metadata = _metadata_dir(crate)
    snapshot_dir = SNAPSHOT_DIR / f"{crate}.git"
    if candidate.exists():
        print(f"Error: candidate already exists: {candidate.relative_to(ROOT)}", file=sys.stderr)
        print("Use status to resume it; choose a new crate name for a new experiment.", file=sys.stderr)
        return 2
    if snapshot_dir.exists():
        print(f"Error: candidate snapshot history already exists: {snapshot_dir.relative_to(ROOT)}", file=sys.stderr)
        print("Restore its matching candidate or choose a new crate name; history was not deleted.", file=sys.stderr)
        return 2

    result = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "build.py"), "create", crate, "--language", design["language"]["name"]],
        cwd=ROOT,
    )
    if result.returncode != 0:
        return result.returncode

    try:
        metadata.mkdir(parents=True)
        (metadata / "design_vector.json").write_text(
            json.dumps(design, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
        )
        source_request = getattr(args, "source_request", None)
        if source_request is not None:
            (metadata / "source_request.json").write_text(
                json.dumps(source_request, indent=2, ensure_ascii=False) + "\n",
                encoding="utf-8",
            )
        algorithm_plan = {
            "schema_version": 1,
            "algorithms": [
                {
                    "name": spec["name"],
                    "description": spec["description"],
                    "core_test": "",
                    "language_test": "",
                    "correctness_summary": "",
                    "arguments": {},
                    "language_query_template": "",
                    "allow_empty_result": False,
                }
                for spec in algorithm_specs(design)
            ],
        }
        (metadata / "algorithm_plan.json").write_text(
            json.dumps(algorithm_plan, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
        )
        replacements = {
            "CRATE_NAME": crate,
            "LANGUAGE": design["language"]["name"],
            "LANGUAGE_LOWER": design["language"]["name"].lower(),
            "GRAMMAR_PATH": design["language"]["grammar_path"],
            "BENCHMARK_THRESHOLD": str(design["quality"]["benchmark_min_pass_rate"]),
        }
        for template, output in (
            ("capability_matrix.tsv", "capability_matrix.tsv"),
            ("decisions.md", "decisions.md"),
            ("final_report.md", "final_report.md"),
        ):
            (metadata / output).write_text(_render_template(template, replacements), encoding="utf-8")
        state = {
            "schema_version": 2,
            "crate_name": crate,
            "design_sha256": hashlib.sha256((metadata / "design_vector.json").read_bytes()).hexdigest(),
            "created_at": _now(),
            "updated_at": _now(),
            "workflow_graph_version": 3,
            "transitions": [],
            "phases": [
                {
                    "name": phase,
                    "status": "active" if index == 0 else "pending",
                    "evidence": "",
                }
                for index, phase in enumerate(PHASES)
            ],
        }
        _write_state(metadata, state)
        snapshot = subprocess.run(
            [sys.executable, str(SNAPSHOT_SCRIPT), "init", crate],
            cwd=ROOT,
        )
        if snapshot.returncode != 0:
            raise RuntimeError("failed to initialize isolated candidate snapshot history")
    except Exception:
        import shutil

        shutil.rmtree(candidate, ignore_errors=True)
        shutil.rmtree(snapshot_dir, ignore_errors=True)
        (CONFIG_DIR / f"{crate}.toml").unlink(missing_ok=True)
        raise

    print("\nSynthesis run initialized.")
    print(f"Candidate: {candidate.relative_to(ROOT)}")
    print(f"Frozen design: {(metadata / 'design_vector.json').relative_to(ROOT)}")
    print("Agent entry: docs/synthesis/README.md")
    print(f"Current phase: docs/synthesis/phases/{PHASES[0]}.md")
    return 0


def cmd_start(args: argparse.Namespace) -> int:
    """One agent-facing entry for requirement intake and graph workflow init."""
    source_kind = "compile" if args.requirements else "from-text" if args.text else "from-paper"
    source = Path(args.requirements or args.text or args.paper).resolve()
    if not source.is_file():
        print(f"Error: requirement source does not exist: {source}", file=sys.stderr)
        return 2
    run_dir = ROOT / "design_runs" / args.run_id
    vector = run_dir / "design_vector.json"
    command = [sys.executable, str(ROOT / "scripts" / "design.py"), source_kind, str(source)]
    if source_kind != "compile":
        if not args.crate:
            print("Error: --crate is required for text or paper input.", file=sys.stderr)
            return 2
        command.extend(["--crate", args.crate, "--mode", args.mode,
                        "--max-repair-rounds", str(args.max_repair_rounds)])
        if args.language:
            command.extend(["--language", args.language])
        if args.llm_fallback:
            command.append("--llm-fallback")
        if getattr(args, "resume", False):
            command.append("--resume")
        if args.env_file:
            command.extend(["--env-file", args.env_file])
    command.extend(["--run-id", args.run_id])
    result = subprocess.run(command, cwd=ROOT)
    if result.returncode != 0:
        return result.returncode
    try:
        design = validate_design(vector)
    except DesignError as exc:
        print(f"Error: design intake produced an invalid vector: {exc}", file=sys.stderr)
        return 2
    if args.crate and design["candidate"]["crate_name"] != args.crate:
        print("Error: design candidate differs from requested crate.", file=sys.stderr)
        return 2
    source_request = {
        "schema_version": 1, "kind": source_kind, "source": str(source),
        "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "design_run_id": args.run_id, "design_mode": args.mode if source_kind != "compile" else "constraint",
        "design_sha256": hashlib.sha256(vector.read_bytes()).hexdigest(),
    }
    return cmd_init(argparse.Namespace(design=str(vector), source_request=source_request))


def _load_state(crate: str) -> tuple[Path, dict[str, Any]]:
    metadata = _metadata_dir(crate)
    path = metadata / "state.json"
    if not path.is_file():
        raise DesignError(f"no initialized synthesis state for {crate}")
    state = _load_json(path)
    if state.get("schema_version") not in {1, 2}:
        raise DesignError("unsupported synthesis state schema_version")
    if state.get("schema_version") == 2 and state.get("design_sha256"):
        try:
            frozen = hashlib.sha256((metadata / "design_vector.json").read_bytes()).hexdigest()
        except OSError as exc:
            raise DesignError(f"cannot read frozen design vector: {exc}") from exc
        if frozen != state["design_sha256"]:
            raise DesignError("frozen design vector changed after initialization")
    return metadata, state


def cmd_status(args: argparse.Namespace) -> int:
    try:
        metadata, state = _load_state(args.crate)
    except DesignError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 2
    active = next((p for p in state["phases"] if p["status"] == "active"), None)
    completed = sum(
        p["status"] in {"passed", "completed_with_limitations"}
        for p in state["phases"]
    )
    limited = sum(p["status"] == "completed_with_limitations" for p in state["phases"])
    print(f"Candidate: {args.crate}")
    print(f"Progress: {completed}/{len(state['phases'])} phases completed")
    if limited:
        print(f"Completed with limitations: {limited} phase(s)")
    if state.get("schema_version") == 2:
        print(f"Workflow: graph with validated transitions; revisits: {sum(t['kind'] == 'feedback' for t in state['transitions'])}")
    print(f"Current phase: {active['name'] if active else 'complete'}")
    print(f"State: {(metadata / 'state.md').relative_to(ROOT)}")
    print(f"Snapshots: python3 scripts/snapshot.py list {args.crate}")
    if active:
        print(f"Instruction: docs/synthesis/phases/{active['name']}.md")
    return 0


def _phase_review(metadata: Path, phase: str, path_value: str | None, decision: str) -> tuple[dict[str, Any], str]:
    if not path_value:
        raise DesignError("graph workflow requires --review <candidate>/.synthesis/reviews/<phase>.json")
    path = Path(path_value).resolve()
    review_dir = (metadata / "reviews").resolve()
    if not path.is_relative_to(review_dir) or path.name != f"{phase}.json":
        raise DesignError(f"review must be {review_dir / (phase + '.json')}")
    raw = path.read_bytes()
    try:
        review = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise DesignError(f"invalid phase review: {exc}") from exc
    if not isinstance(review, dict) or review.get("schema_version") != 1:
        raise DesignError("phase review needs schema_version=1")
    if review.get("phase") != phase or review.get("decision") != decision:
        raise DesignError("phase review must name the active phase and decision")
    observations = review.get("observations")
    if not isinstance(observations, list) or not observations or any(
        not isinstance(item, str) or not item.strip() for item in observations
    ):
        raise DesignError("phase review needs non-empty observations")
    if not isinstance(review.get("next_action"), str) or not review["next_action"].strip():
        raise DesignError("phase review needs a concrete next_action")
    if not isinstance(review.get("self_assessment"), str) or not review["self_assessment"].strip():
        raise DesignError("phase review needs a self_assessment of evidence and limitations")
    if decision == "revisit" and (
        not isinstance(review.get("diagnosis"), str) or not review["diagnosis"].strip()
    ):
        raise DesignError("feedback review needs a concrete diagnosis")
    if decision == "advance" and phase == "00_foundation" and (
        not isinstance(review.get("design_self_assessment"), str)
        or not review["design_self_assessment"].strip()
    ):
        raise DesignError("foundation review must reflect on the frozen design vector")
    risks = review.get("remaining_risks")
    if not isinstance(risks, list) or any(not isinstance(item, str) or not item.strip() for item in risks):
        raise DesignError("phase review needs remaining_risks as a string array")
    return review, hashlib.sha256(raw).hexdigest()


def _gate_commands(crate: str, phase: str, metadata: Path) -> list[list[str]]:
    if PHASES.index(phase) <= PHASES.index("10_assembly"):
        return [["cargo", "test", "-p", crate, "--quiet"]]
    verify = [sys.executable, str(ROOT / "scripts" / "build.py"), "verify", crate]
    if phase == "11_build_integration":
        return [verify]
    design = _load_json(metadata / "design_vector.json")
    threshold = str(design["quality"]["benchmark_min_pass_rate"])
    shared = [
        verify,
        [sys.executable, str(ROOT / "scripts" / "build.py"), "benchmark", crate,
         "--min-pass-rate", threshold],
        [sys.executable, str(ROOT / "scripts" / "build.py"), "system-benchmark", crate,
         "--design", str(metadata / "design_vector.json")],
    ]
    if algorithm_specs(design):
        shared.append([
            sys.executable, str(ROOT / "scripts" / "build.py"), "algorithm-check", crate,
            "--design", str(metadata / "design_vector.json"),
        ])
    return shared


def _run_phase_gate(metadata: Path, crate: str, phase: str) -> str:
    """Run fixed checks independently of the agent's self-review."""
    review_dir = metadata / "reviews"
    review_dir.mkdir(exist_ok=True)
    log_path = review_dir / f"{phase}.gate.log"
    report_path = review_dir / f"{phase}.gate.json"
    results = []
    with log_path.open("w", encoding="utf-8") as log:
        for command in _gate_commands(crate, phase, metadata):
            print(f"$ {' '.join(command)}", file=log, flush=True)
            started = _now()
            result = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
            results.append({
                "command": command, "started_at": started, "finished_at": _now(),
                "exit_code": result.returncode,
            })
            print(f"exit_code={result.returncode}\n", file=log, flush=True)
            if result.returncode != 0:
                break
    report = {
        "schema_version": 1, "phase": phase, "candidate_revision": _source_revision(crate),
        "checks": results, "log_sha256": hashlib.sha256(log_path.read_bytes()).hexdigest(),
        "passed": bool(results) and all(row["exit_code"] == 0 for row in results),
    }
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if not report["passed"]:
        raise DesignError(f"phase gate failed; inspect {log_path}")
    return hashlib.sha256(report_path.read_bytes()).hexdigest()


def cmd_advance(args: argparse.Namespace) -> int:
    try:
        metadata, state = _load_state(args.crate)
    except DesignError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 2
    active_index = next(
        (index for index, phase in enumerate(state["phases"]) if phase["status"] == "active"),
        None,
    )
    if active_index is None:
        print("Error: synthesis run is already complete.", file=sys.stderr)
        return 2
    active = state["phases"][active_index]
    if args.phase != active["name"]:
        print(f"Error: current phase is {active['name']}, not {args.phase}.", file=sys.stderr)
        return 2
    graph_workflow = state.get("schema_version") == 2
    evidence = (args.evidence or "").strip()
    if not graph_workflow and not evidence:
        print("Error: gate evidence must not be empty.", file=sys.stderr)
        return 2
    review_hash = gate_hash = ""
    if graph_workflow:
        try:
            review, review_hash = _phase_review(metadata, args.phase, args.review, "advance")
            gate_hash = _run_phase_gate(metadata, args.crate, args.phase)
            evidence = "; ".join(review["observations"])
        except (DesignError, OSError, ValueError) as exc:
            print(f"Error: phase self-check failed: {exc}", file=sys.stderr)
            return 2
    if args.phase in {"03_algorithms", "09_executor_tests", "12_benchmark_fix"}:
        try:
            design = _load_json(metadata / "design_vector.json")
            plan = validate_algorithm_plan(
                design,
                metadata / "algorithm_plan.json",
                require_complete=args.phase in {"09_executor_tests", "12_benchmark_fix"},
            )
            if args.phase == "03_algorithms":
                for row in plan["algorithms"]:
                    if not str(row["core_test"]).strip() or not str(row["correctness_summary"]).strip():
                        raise DesignError(
                            f"algorithm {row['name']} needs core_test and correctness_summary "
                            "before phase 03 can pass"
                        )
            if args.phase == "12_benchmark_fix" and plan["algorithms"] and not graph_workflow:
                check = subprocess.run(
                    [
                        sys.executable,
                        str(ROOT / "scripts" / "build.py"),
                        "algorithm-check",
                        args.crate,
                        "--design",
                        str(metadata / "design_vector.json"),
                    ],
                    cwd=ROOT,
                )
                if check.returncode != 0:
                    raise DesignError("generated algorithm tests did not pass")
        except DesignError as exc:
            print(f"Error: algorithm evidence gate failed: {exc}", file=sys.stderr)
            return 2
    phase_outcome = "passed"
    if args.phase == "13_performance_optimization":
        try:
            check_performance_section(metadata / "final_report.md")
            phase_outcome = check_performance_evidence(metadata, args.crate)
        except (DesignError, OSError) as exc:
            print(f"Error: performance evidence gate failed: {exc}", file=sys.stderr)
            return 2
    state_json_path = metadata / "state.json"
    state_markdown_path = metadata / "state.md"
    old_state_json = state_json_path.read_text(encoding="utf-8")
    old_state_markdown = state_markdown_path.read_text(encoding="utf-8")

    active["status"] = phase_outcome
    active["evidence"] = evidence
    if graph_workflow:
        active["review_sha256"] = review_hash
        active["gate_sha256"] = gate_hash
        active["candidate_revision"] = _source_revision(args.crate)
        next_name = ready_phase(state["phases"])
        if next_name:
            next(row for row in state["phases"] if row["name"] == next_name)["status"] = "active"
        elif any(row["status"] == "pending" for row in state["phases"]):
            print("Error: workflow graph has pending phases with unmet dependencies.", file=sys.stderr)
            return 2
        state["transitions"].append({
            "kind": "advance", "from": args.phase, "to": next_name or "complete",
            "review_sha256": review_hash, "gate_sha256": gate_hash,
            "candidate_revision": active["candidate_revision"], "at": _now(),
        })
    else:
        if active_index + 1 < len(state["phases"]):
            state["phases"][active_index + 1]["status"] = "active"
    state["updated_at"] = _now()
    _write_state(metadata, state)
    snapshot = subprocess.run(
        [
            sys.executable,
            str(SNAPSHOT_SCRIPT),
            "checkpoint",
            args.crate,
            "--name",
            args.phase,
            "--label",
            f"phase {args.phase} {phase_outcome}",
        ],
        cwd=ROOT,
    )
    if snapshot.returncode != 0:
        state_json_path.write_text(old_state_json, encoding="utf-8")
        state_markdown_path.write_text(old_state_markdown, encoding="utf-8")
        print(
            "Error: phase state was not advanced because its isolated snapshot failed.",
            file=sys.stderr,
        )
        return snapshot.returncode
    if not graph_workflow:
        next_name = (
            state["phases"][active_index + 1]["name"]
            if active_index + 1 < len(state["phases"])
            else "complete"
        )
    print(f"Completed {args.phase} as {phase_outcome}. Next: {next_name}")
    return 0



def cmd_revisit(args: argparse.Namespace) -> int:
    """Follow an upstream diagnostic edge at any active generation node."""
    try:
        metadata, state = _load_state(args.crate)
        if state.get("schema_version") != 2:
            raise DesignError("feedback routing is available only for graph-workflow runs")
        active = next((row for row in state["phases"] if row["status"] == "active"), None)
        if not active or active["name"] != args.phase:
            raise DesignError("feedback must originate at the current active phase")
        target = return_target(args.phase, args.to)
        review, review_hash = _phase_review(metadata, args.phase, args.review, "revisit")
        if review.get("target") != target:
            raise DesignError("review target does not match the agent-selected return edge")
    except (DesignError, OSError, ValueError) as exc:
        print(f"Error: feedback review failed: {exc}", file=sys.stderr)
        return 2
    state_json_path = metadata / "state.json"
    state_markdown_path = metadata / "state.md"
    old_json = state_json_path.read_text(encoding="utf-8")
    old_markdown = state_markdown_path.read_text(encoding="utf-8")
    affected = return_affected(args.phase, target)
    for row in state["phases"]:
        if row["name"] in affected:
            row["status"] = "pending"
            row["evidence"] = ""
            for key in ("review_sha256", "gate_sha256", "candidate_revision"):
                row.pop(key, None)
    selected = next(row for row in state["phases"] if row["name"] == target)
    selected["status"] = "active"
    state["transitions"].append({
        "kind": "feedback", "from": args.phase, "to": target,
        "diagnosis": review["diagnosis"], "invalidated": sorted(affected),
        "review_sha256": review_hash, "candidate_revision": _source_revision(args.crate),
        "at": _now(),
    })
    state["updated_at"] = _now()
    _write_state(metadata, state)
    snapshot = subprocess.run([
        sys.executable, str(SNAPSHOT_SCRIPT), "checkpoint", args.crate,
        "--name", f"feedback-{len(state['transitions'])}",
        "--label", f"feedback {args.phase} to {target}",
    ], cwd=ROOT)
    if snapshot.returncode != 0:
        state_json_path.write_text(old_json, encoding="utf-8")
        state_markdown_path.write_text(old_markdown, encoding="utf-8")
        print("Error: feedback state was not changed because its snapshot failed.", file=sys.stderr)
        return snapshot.returncode
    print(f"Revisit {target} from {args.phase}; revalidate {len(affected)} affected phases.")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate, initialize, and track a file-guided GDBMS synthesis run."
    )
    sub = parser.add_subparsers(dest="command", required=True)
    check = sub.add_parser("check", help="validate a finalized design vector")
    check.add_argument("--design", required=True, help="path to design-vector JSON")
    init = sub.add_parser("init", help="validate d, create a candidate, and initialize phase state")
    init.add_argument("--design", required=True, help="path to design-vector JSON")
    start = sub.add_parser("start", help="intake requirements and initialize one graph-workflow run")
    source = start.add_mutually_exclusive_group(required=True)
    source.add_argument("--requirements", help="structured requirement JSON")
    source.add_argument("--text", help="natural-language requirement file")
    source.add_argument("--paper", help="paper PDF or text file")
    start.add_argument("--crate", help="required for text or paper; checked for structured input")
    start.add_argument("--language", choices=sorted(LANGUAGES))
    start.add_argument("--mode", choices=("constraint", "llm-direct"), default="constraint")
    start.add_argument("--llm-fallback", action="store_true")
    start.add_argument("--resume", action="store_true", help="resume failed llm-direct design intake")
    start.add_argument("--max-repair-rounds", type=int, default=3)
    start.add_argument("--env-file")
    start.add_argument("--run-id", required=True)
    status = sub.add_parser("status", help="show the current phase of an initialized candidate")
    status.add_argument("crate", help="candidate crate name")
    advance = sub.add_parser("advance", help="record a passed phase and activate the next one")
    advance.add_argument("crate", help="candidate crate name")
    advance.add_argument("--phase", required=True, choices=PHASES)
    advance.add_argument("--evidence", help="legacy phase evidence; v2 uses --review")
    advance.add_argument("--review", help="candidate-local structured self-review for graph workflow")
    revisit = sub.add_parser("revisit", help="return from any active phase to a diagnosed upstream owner")
    revisit.add_argument("crate", help="candidate crate name")
    revisit.add_argument("--phase", required=True, choices=PHASES)
    revisit.add_argument("--to", required=True, choices=PHASES)
    revisit.add_argument("--review", required=True)
    args = parser.parse_args(argv)
    return {
        "check": cmd_check,
        "init": cmd_init,
        "start": cmd_start,
        "status": cmd_status,
        "advance": cmd_advance,
        "revisit": cmd_revisit,
    }[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
