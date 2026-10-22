#!/usr/bin/env python3
"""Compare fixed metaGDB micro-benchmark runs using design-vector weights.

The comparator answers two separate questions and keeps them separate:

* **Score.** The legacy weighted geometric mean remains in the report. New
  designs may also use a topology-robust log gain: weighted log speedups minus
  a penalty for graph scenarios whose weighted operations regress together.
* **Guardrails.** Did anything outside the optimization target break? A naive
  reading of this question -- "no metric may be slower than ``baseline * 1.2``
  in one pair of runs" -- cannot be answered by this benchmark, because the
  same binary measured twice on the same machine spreads well beyond 1.2x for
  a large share of its metrics. A guardrail tighter than the measurement noise
  rejects at random and says nothing about the candidate.

So the guardrail decision is stated in terms of what the environment can
actually resolve:

1. **Scope.** A hard cap applies to metrics the design protects explicitly
   (disk size, reopen) and to metrics with a nonzero workload weight. A metric
   the design scores at weight zero only has a loose catastrophic cap: it must
   not veto an experiment it does not participate in.
2. **Measured noise floor.** Each side may be measured more than once (pass
   ``--baseline``/``--candidate`` several times). Every metric is compared on
   the median across the given directories, and the largest spread on either
   side is that metric's noise floor. The effective tolerance is
   ``max(configured_cap, noise_floor)``, so a difference the environment cannot
   reproduce is never called a regression. Metrics whose own noise floor exceeds
   the configured cap are reported as ``noise_limited``: the guardrail could not
   be evaluated for them, and the report says so instead of guessing.

   Replicates are only replicates if they measured the same binary and the same
   design, so every directory's ``run.json`` is read first: a side whose
   directories carry different source revisions is refused rather than averaged,
   because the median of two different binaries is not a measurement of either.
   Directories written with ``--reuse-data`` carry a stale import value, so a
   comparison containing them is refused unless ``--allow-reused`` says the run
   is exploratory.
3. **Reproduce before rejecting.** A measured failure is tentative on the first
   comparison (exit code 3). It becomes fatal only when a second, independent
   comparison shows the same failure for the same dataset and metric
   (``--confirm-with``). Structural failures -- a metric that did not run, an
   absolute target the user declared -- are fatal immediately, because they are
   not measurements.
4. **No net regression.** The weighted geometric mean over the metrics the
   design cares about may not fall below ``minimum_overall_ratio`` (default
   1.0). New designs additionally require nonnegative topology-robust log gain:
   weighted log speedup minus a one-sided squared penalty for graph scenarios
   whose weighted operations regress together.

Exit codes: 0 accepted, 1 rejected (confirmed failures), 2 comparison error,
3 provisional (failures need an independent measurement before they are fatal),
4 inconclusive (required metrics are noise-limited).
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import re
import statistics
import sys
from pathlib import Path
from typing import Any


WORKLOAD_METRICS = {
    "storage_import": "storage.import.us",
    "storage_vertex_point_read": "storage.vertex_point_read.p95",
    "storage_edge_point_read": "storage.edge_point_read.p95",
    "storage_out_adjacency": "storage.out_adjacency.p95",
    "storage_hotspot_out_adjacency": "storage.hotspot_out_adjacency.p95",
    "storage_scan": "storage.scan.p95",
    "storage_reopen": "storage.reopen.us",
    "storage_disk_size": "storage.disk_bytes",
    "query_point_read": "query.point_read.p95",
    "query_one_hop": "query.one_hop.p95",
    "transaction_commit": "transaction.commit.p95",
    "transaction_edge_insert_window": "transaction.edge_insert_window.us_per_edge",
    "algorithm_projection_build": "algorithm.projection_build.us",
    "algorithm_bfs": "algorithm.bfs.p95",
}

#: Metrics whose failure is a broken run, not a measurement: fatal immediately.
STRUCTURAL_KINDS = ("metric_unavailable", "absolute_target")

#: Cap for a metric with a nonzero workload weight when the design sets none.
#: It is a catastrophe bound, not a noise discriminator: the measured noise
#: floor raises it further wherever the environment cannot resolve the metric.
DEFAULT_WEIGHTED_LATENCY_MAX = 2.0

#: Cap for a metric the design scores at weight zero. It exists only to catch a
#: catastrophic regression (a removed fsync, a dropped index); it can never
#: veto on ordinary measurement noise.
DEFAULT_UNWEIGHTED_LATENCY_MAX = 5.0

#: The weighted workload may not get worse overall.
DEFAULT_MINIMUM_OVERALL_RATIO = 1.0


class ComparisonError(ValueError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ComparisonError(f"cannot read {path}: {error}") from error
    if not isinstance(value, dict):
        raise ComparisonError(f"{path} must contain a JSON object")
    return value


def load_report(directory: Path, dataset: str) -> dict[str, dict[str, str]]:
    path = directory / f"{dataset}.tsv"
    try:
        with path.open(encoding="utf-8", newline="") as handle:
            rows = list(csv.DictReader(handle, delimiter="\t"))
    except OSError as error:
        raise ComparisonError(f"cannot read report {path}: {error}") from error
    required = {"dataset", "metric", "status", "p95_us", "value", "detail"}
    if not rows or not required.issubset(rows[0]):
        raise ComparisonError(f"report has an invalid header or no rows: {path}")
    result: dict[str, dict[str, str]] = {}
    for row in rows:
        if row["dataset"] != dataset:
            raise ComparisonError(f"report {path} contains dataset {row['dataset']!r}")
        if row["metric"] in result:
            raise ComparisonError(f"report {path} repeats metric {row['metric']!r}")
        result[row["metric"]] = row
    return result


#: Prefix the runner writes into `detail` for a value it did not measure.
REUSED_PREFIX = "reused:"


def load_run_meta(directory: Path) -> dict[str, Any] | None:
    """The `run.json` build.py writes next to a run's dataset reports."""
    path = directory / "run.json"
    if not path.is_file():
        return None
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def check_run_metadata(
    directories: list[Path],
    side: str,
    *,
    expected_design_sha: str | None,
    expected_crate: str,
    expected_repeat: int,
    expected_datasets: list[str],
    allow_unstamped: bool,
) -> tuple[list[str], list[str], list[str], list[str]]:
    """Validate that report directories are true replicates of one experiment."""
    resolved = [str(directory.resolve()) for directory in directories]
    if len(set(resolved)) != len(resolved):
        raise ComparisonError(
            f"{side} repeats a report directory; replicates must be independent runs"
        )
    fields: dict[str, dict[str, list[str]]] = {
        "revision": {},
        "design_sha256": {},
        "crate": {},
        "repeat": {},
        "datasets": {},
        "dataset_sha256": {},
        "query_templates_sha256": {},
    }
    unstamped: list[str] = []
    for directory in directories:
        meta = load_run_meta(directory)
        required = (
            "revision", "design_sha256", "crate", "repeat", "datasets",
            "dataset_sha256", "query_templates_sha256",
        )
        if meta is None or any(meta.get(field) in (None, "") for field in required):
            unstamped.append(directory.name)
            continue
        normalized = {
            "revision": str(meta["revision"]),
            "design_sha256": str(meta["design_sha256"]),
            "crate": str(meta["crate"]),
            "repeat": str(meta["repeat"]),
            "datasets": json.dumps(sorted(meta["datasets"])),
            "dataset_sha256": json.dumps(meta["dataset_sha256"], sort_keys=True),
        }
        if meta.get("query_templates_sha256"):
            normalized["query_templates_sha256"] = str(meta["query_templates_sha256"])
        for field, value in normalized.items():
            fields[field].setdefault(value, []).append(directory.name)
    if unstamped and not allow_unstamped:
        raise ComparisonError(
            f"{side} directories lack complete run.json metadata "
            f"({', '.join(sorted(unstamped))}); re-measure them, or pass "
            "--allow-unstamped only for an exploratory legacy comparison"
        )
    for field, values in fields.items():
        if len(values) > 1:
            described = "; ".join(
                f"{value[:20]}: {', '.join(sorted(names))}"
                for value, names in sorted(values.items())
            )
            raise ComparisonError(
                f"{side} directories disagree on {field} ({described}); "
                "they are not replicates of one experiment"
            )
    revisions = sorted(fields["revision"])
    design_hashes = sorted(fields["design_sha256"])
    if expected_design_sha and design_hashes and design_hashes != [expected_design_sha]:
        raise ComparisonError(
            f"{side} reports used design {design_hashes[0][:12]}, but the selected "
            f"design is {expected_design_sha[:12]}"
        )
    if fields["crate"] and sorted(fields["crate"]) != [expected_crate]:
        raise ComparisonError(
            f"{side} reports belong to {next(iter(fields['crate']))!r}, "
            f"expected {expected_crate!r}"
        )
    if fields["repeat"] and sorted(fields["repeat"]) != [str(expected_repeat)]:
        raise ComparisonError(
            f"{side} reports used repeat {next(iter(fields['repeat']))}, "
            f"expected {expected_repeat}"
        )
    expected_dataset_key = json.dumps(sorted(expected_datasets))
    if fields["datasets"] and sorted(fields["datasets"]) != [expected_dataset_key]:
        raise ComparisonError(
            f"{side} run.json does not name exactly the selected design datasets"
        )
    if fields["dataset_sha256"]:
        dataset_hashes = json.loads(next(iter(fields["dataset_sha256"])))
        if sorted(dataset_hashes) != sorted(expected_datasets):
            raise ComparisonError(
                f"{side} run.json does not hash exactly the selected design datasets"
            )
    return revisions, sorted(unstamped), design_hashes, sorted(fields["dataset_sha256"])


def _resolved_directories(values: list[str]) -> set[str]:
    return {str(Path(value).resolve()) for value in values}


def check_confirmation(
    prior: dict[str, Any],
    baseline_dirs: list[Path],
    candidate_dirs: list[Path],
    baseline_revisions: list[str],
    candidate_revisions: list[str],
    design_sha: str | None,
    dataset_sha: dict[str, str] | None,
) -> None:
    """Require the confirmation to be an independent repeat of the same code."""
    if prior.get("schema_version") != 2:
        raise ComparisonError("confirmation report has an unsupported schema")
    current = {str(path.resolve()) for path in baseline_dirs + candidate_dirs}
    previous = _resolved_directories(
        list(prior.get("baseline_directories", []))
        + list(prior.get("candidate_directories", []))
    )
    overlap = sorted(current & previous)
    if overlap:
        raise ComparisonError(
            "confirmation reuses measurement directories from the prior comparison "
            f"({', '.join(overlap)}); re-measure both sides into new directories"
        )
    if prior.get("baseline_revisions", []) != baseline_revisions:
        raise ComparisonError("confirmation baseline revision differs from the prior comparison")
    if prior.get("candidate_revisions", []) != candidate_revisions:
        raise ComparisonError("confirmation candidate revision differs from the prior comparison")
    prior_design = prior.get("design_sha256")
    if design_sha and prior_design != design_sha:
        raise ComparisonError("confirmation design differs from the prior comparison")
    if dataset_sha and prior.get("dataset_sha256") != dataset_sha:
        raise ComparisonError("confirmation dataset contents differ from the prior comparison")


def metric_value(report: dict[str, dict[str, str]], metric: str) -> tuple[float | None, str]:
    if metric.endswith(".p95"):
        source, column = metric[:-4], "p95_us"
    elif metric == "storage.reopen.us":
        source, column = "storage.reopen", "value"
    else:
        source, column = metric, "value"
    row = report.get(source)
    if row is None:
        return None, f"missing metric {source}"
    if row.get("status") != "PASS":
        return None, row.get("detail") or f"{source} status is {row.get('status')}"
    try:
        value = float(row[column])
    except (KeyError, TypeError, ValueError):
        return None, f"invalid {column} for {source}"
    if not math.isfinite(value) or value < 0:
        return None, f"non-finite or negative value for {source}"
    return value, ""


def geometric_mean(entries: list[tuple[float, float]]) -> float:
    positive = [(ratio, weight) for ratio, weight in entries if weight > 0]
    total = sum(weight for _, weight in positive)
    if total <= 0:
        raise ComparisonError("score has no positive weights")
    if any(ratio <= 0 for ratio, _ in positive):
        raise ComparisonError("score contains a non-positive ratio")
    return math.exp(sum(weight * math.log(ratio) for ratio, weight in positive) / total)


def topology_robust_gain(
    by_dataset: dict[str, float], dataset_weights: dict[str, float], penalty: float,
) -> tuple[float, float, float]:
    """Return score, weighted log gain, and graph-shape downside term.

    Each dataset's value is the unnormalized workload-weighted log speedup.
    The second term penalizes only whole-scenario regressions, while existing
    per-metric guardrails still protect individual operations.
    """
    weighted_gain = sum(float(dataset_weights[name]) * gain for name, gain in by_dataset.items())
    downside = sum(
        float(dataset_weights[name]) * min(gain, 0.0) ** 2
        for name, gain in by_dataset.items()
    )
    return weighted_gain - penalty * downside, weighted_gain, downside


def algorithm_entries(design: dict[str, Any]) -> list[tuple[str, float]]:
    raw = design["algorithms"]["required"]
    if not raw:
        return []
    names: list[str] = []
    weights: list[float | None] = []
    for item in raw:
        if isinstance(item, str):
            names.append(item)
            weights.append(None)
        else:
            names.append(item["name"])
            weights.append(item.get("performance_weight"))
    if all(weight is None for weight in weights):
        return [(name, 1.0 / len(names)) for name in names]
    if any(weight is None for weight in weights):
        raise ComparisonError("algorithm performance weights must be all present or all absent")
    return [(name, float(weight)) for name, weight in zip(names, weights)]


def metric_slug(name: str) -> str:
    return "_".join(part for part in re.split(r"[^a-z0-9]+", name.lower()) if part)


def reused_metrics(reports: list[dict[str, dict[str, str]]], dataset: str) -> list[str]:
    """Metrics whose value the runner carried over instead of measuring."""
    found: list[str] = []
    for report in reports:
        for name, row in report.items():
            if row.get("detail", "").startswith(REUSED_PREFIX):
                found.append(f"{dataset}/{name}")
    return found


def measure_side(directories: list[Path], dataset: str, metrics: set[str]) -> dict[str, dict[str, Any]]:
    """Per-metric values across one side's report directories."""
    reports = [load_report(directory, dataset) for directory in directories]
    measured: dict[str, dict[str, Any]] = {}
    for metric in metrics:
        values: list[float] = []
        error = ""
        for report in reports:
            value, failure = metric_value(report, metric)
            if failure:
                error = error or failure
                continue
            assert value is not None
            values.append(value)
        measured[metric] = {"values": values, "error": error}
    return measured


def median(values: list[float]) -> float:
    return statistics.median(values)


def spread(values: list[float]) -> float:
    """Same-code spread: how far apart repeated measurements of one metric are."""
    positive = [value for value in values if value > 0]
    if len(positive) < 2:
        return 1.0
    return max(positive) / min(positive)


def guardrail_caps(performance: dict[str, Any]) -> dict[str, float]:
    guardrails = performance["guardrails"]
    weighted = guardrails.get("weighted_latency_regression_max")
    if weighted is None:
        # `other_latency_regression_max` is the pre-noise-floor spelling; it now
        # caps weighted metrics only.
        weighted = guardrails.get("other_latency_regression_max", DEFAULT_WEIGHTED_LATENCY_MAX)
    return {
        "protected_disk": float(guardrails["disk_size_regression_max"]),
        "protected_reopen": float(guardrails["reopen_time_regression_max"]),
        "weighted": float(weighted),
        "unweighted": float(
            guardrails.get("unweighted_latency_regression_max", DEFAULT_UNWEIGHTED_LATENCY_MAX)
        ),
    }


def classify_metric(
    metric: str, operation_weight: float, caps: dict[str, float]
) -> tuple[str, float]:
    """Return (class, cap) for one metric."""
    if metric == "storage.disk_bytes":
        return "protected", caps["protected_disk"]
    if metric == "storage.reopen.us":
        return "protected", caps["protected_reopen"]
    if operation_weight > 0:
        return "weighted", caps["weighted"]
    return "unweighted", caps["unweighted"]


def failure_key(entry: dict[str, Any]) -> tuple[str, str, str]:
    return (entry["dataset"], entry["metric"], entry["kind"])


def compare(
    design: dict[str, Any],
    baseline_dirs: list[Path],
    candidate_dirs: list[Path],
    prior: dict[str, Any] | None = None,
    allow_reused: bool = False,
    *,
    expected_design_sha: str | None = None,
    allow_unstamped: bool = False,
) -> dict[str, Any]:
    performance = design["performance"]
    expected_crate = str(design["candidate"]["crate_name"])
    expected_repeat = int(performance["repeat"])
    expected_datasets = list(performance["datasets"])
    (
        baseline_revisions,
        baseline_unstamped,
        baseline_designs,
        baseline_dataset_hashes,
    ) = check_run_metadata(
        baseline_dirs,
        "baseline",
        expected_design_sha=expected_design_sha,
        expected_crate=expected_crate,
        expected_repeat=expected_repeat,
        expected_datasets=expected_datasets,
        allow_unstamped=allow_unstamped,
    )
    (
        candidate_revisions,
        candidate_unstamped,
        candidate_designs,
        candidate_dataset_hashes,
    ) = check_run_metadata(
        candidate_dirs,
        "candidate",
        expected_design_sha=expected_design_sha,
        expected_crate=expected_crate,
        expected_repeat=expected_repeat,
        expected_datasets=expected_datasets,
        allow_unstamped=allow_unstamped,
    )
    known_designs = sorted(set(baseline_designs + candidate_designs))
    if len(known_designs) > 1:
        raise ComparisonError("baseline and candidate reports used different design vectors")
    known_datasets = sorted(set(baseline_dataset_hashes + candidate_dataset_hashes))
    if len(known_datasets) > 1:
        raise ComparisonError("baseline and candidate reports used different dataset contents")
    template_hashes = {
        str(meta["query_templates_sha256"])
        for directory in baseline_dirs + candidate_dirs
        if (meta := load_run_meta(directory)) is not None
        and meta.get("query_templates_sha256")
    }
    if len(template_hashes) > 1:
        raise ComparisonError(
            "baseline and candidate reports used different performance query templates"
        )
    design_sha = expected_design_sha or (known_designs[0] if known_designs else None)
    dataset_sha = json.loads(known_datasets[0]) if known_datasets else None
    if prior is not None:
        check_confirmation(
            prior,
            baseline_dirs,
            candidate_dirs,
            baseline_revisions,
            candidate_revisions,
            design_sha,
            dataset_sha,
        )
    datasets: list[str] = performance["datasets"]
    dataset_weights: dict[str, float] = performance["dataset_weights"]
    workload: dict[str, float] = performance["workload"]
    primary = set(performance["primary_metrics"])
    caps = guardrail_caps(performance)
    absolute_targets: dict[str, float] = performance["absolute_targets"]

    workload_by_metric = {
        metric: workload[key]
        for key, metric in WORKLOAD_METRICS.items()
        if key in workload
    }
    if "algorithm_kernel" in workload or "algorithm_e2e" in workload:
        for name, algorithm_weight in algorithm_entries(design):
            slug = metric_slug(name)
            workload_by_metric[f"algorithm.{slug}.kernel.p95"] = (
                float(workload.get("algorithm_kernel", 0.0)) * algorithm_weight
            )
            workload_by_metric[f"algorithm.{slug}.e2e.p95"] = (
                float(workload.get("algorithm_e2e", 0.0)) * algorithm_weight
            )
    guardrail_metrics = {"storage.disk_bytes"}
    if "reopen" in performance["cache_modes"]:
        guardrail_metrics.add("storage.reopen.us")
    all_metrics = (
        set(workload_by_metric)
        | guardrail_metrics
        | set(absolute_targets)
        | primary
    )
    ratios: list[tuple[float, float]] = []
    primary_ratios: list[tuple[float, float]] = []
    topology_log_gain = {name: 0.0 for name in datasets}
    rows: list[dict[str, Any]] = []
    fatal: list[dict[str, Any]] = []
    within_noise: list[dict[str, Any]] = []
    noise_limited: list[dict[str, Any]] = []

    reused: list[str] = []
    for dataset in datasets:
        baseline_reports = [load_report(directory, dataset) for directory in baseline_dirs]
        candidate_reports = [load_report(directory, dataset) for directory in candidate_dirs]
        reused.extend(reused_metrics(baseline_reports, dataset))
        reused.extend(reused_metrics(candidate_reports, dataset))
        baseline = measure_side(baseline_dirs, dataset, all_metrics)
        candidate = measure_side(candidate_dirs, dataset, all_metrics)
        dataset_weight = float(dataset_weights[dataset])
        for metric in sorted(all_metrics):
            operation_weight = float(workload_by_metric.get(metric, 0.0))
            combined_weight = dataset_weight * operation_weight
            metric_class, cap = classify_metric(metric, operation_weight, caps)
            base_side, candidate_side = baseline[metric], candidate[metric]
            base_values, candidate_values = base_side["values"], candidate_side["values"]
            row: dict[str, Any] = {
                "dataset": dataset,
                "dataset_weight": dataset_weight,
                "metric": metric,
                "class": metric_class,
                "operation_weight": operation_weight,
                "baseline": median(base_values) if base_values else None,
                "candidate": median(candidate_values) if candidate_values else None,
                "baseline_values": base_values,
                "candidate_values": candidate_values,
                "baseline_noise_floor": spread(base_values),
                "candidate_noise_floor": spread(candidate_values),
                "noise_floor": max(spread(base_values), spread(candidate_values)),
                "cap": cap,
                "ratio": None,
                "regression": None,
                "within_noise_floor": False,
            }
            if base_side["error"] or candidate_side["error"]:
                parts = []
                if base_side["error"]:
                    parts.append(f"baseline: {base_side['error']}")
                if candidate_side["error"]:
                    parts.append(f"candidate: {candidate_side['error']}")
                row["error"] = "; ".join(parts)
                if combined_weight > 0 or metric in primary or metric in absolute_targets or metric in guardrail_metrics:
                    fatal.append({
                        "dataset": dataset,
                        "metric": metric,
                        "kind": "metric_unavailable",
                        "observed": row["error"],
                        "limit": "",
                    })
                rows.append(row)
                continue
            assert row["baseline"] is not None and row["candidate"] is not None
            base_value, candidate_value = row["baseline"], row["candidate"]
            ratio = 1.0 if base_value == candidate_value == 0 else (
                max(base_value, 1e-9) / max(candidate_value, 1e-9)
            )
            regression = 1.0 if base_value == candidate_value == 0 else (
                max(candidate_value, 1e-9) / max(base_value, 1e-9)
            )
            tolerance = max(cap, row["noise_floor"])
            row["ratio"] = ratio
            row["regression"] = regression
            row["tolerance"] = tolerance
            rows.append(row)
            if combined_weight > 0:
                ratios.append((ratio, combined_weight))
                topology_log_gain[dataset] += operation_weight * math.log(ratio)
            if metric in primary and dataset_weight > 0:
                primary_ratios.append((ratio, dataset_weight / len(primary)))

            if row["noise_floor"] > cap:
                noise_limited.append({
                    "dataset": dataset,
                    "metric": metric,
                    "kind": "noise_limited",
                    "noise_floor": row["noise_floor"],
                    "cap": cap,
                    "observed": row["noise_floor"],
                    "limit": cap,
                })
            if regression > tolerance:
                fatal.append({
                    "dataset": dataset,
                    "metric": metric,
                    "kind": "latency_regression",
                    "observed": regression,
                    "limit": tolerance,
                })
            elif regression > cap:
                row["within_noise_floor"] = True
                within_noise.append({
                    "dataset": dataset,
                    "metric": metric,
                    "kind": "within_noise_floor",
                    "observed": regression,
                    "cap": cap,
                    "limit": cap,
                    "noise_floor": row["noise_floor"],
                })
            target = absolute_targets.get(metric)
            if target is not None and candidate_value > float(target):
                fatal.append({
                    "dataset": dataset,
                    "metric": metric,
                    "kind": "absolute_target",
                    "observed": candidate_value,
                    "limit": float(target),
                })

    overall_ratio = geometric_mean(ratios)
    primary_ratio = geometric_mean(primary_ratios)
    downside_penalty = float(performance.get("topology_downside_penalty", 0.0))
    robust_gain, weighted_log_gain, topology_downside = topology_robust_gain(
        topology_log_gain, dataset_weights, downside_penalty,
    )
    minimum_robust_gain = float(
        performance["guardrails"].get("minimum_topology_robust_gain", 0.0)
    )
    if (downside_penalty > 0 or "minimum_topology_robust_gain" in performance["guardrails"]) \
            and robust_gain < minimum_robust_gain:
        fatal.append({
            "dataset": "*", "metric": "topology_robust_gain",
            "kind": "topology_robust_gain_shortfall",
            "observed": robust_gain, "limit": minimum_robust_gain,
        })
    required = float(performance["guardrails"]["minimum_primary_improvement"])
    primary_improvement = primary_ratio - 1.0
    if primary_improvement < required:
        fatal.append({
            "dataset": "*",
            "metric": "primary_improvement",
            "kind": "primary_improvement_shortfall",
            "observed": primary_improvement,
            "limit": required,
        })
    minimum_overall = float(
        performance["guardrails"].get("minimum_overall_ratio", DEFAULT_MINIMUM_OVERALL_RATIO)
    )
    if overall_ratio < minimum_overall:
        fatal.append({
            "dataset": "*",
            "metric": "overall_ratio",
            "kind": "overall_regression",
            "observed": overall_ratio,
            "limit": minimum_overall,
        })

    if reused and not allow_reused:
        raise ComparisonError(
            f"{len(reused)} metric value(s) were reused rather than measured "
            f"({', '.join(sorted(set(reused))[:4])}); an acceptance decision needs "
            "each side measured without --reuse-data, or pass --allow-reused for "
            "an exploratory comparison"
        )

    confirmed: list[dict[str, Any]] = []
    tentative: list[dict[str, Any]] = []
    if prior is None:
        structural = [entry for entry in fatal if entry["kind"] in STRUCTURAL_KINDS]
        confirmed.extend(structural)
        tentative.extend(entry for entry in fatal if entry["kind"] not in STRUCTURAL_KINDS)
    else:
        # A previous measurement may itself have been tentative: what matters is
        # whether it observed the same failure, not whether it acted on it.
        seen = {
            failure_key(entry)
            for entry in prior.get("failure_details", []) + prior.get("tentative_failure_details", [])
        }
        for entry in fatal:
            if entry["kind"] in STRUCTURAL_KINDS or failure_key(entry) in seen:
                confirmed.append(entry)
            else:
                tentative.append(entry)

    status = (
        "rejected" if confirmed else
        "provisional" if tentative else
        "inconclusive" if noise_limited else
        "accepted"
    )
    return {
        "schema_version": 2,
        "status": status,
        "accepted": status == "accepted",
        "overall_ratio": overall_ratio,
        "topology_log_gain": topology_log_gain,
        "weighted_log_gain": weighted_log_gain,
        "topology_downside": topology_downside,
        "topology_downside_penalty": downside_penalty,
        "topology_robust_gain": robust_gain,
        "minimum_topology_robust_gain": minimum_robust_gain,
        "primary_ratio": primary_ratio,
        "primary_improvement": primary_improvement,
        "minimum_overall_ratio": minimum_overall,
        "dataset_weights": dataset_weights,
        "baseline_directories": [str(path) for path in baseline_dirs],
        "candidate_directories": [str(path) for path in candidate_dirs],
        "baseline_revisions": baseline_revisions,
        "candidate_revisions": candidate_revisions,
        "design_sha256": design_sha,
        "dataset_sha256": dataset_sha,
        "unstamped_directories": sorted(baseline_unstamped + candidate_unstamped),
        "reused_metrics": sorted(set(reused)),
        "guardrail_caps": caps,
        "rows": rows,
        "failures": [describe(entry) for entry in confirmed],
        "failure_details": confirmed,
        "tentative_failures": [describe(entry) for entry in tentative],
        "tentative_failure_details": tentative,
        "within_noise_floor": [describe(entry) for entry in within_noise],
        "within_noise_floor_details": within_noise,
        "noise_limited": [describe(entry) for entry in noise_limited],
        "noise_limited_details": noise_limited,
    }


def describe(entry: dict[str, Any]) -> str:
    kind = entry.get("kind", "")
    if kind == "noise_limited":
        return (
            f"{entry['dataset']}/{entry['metric']}: same-code spread "
            f"{entry['noise_floor']:.2f}x is above the configured cap {entry['cap']:.2f}x"
        )
    if kind == "within_noise_floor":
        return (
            f"{entry['dataset']}/{entry['metric']}: {entry['observed']:.4f} is above the cap "
            f"{entry['cap']:.2f}x but inside the measured noise floor {entry['noise_floor']:.2f}x"
        )
    if kind == "metric_unavailable":
        return f"{entry['dataset']}/{entry['metric']}: {entry['observed']}"
    if kind == "latency_regression":
        return (
            f"{entry['dataset']}/{entry['metric']}: regression {entry['observed']:.4f} "
            f"exceeds tolerance {entry['limit']:.4f}"
        )
    if kind == "absolute_target":
        return (
            f"{entry['dataset']}/{entry['metric']}: {entry['observed']:.4f} "
            f"exceeds target {entry['limit']:.4f}"
        )
    if kind == "primary_improvement_shortfall":
        return (
            f"weighted primary improvement {entry['observed']:.4%} "
            f"is below {entry['limit']:.4%}"
        )
    if kind == "overall_regression":
        return (
            f"weighted overall ratio {entry['observed']:.4f} "
            f"is below {entry['limit']:.4f}"
        )
    if kind == "topology_robust_gain_shortfall":
        return (
            f"topology-robust log gain {entry['observed']:.4f} "
            f"is below {entry['limit']:.4f}"
        )
    return f"{entry['dataset']}/{entry['metric']}: {kind}"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--design", type=Path, required=True)
    parser.add_argument(
        "--baseline", type=Path, action="append", required=True,
        help="Baseline report directory; repeat to add replicates of the same code",
    )
    parser.add_argument(
        "--candidate", type=Path, action="append", required=True,
        help="Candidate report directory; repeat to add replicates of the same code",
    )
    parser.add_argument(
        "--confirm-with", type=Path,
        help="A previous comparison JSON from an independent measurement; only "
             "failures reproduced there become fatal",
    )
    parser.add_argument(
        "--allow-reused", action="store_true",
        help="Allow directories measured with --reuse-data; an acceptance "
             "decision needs every side measured for this revision",
    )
    parser.add_argument(
        "--allow-unstamped", action="store_true",
        help="Allow legacy directories without complete run.json metadata; "
             "use only for exploratory comparisons",
    )
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        prior = load_json(args.confirm_with) if args.confirm_with else None
        result = compare(
            load_json(args.design),
            [path.resolve() for path in args.baseline],
            [path.resolve() for path in args.candidate],
            prior,
            args.allow_reused,
            expected_design_sha=hashlib.sha256(args.design.read_bytes()).hexdigest(),
            allow_unstamped=args.allow_unstamped,
        )
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    except (ComparisonError, KeyError, TypeError, ValueError) as error:
        print(f"Error: {error}", file=sys.stderr)
        return 2
    print(f"Weighted overall ratio: {result['overall_ratio']:.4f}x")
    print(f"Topology-robust log gain: {result['topology_robust_gain']:.4f}")
    print(f"Weighted primary improvement: {result['primary_improvement']:.2%}")
    for side in ("baseline_revisions", "candidate_revisions"):
        if result[side]:
            print(f"{side.replace('_', ' ')}: {', '.join(r[:12] for r in result[side])}")
    if result["unstamped_directories"]:
        print(
            "  ! directories without run.json (revision unknown): "
            + ", ".join(result["unstamped_directories"])
        )
    if result["reused_metrics"]:
        print(
            "  ~ reused (not measured in this run): "
            + ", ".join(result["reused_metrics"][:6])
            + (" ..." if len(result["reused_metrics"]) > 6 else "")
        )
    if result["noise_limited_details"]:
        print(
            f"Noise-limited metrics (same-code spread above the configured cap, "
            f"guardrail not evaluable): {len(result['noise_limited_details'])}"
        )
    for note in result["within_noise_floor"]:
        print(f"  ~ within noise floor: {note}")
    print(f"Decision: {result['status'].upper()}")
    for failure in result["failures"]:
        print(f"  - {failure}")
    for failure in result["tentative_failures"]:
        print(f"  ? unconfirmed: {failure}")
    if result["status"] == "provisional":
        print(
            "  Re-measure both sides and re-run with --confirm-with "
            f"{args.output} before treating these as regressions."
        )
    if result["status"] == "inconclusive":
        print(
            "  At least one required metric varies more than its guardrail cap; "
            "this environment cannot support an acceptance decision."
        )
    print(f"Report: {args.output}")
    if result["status"] == "accepted":
        return 0
    if result["status"] == "rejected":
        return 1
    return 3 if result["status"] == "provisional" else 4


if __name__ == "__main__":
    raise SystemExit(main())
