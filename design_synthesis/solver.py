"""Constraint propagation, component composition, Pareto filtering, and clarification."""

from __future__ import annotations

from dataclasses import asdict, dataclass
from collections import Counter
from itertools import product
import math
from typing import Any

from .catalog import Catalog, Component, EXCLUSIVE_DIMENSIONS
from .requirements import RequirementIR


class UnsatisfiableRequirements(ValueError):
    def __init__(self, reasons: list[str]):
        super().__init__("; ".join(reasons))
        self.reasons = reasons


@dataclass
class Candidate:
    components: tuple[str, ...]
    score: float
    utility: float
    complexity: float
    risk: float
    assumptions: tuple[str, ...]
    trace: tuple[str, ...]

    def as_dict(self) -> dict[str, Any]:
        return asdict(self)


def _hard_selections(ir: RequirementIR, catalog: Catalog) -> tuple[dict[str, str], list[str]]:
    forced: dict[str, str] = {}
    reasons: list[str] = []
    for fact in ir.facts:
        if fact.field != "component.required" or fact.strength != "hard":
            continue
        component = catalog.by_id.get(str(fact.value))
        if component is None:
            reasons.append(f"required component {fact.value!r} is not in the catalog")
        elif component.dimension != "augmentation":
            previous = forced.get(component.dimension)
            if previous and previous != component.id:
                reasons.append(f"conflicting required components select both {previous} and {component.id}")
            forced[component.dimension] = component.id
    field_to_dimension = {
        "schema.mode": ("schema", "mode"),
        "correctness.isolation_level": ("isolation", "isolation_level"),
        "preference.execution_model": ("execution", "model"),
        "preference.execution_model_required": ("execution", "model"),
        "preference.concurrency_control": ("concurrency_control", "concurrency_control"),
        "preference.storage_layout": ("storage_layout", None),
        "preference.storage_update": ("storage_update", None),
        "preference.recovery": ("recovery", None),
    }
    for fact in ir.facts:
        if fact.strength != "hard" or fact.field not in field_to_dimension:
            continue
        dimension, emitted = field_to_dimension[fact.field]
        matches = []
        for option in catalog.by_dimension[dimension]:
            if emitted is None:
                if fact.value in {option.id, option.id.split(".", 1)[-1]}:
                    matches.append(option.id)
            elif option.emits.get(emitted) == fact.value:
                matches.append(option.id)
        if len(matches) != 1:
            reasons.append(f"hard requirement {fact.field}={fact.value!r} has {len(matches)} catalog matches")
            continue
        previous = forced.get(dimension)
        if previous and previous != matches[0]:
            reasons.append(f"conflicting hard requirements select both {previous} and {matches[0]}")
        forced[dimension] = matches[0]
    return forced, reasons


def _required_capabilities(ir: RequirementIR, objectives: dict[str, float]) -> set[str]:
    required = {"disk_records", "crash_safe", "atomic_commit"}
    for fact in ir.facts:
        if fact.field == "storage.required_capability":
            required.add(str(fact.value))
    if objectives["point_read"] > 0:
        required.update({"vertex_point_lookup", "edge_point_lookup"})
    if objectives["traversal"] > 0:
        required.update({"outgoing_adjacency", "incoming_adjacency"})
    if objectives["analytics"] > 0 or ir.algorithms:
        required.add("analytics_projection")
    return required


def _compatible(selected: dict[str, Component]) -> tuple[bool, str]:
    ids = {component.id for component in selected.values()}
    for component in selected.values():
        conflicts = component.conflicts & ids
        if conflicts:
            return False, f"{component.id} conflicts with {sorted(conflicts)[0]}"
        if not component.requires_components.issubset(ids):
            missing = sorted(component.requires_components - ids)
            return False, f"{component.id} requires component {missing[0]}"
        for dimension, allowed in component.compatible_with.items():
            actual = selected.get(dimension)
            if actual is None or actual.id not in allowed:
                return False, f"{component.id} is incompatible with {actual.id if actual else dimension}"
    return True, ""


def _preference_bonus(ir: RequirementIR, component: Component) -> tuple[float, list[str]]:
    bonus = 0.0
    trace: list[str] = []
    soft_design_fields = {
        "schema.mode", "correctness.isolation_level", "preference.execution_model",
        "preference.concurrency_control", "preference.storage_layout",
        "preference.storage_update", "preference.recovery", "preference.component",
    }
    for fact in ir.facts:
        if fact.strength != "soft" or fact.field not in soft_design_fields:
            continue
        value = str(fact.value)
        emitted_values = {str(item) for item in component.emits.values()}
        if value in {component.id, component.id.split(".", 1)[-1]} or value in emitted_values:
            amount = 2.0 * fact.weight * fact.confidence
            bonus += amount
            trace.append(f"{component.id} receives +{amount:.2f} from {fact.field}")
    return bonus, trace


def _augment(
    base: dict[str, Component], required: set[str], catalog: Catalog,
    objectives: dict[str, float], ir: RequirementIR,
) -> tuple[dict[str, Component] | None, list[str]]:
    selected = {component.id: component for component in base.values()}
    trace: list[str] = []
    for fact in ir.facts:
        if fact.field != "component.required" or fact.strength != "hard":
            continue
        component = catalog.by_id.get(str(fact.value))
        if component is not None and component.dimension == "augmentation":
            selected[component.id] = component
            trace.append(f"added required component {component.id}")
    while True:
        provided = set().union(*(component.provides for component in selected.values()))
        dependency_caps = set().union(*(component.requires_capabilities for component in selected.values()))
        missing = sorted((required | dependency_caps) - provided)
        if not missing:
            break
        capability = missing[0]
        providers = [
            component for component in catalog.providers(capability)
            if component.dimension == "augmentation" and component.id not in selected
        ]
        if not providers:
            return None, trace + [f"no provider for required capability {capability}"]
        def provider_rank(component: Component) -> tuple[float, float, str]:
            affinity = sum(objectives.get(name, 0.0) * value for name, value in component.affinity.items())
            preference, _ = _preference_bonus(ir, component)
            return (-(affinity + preference - 0.08 * component.complexity - 0.15 * component.risk), component.complexity, component.id)
        chosen = min(providers, key=provider_rank)
        selected[chosen.id] = chosen
        trace.append(f"added {chosen.id} to provide {capability}")
    ids = set(selected)
    for component in selected.values():
        if component.conflicts & ids or not component.requires_components.issubset(ids):
            return None, trace + [f"augmentation {component.id} violates a component constraint"]
    return selected, trace


def _candidate(
    base: dict[str, Component], ir: RequirementIR, catalog: Catalog,
    objectives: dict[str, float], rejection_reasons: Counter[str] | None = None,
) -> Candidate | None:
    okay, reason = _compatible(base)
    if not okay:
        if rejection_reasons is not None:
            rejection_reasons[reason] += 1
        return None
    required = _required_capabilities(ir, objectives)
    selected, closure_trace = _augment(base, required, catalog, objectives, ir)
    if selected is None:
        if rejection_reasons is not None:
            rejection_reasons[closure_trace[-1]] += 1
        return None
    utility = 0.0
    preference_trace: list[str] = []
    for component in selected.values():
        utility += sum(objectives.get(name, 0.0) * value for name, value in component.affinity.items())
        bonus, trace = _preference_bonus(ir, component)
        utility += bonus
        preference_trace.extend(trace)
    complexity = sum(component.complexity for component in selected.values())
    risk = sum(component.risk for component in selected.values())
    score = utility - 0.08 * complexity - 0.15 * risk
    ids = tuple(sorted(selected))
    assumptions = []
    if not any(f.field == "schema.mode" for f in ir.facts):
        assumptions.append("schema mode was selected from workload affinity")
    if not any(f.field == "correctness.isolation_level" for f in ir.facts):
        assumptions.append("isolation level was selected from correctness and complexity trade-offs")
    return Candidate(ids, score, utility, complexity, risk, tuple(assumptions), tuple([reason] + closure_trace + preference_trace))


def pareto_front(candidates: list[Candidate]) -> list[Candidate]:
    front: list[Candidate] = []
    for candidate in candidates:
        dominated = any(
            other.utility >= candidate.utility
            and other.complexity <= candidate.complexity
            and other.risk <= candidate.risk
            and (other.utility > candidate.utility or other.complexity < candidate.complexity or other.risk < candidate.risk)
            for other in candidates if other is not candidate
        )
        if not dominated:
            front.append(candidate)
    return sorted(front, key=lambda item: (-item.score, item.risk, item.complexity, item.components))


def solve(ir: RequirementIR, catalog: Catalog) -> tuple[list[Candidate], dict[str, Any]]:
    objectives = ir.objective_weights()
    forced, reasons = _hard_selections(ir, catalog)
    if reasons:
        raise UnsatisfiableRequirements(reasons)
    option_sets = [
        (catalog.by_id[forced[dimension]],) if dimension in forced else catalog.by_dimension[dimension]
        for dimension in EXCLUSIVE_DIMENSIONS
    ]
    feasible: list[Candidate] = []
    rejection_reasons: Counter[str] = Counter()
    rejected = 0
    for values in product(*option_sets):
        base = dict(zip(EXCLUSIVE_DIMENSIONS, values))
        candidate = _candidate(base, ir, catalog, objectives, rejection_reasons)
        if candidate is None:
            rejected += 1
        else:
            feasible.append(candidate)
    if not feasible:
        detail = ["no component composition satisfies all hard requirements"]
        detail.extend(f"forced {dimension}={component}" for dimension, component in sorted(forced.items()))
        detail.extend(
            f"rejected {count} compositions: {reason}"
            for reason, count in sorted(
                rejection_reasons.items(), key=lambda item: (-item[1], item[0])
            )[:8]
        )
        raise UnsatisfiableRequirements(detail)
    ranked = sorted(feasible, key=lambda item: (-item.score, item.risk, item.complexity, item.components))
    front = pareto_front(feasible)
    trace = {
        "objective_weights": objectives,
        "forced_dimensions": forced,
        "enumerated": len(feasible) + rejected,
        "feasible": len(feasible),
        "rejected": rejected,
        "pareto_size": len(front),
        "ranking": "hard constraints, unsupported/approximation avoidance, utility, risk, complexity, stable component id",
    }
    return ranked, trace


def clarification_candidates(candidates: list[Candidate], catalog: Catalog, *, limit: int = 8) -> list[dict[str, Any]]:
    sample = candidates[: max(2, limit)]
    if len(sample) < 2:
        return []
    questions = []
    question_data = __import__("json").loads((__import__("pathlib").Path(__file__).resolve().parent / "catalog" / "questions.json").read_text(encoding="utf-8"))["questions"]
    by_question = {item["dimension"]: item for item in question_data}
    for dimension in EXCLUSIVE_DIMENSIONS:
        groups: dict[str, list[Candidate]] = {}
        for candidate in sample:
            choice = next((item for item in candidate.components if catalog.by_id[item].dimension == dimension), "")
            groups.setdefault(choice, []).append(candidate)
        if len(groups) < 2 or dimension not in by_question:
            continue
        probabilities = [len(group) / len(sample) for group in groups.values()]
        information_gain = -sum(value * math.log2(value) for value in probabilities)
        impact = max(candidate.score for candidate in sample) - min(candidate.score for candidate in sample) + len(groups) / len(sample)
        template = by_question[dimension]
        value = information_gain * impact - float(template.get("burden", 1.0)) * 0.1
        questions.append({
            "dimension": dimension,
            "prompt": template["prompt"],
            "options": template["options"],
            "candidate_choices": {key: len(group) for key, group in sorted(groups.items())},
            "information_gain": round(information_gain, 6),
            "design_impact": round(impact, 6),
            "value": round(value, 6),
        })
    return sorted(questions, key=lambda item: (-item["value"], item["dimension"]))
