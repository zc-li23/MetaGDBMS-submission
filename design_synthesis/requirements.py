"""Requirement IR and deterministic normalization of structured user needs."""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
import json
import re
from pathlib import Path
from typing import Any


LEVELS = {"none": 0.0, "low": 0.35, "medium": 0.65, "high": 1.0, "critical": 1.4}
OBJECTIVES = {
    "point_read", "traversal", "scan", "write", "analytics", "recovery",
    "schema_rigidity", "write_flexibility", "read_concurrency", "high_contention",
    "low_contention", "implementation_simplicity", "disk_efficiency",
}


class RequirementError(ValueError):
    """A malformed or contradictory requirement document."""


@dataclass(frozen=True)
class Fact:
    field: str
    value: Any
    strength: str = "soft"
    weight: float = 1.0
    confidence: float = 1.0
    evidence: str = ""
    source: str = "user"

    def __post_init__(self) -> None:
        if self.strength not in {"hard", "soft"}:
            raise RequirementError(f"invalid fact strength: {self.strength}")
        if not 0 <= self.confidence <= 1:
            raise RequirementError("fact confidence must be between 0 and 1")
        if self.weight < 0:
            raise RequirementError("fact weight must be non-negative")


@dataclass
class RequirementIR:
    candidate: dict[str, str]
    language: dict[str, str]
    facts: list[Fact] = field(default_factory=list)
    graph: dict[str, Any] = field(default_factory=dict)
    schema: dict[str, Any] = field(default_factory=dict)
    algorithms: list[dict[str, Any]] = field(default_factory=list)
    capabilities: dict[str, list[str]] = field(default_factory=dict)
    custom_components: list[dict[str, Any]] = field(default_factory=list)
    unknowns: list[str] = field(default_factory=list)
    source: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        value = asdict(self)
        value["schema_version"] = 1
        return value

    def objective_weights(self) -> dict[str, float]:
        result = {name: 0.0 for name in OBJECTIVES}
        for fact in self.facts:
            if not fact.field.startswith("objective."):
                continue
            name = fact.field.split(".", 1)[1]
            if name not in result:
                continue
            raw = fact.value
            if isinstance(raw, str):
                if raw not in LEVELS:
                    raise RequirementError(f"unknown objective priority {raw!r} for {name}")
                value = LEVELS[raw]
            elif isinstance(raw, (int, float)) and not isinstance(raw, bool) and raw >= 0:
                value = float(raw)
            else:
                raise RequirementError(f"objective {name} must be a non-negative number or priority")
            result[name] = max(result[name], value * fact.weight * fact.confidence)
        topology = next((fact.value for fact in self.facts if fact.field == "graph.topology"), self.graph.get("topology"))
        names = set(topology) if isinstance(topology, dict) else {topology}
        if "power_law_social" in names:
            result["traversal"] += 0.45
            result["read_concurrency"] += 0.20
        if "sparse_railway" in names:
            result["traversal"] += 0.30
            result["point_read"] += 0.20
        if "uniform_regular" in names:
            result["scan"] += 0.25
        if not any(result.values()):
            result.update({"point_read": 0.7, "traversal": 0.8, "write": 0.5, "recovery": 0.8})
        return result


def _priority(value: Any, field_name: str) -> float | str:
    if isinstance(value, str):
        normalized = value.strip().lower()
        if normalized not in LEVELS:
            raise RequirementError(f"{field_name} must be one of {', '.join(LEVELS)} or a number")
        return normalized
    if isinstance(value, (int, float)) and not isinstance(value, bool) and value >= 0:
        return float(value)
    raise RequirementError(f"{field_name} must be a non-negative number or priority")


def _algorithm(value: Any, index: int) -> dict[str, Any]:
    if isinstance(value, str) and value.strip():
        return {"name": value.strip(), "description": f"Implement the user-requested {value.strip()} algorithm"}
    if not isinstance(value, dict):
        raise RequirementError(f"algorithms[{index}] must be a string or object")
    allowed = {"name", "description", "performance_weight"}
    if set(value) - allowed or not {"name", "description"}.issubset(value):
        raise RequirementError(f"algorithms[{index}] must contain name, description, and optional performance_weight")
    if not all(isinstance(value[key], str) and value[key].strip() for key in ("name", "description")):
        raise RequirementError(f"algorithms[{index}] name and description must be non-empty strings")
    return dict(value)


def normalize_requirements(raw: dict[str, Any], *, source_path: str = "") -> RequirementIR:
    """Normalize the friendly requirement document into an evidence-bearing IR."""
    if raw.get("schema_version", 1) != 1:
        raise RequirementError("requirements schema_version must be 1")
    candidate = raw.get("candidate")
    if not isinstance(candidate, dict) or not isinstance(candidate.get("crate_name"), str):
        raise RequirementError("candidate.crate_name is required")
    crate = candidate["crate_name"].strip()
    if not re.fullmatch(r"metagdb_[a-z0-9_]+", crate):
        raise RequirementError("candidate.crate_name must be lowercase and begin with metagdb_")
    description = candidate.get("description", "").strip()

    language_raw = raw.get("language")
    if isinstance(language_raw, str):
        language_name = language_raw.strip()
        grammar_path = f"data/language/{language_name}.g4"
    elif isinstance(language_raw, dict):
        language_name = str(language_raw.get("name", "")).strip()
        grammar_path = str(language_raw.get("grammar_path", f"data/language/{language_name}.g4")).strip()
    else:
        raise RequirementError("language must name Cypher, GQL, or Gremlin")
    if language_name not in {"Cypher", "GQL", "Gremlin"}:
        raise RequirementError("language must name Cypher, GQL, or Gremlin")

    facts: list[Fact] = []
    schema = raw.get("schema", {})
    if not isinstance(schema, dict):
        raise RequirementError("schema must be an object")
    if schema.get("mode"):
        facts.append(Fact("schema.mode", schema["mode"], "hard", evidence="explicit schema mode"))

    graph = raw.get("graph", {})
    if not isinstance(graph, dict):
        raise RequirementError("graph must be an object")
    topology = graph.get("topology")
    if topology:
        facts.append(Fact("graph.topology", topology, "soft", evidence="declared graph topology"))

    workload = raw.get("workload", {})
    if not isinstance(workload, dict):
        raise RequirementError("workload must be an object")
    for name, value in workload.items():
        if name not in OBJECTIVES:
            raise RequirementError(f"unknown workload objective: {name}")
        facts.append(Fact(f"objective.{name}", _priority(value, f"workload.{name}"), "soft", evidence="declared workload"))

    correctness = raw.get("correctness", {})
    if not isinstance(correctness, dict):
        raise RequirementError("correctness must be an object")
    isolation = correctness.get("isolation_level")
    if isolation:
        facts.append(Fact("correctness.isolation_level", isolation, "hard", evidence="explicit isolation requirement"))
    if correctness.get("crash_safe", True) is not True:
        raise RequirementError("MetaGDB research prototypes require crash-safe durability")

    preferences = raw.get("preferences", {})
    if not isinstance(preferences, dict):
        raise RequirementError("preferences must be an object")
    for name, value in preferences.items():
        strength = "hard" if name.endswith("_required") else "soft"
        field_name = name.removesuffix("_required")
        facts.append(Fact(f"preference.{field_name}", value, strength, evidence="explicit design preference"))

    storage = raw.get("storage", {})
    if not isinstance(storage, dict):
        raise RequirementError("storage must be an object")
    for capability in storage.get("required_capabilities", []):
        facts.append(Fact("storage.required_capability", capability, "hard", evidence="explicit storage capability"))
    for component in storage.get("preferred_components", []):
        facts.append(Fact("preference.component", component, "soft", weight=1.5, evidence="preferred storage component"))
    for component in storage.get("required_components", []):
        facts.append(Fact("component.required", component, "hard", evidence="explicit required component"))

    for item in raw.get("facts", []):
        if not isinstance(item, dict) or "field" not in item or "value" not in item:
            raise RequirementError("each facts entry requires field and value")
        facts.append(Fact(**item))

    algorithms = [_algorithm(value, index) for index, value in enumerate(raw.get("algorithms", []))]
    explicit_weights = [item.get("performance_weight") for item in algorithms]
    if algorithms and any(weight is not None for weight in explicit_weights):
        if any(weight is None for weight in explicit_weights):
            raise RequirementError("algorithm performance weights must be provided for all algorithms or none")
        if abs(sum(float(weight) for weight in explicit_weights) - 1.0) > 1e-6:
            raise RequirementError("algorithm performance weights must sum to 1.0")

    capabilities = raw.get("capabilities", {})
    if not isinstance(capabilities, dict):
        raise RequirementError("capabilities must be an object")
    normalized_caps = {
        "required": list(capabilities.get("required", [])),
        "optional": list(capabilities.get("optional", [])),
    }
    custom = storage.get("custom_components", raw.get("custom_components", []))
    if not isinstance(custom, list):
        raise RequirementError("custom_components must be an array")

    source = dict(raw.get("source", {}))
    source.setdefault("kind", "structured")
    if source_path:
        source["path"] = source_path

    return RequirementIR(
        candidate={"crate_name": crate, "description": description},
        language={"name": language_name, "grammar_path": grammar_path},
        facts=facts,
        graph=graph,
        schema=schema,
        algorithms=algorithms,
        capabilities=normalized_caps,
        custom_components=custom,
        unknowns=list(raw.get("unknowns", [])),
        source=source,
    )


def load_requirements(path: Path) -> RequirementIR:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise RequirementError(f"requirements file does not exist: {path}") from exc
    except json.JSONDecodeError as exc:
        raise RequirementError(f"invalid requirements JSON at {exc.lineno}:{exc.colno}: {exc.msg}") from exc
    if not isinstance(raw, dict):
        raise RequirementError("requirements root must be an object")
    return normalize_requirements(raw, source_path=str(path))
