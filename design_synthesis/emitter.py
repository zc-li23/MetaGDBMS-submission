"""Compile one selected component composition into the frozen backend design vector."""

from __future__ import annotations

import re
from typing import Any

from .catalog import Catalog, Component
from .requirements import RequirementIR
from .solver import Candidate


WORKLOAD_BASE = {
    "storage_import": 0.04,
    "storage_vertex_point_read": 0.08,
    "storage_edge_point_read": 0.04,
    "storage_out_adjacency": 0.12,
    "storage_hotspot_out_adjacency": 0.06,
    "storage_scan": 0.08,
    "storage_reopen": 0.03,
    "storage_disk_size": 0.03,
    "query_point_read": 0.10,
    "query_one_hop": 0.10,
    "transaction_commit": 0.06,
    "algorithm_projection_build": 0.06,
    "algorithm_kernel": 0.10,
    "algorithm_e2e": 0.10,
}
OBJECTIVE_METRICS = {
    "point_read": ("storage_vertex_point_read", "storage_edge_point_read", "query_point_read"),
    "traversal": ("storage_out_adjacency", "storage_hotspot_out_adjacency", "query_one_hop"),
    "scan": ("storage_scan",),
    "write": ("storage_import", "transaction_commit"),
    "analytics": ("algorithm_projection_build", "algorithm_kernel", "algorithm_e2e"),
    "recovery": ("storage_reopen",),
    "disk_efficiency": ("storage_disk_size",),
}
METRIC_NAMES = {
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
    "algorithm_projection_build": "algorithm.projection_build.us",
}


def _normalize(weights: dict[str, float]) -> dict[str, float]:
    total = sum(weights.values())
    if total <= 0:
        raise ValueError("weight map has no positive values")
    values = {key: value / total for key, value in weights.items()}
    # Stable residual correction prevents floating sum drift in serialized JSON.
    largest = max(values, key=values.get)
    values[largest] += 1.0 - sum(values.values())
    return {key: round(value, 12) for key, value in values.items()}


def _workload(ir: RequirementIR) -> dict[str, float]:
    weights = dict(WORKLOAD_BASE)
    objectives = ir.objective_weights()
    for objective, metrics in OBJECTIVE_METRICS.items():
        priority = objectives.get(objective, 0.0)
        multiplier = 0.4 + 2.2 * priority
        for metric in metrics:
            weights[metric] *= multiplier
    if not ir.algorithms:
        for metric in ("algorithm_projection_build", "algorithm_kernel", "algorithm_e2e"):
            weights[metric] = 0.0
    return _normalize(weights)


def _topology_weights(ir: RequirementIR) -> dict[str, float]:
    topology = next((fact.value for fact in ir.facts if fact.field == "graph.topology"), ir.graph.get("topology"))
    if isinstance(topology, dict):
        mapping = {"uniform_regular": "uniform_standard", "sparse_railway": "rn", "power_law_social": "yt"}
        weights = {"uniform_smoke": 0.05}
        for name, value in topology.items():
            dataset = mapping.get(name, name)
            weights[dataset] = weights.get(dataset, 0.0) + float(value)
        return _normalize(weights)
    if topology == "power_law_social":
        return {"uniform_smoke": 0.05, "uniform_standard": 0.20, "rn": 0.10, "yt": 0.65}
    if topology == "sparse_railway":
        return {"uniform_smoke": 0.05, "uniform_standard": 0.20, "rn": 0.65, "yt": 0.10}
    if topology == "uniform_regular":
        return {"uniform_smoke": 0.10, "uniform_standard": 0.65, "rn": 0.15, "yt": 0.10}
    return {"uniform_smoke": 0.05, "uniform_standard": 0.25, "rn": 0.30, "yt": 0.40}


def _profile(objectives: dict[str, float]) -> str:
    grouped = {
        "point_read_optimized": objectives["point_read"],
        "traversal_optimized": objectives["traversal"],
        "scan_analytics": objectives["scan"] + objectives["analytics"],
        "write_optimized": objectives["write"],
        "recovery_optimized": objectives["recovery"],
    }
    ordered = sorted(grouped.items(), key=lambda item: (-item[1], item[0]))
    if len(ordered) > 1 and ordered[0][1] - ordered[1][1] < 0.15:
        return "balanced"
    return ordered[0][0]


def _primary_metrics(workload: dict[str, float], algorithms: list[dict[str, Any]]) -> list[str]:
    expanded = dict(METRIC_NAMES)
    if algorithms:
        # The performance harness expands these generic workload categories per required algorithm.
        for algorithm in algorithms:
            slug = re.sub(r"[^a-z0-9]+", "_", algorithm["name"].lower()).strip("_")
            expanded[f"algorithm_kernel:{slug}"] = f"algorithm.{slug}.kernel.p95"
            expanded[f"algorithm_e2e:{slug}"] = f"algorithm.{slug}.e2e.p95"
    choices: list[tuple[float, str]] = []
    for key, value in workload.items():
        if key in {"algorithm_kernel", "algorithm_e2e"} and algorithms:
            for algorithm in algorithms:
                slug = re.sub(r"[^a-z0-9]+", "_", algorithm["name"].lower()).strip("_")
                choices.append((value * float(algorithm.get("performance_weight", 1 / len(algorithms))), expanded[f"{key}:{slug}"]))
        elif key in expanded:
            choices.append((value, expanded[key]))
    return [name for _, name in sorted(choices, key=lambda item: (-item[0], item[1]))[:2]]


def _algorithm_specs(ir: RequirementIR) -> list[dict[str, Any]]:
    algorithms = [dict(item) for item in ir.algorithms]
    if algorithms and not any("performance_weight" in item for item in algorithms):
        weight = 1.0 / len(algorithms)
        for item in algorithms:
            item["performance_weight"] = weight
    return algorithms


def compile_design(ir: RequirementIR, candidate: Candidate, catalog: Catalog) -> dict[str, Any]:
    components = [catalog.by_id[component_id] for component_id in candidate.components]
    by_dimension = {component.dimension: component for component in components if component.dimension != "augmentation"}
    augmentations = [component for component in components if component.dimension == "augmentation"]
    schema_mode = by_dimension["schema"].emits["mode"]
    if schema_mode == "strict" and (not ir.schema.get("vertex_labels") or not ir.schema.get("edge_types")):
        raise ValueError("strict schema requires explicit vertex_labels and edge_types; use guided input or structured requirements")
    cc = by_dimension["concurrency_control"].emits["concurrency_control"]
    isolation = by_dimension["isolation"].emits["isolation_level"]
    execution = by_dimension["execution"].emits["model"]

    layout_parts = [
        by_dimension["storage_layout"].emits["layout"],
        by_dimension["storage_update"].emits["update"],
    ]
    layout_parts.extend(
        component.description for component in augmentations
        if component.id.startswith(("support.", "projection.", "paper."))
    )
    for dimension in ("storage_layout", "storage_update"):
        component = by_dimension[dimension]
        if component.id.startswith("paper.") and component.description not in layout_parts:
            layout_parts.append(component.description)
    indexes: list[str] = []
    for component in augmentations:
        if "index" in component.emits:
            indexes.append(str(component.emits["index"]))
        indexes.extend(map(str, component.emits.get("indexes", [])))
    indexes = list(dict.fromkeys(indexes))

    workload = _workload(ir)
    dataset_weights = _topology_weights(ir)
    algorithms = _algorithm_specs(ir)
    objectives = ir.objective_weights()
    description = ir.candidate["description"] or (
        f"A {schema_mode}, crash-safe disk graph database for {ir.language['name']} "
        f"selected by constraint-guided component synthesis"
    )
    required_caps = list(ir.capabilities.get("required", []))
    required_caps.extend(
        f"implement {component.id}: {component.description}"
        for component in components if component.id.startswith("paper.")
    )
    required_caps.extend([
        f"execute the fixed {ir.language['name']} Core Language Profile through the generated ANTLR frontend",
        "create, read, update, and delete graph data with stable typed results",
        "preserve graph consistency and transaction atomicity across abort, commit, reopen, and abrupt recovery",
    ])
    for algorithm in algorithms:
        required_caps.append(f"execute user-requested {algorithm['name']} through both the core and public language paths")
    required_caps = list(dict.fromkeys(required_caps))

    return {
        "schema_version": 1,
        "candidate": {"crate_name": ir.candidate["crate_name"], "description": description},
        "language": dict(ir.language),
        "schema": {
            "mode": schema_mode,
            "vertex_labels": list(ir.schema.get("vertex_labels", [])),
            "edge_types": list(ir.schema.get("edge_types", [])),
            "property_keys": list(ir.schema.get("property_keys", [])),
        },
        "storage": {
            "medium": "local disk",
            "layout": "; ".join(layout_parts),
            "indexes": indexes,
            "persistence": "all committed graph data survives clean process restart",
            "recovery": by_dimension["recovery"].emits["recovery"],
        },
        "transactions": {
            "atomicity_scope": "multi_operation_transaction",
            "concurrency_control": cc,
            "isolation_level": isolation,
            "durability": "crash_safe",
        },
        "execution": {"model": execution},
        "algorithms": {"required": algorithms},
        "capabilities": {
            "required": required_caps,
            "optional": list(ir.capabilities.get("optional", [])),
            "unsupported_behavior": "return a precise unsupported error; never silently return an empty or fabricated result",
        },
        "quality": {
            "benchmark_min_pass_rate": 0.9,
            "system_benchmark_required": True,
            "required_tests": [
                "generated ANTLR parser and CST lowering tests",
                "planner, optimizer, and executor unit tests",
                "storage restart and graph consistency tests",
                "transaction visibility, rollback, isolation, and write-conflict tests",
                "fixed ACID system benchmark",
                "abrupt-process crash recovery tests",
                "public-runtime semantic integration tests",
                "self-checking end-to-end demo",
            ],
            "zero_compiler_warnings": True,
        },
        "performance": {
            "profile": _profile(objectives),
            "threads": 1,
            "datasets": list(dataset_weights),
            "dataset_weights": dataset_weights,
            "cache_modes": ["warm", "reopen"],
            "repeat": 5,
            "max_optimization_iterations": 12,
            "no_improvement_patience": 3,
            "topology_downside_penalty": 1.0,
            "workload": workload,
            "primary_metrics": _primary_metrics(workload, algorithms),
            "guardrails": {
                "other_latency_regression_max": 1.2,
                "weighted_latency_regression_max": 2.0,
                "unweighted_latency_regression_max": 5.0,
                "disk_size_regression_max": 1.3,
                "reopen_time_regression_max": 1.3,
                "minimum_primary_improvement": 0.03,
                "minimum_overall_ratio": 1.0,
                "minimum_topology_robust_gain": 0.0,
            },
            "absolute_targets": {},
        },
        "boundaries": {
            "allow_framework_changes": False,
            "allow_benchmark_changes": False,
            "allow_grammar_changes": False,
        },
        "budget": {"max_fix_iterations": 0},
    }
