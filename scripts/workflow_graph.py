"""Implementation prerequisites and generation-time upstream return edges.

Normal code generation follows prerequisite edges.  If an active stage discovers
that an earlier implementation cannot support it, the agent returns to that
owner, revises it, and revalidates affected stages.  The return edge makes the
actual generation path cyclic.
"""

from __future__ import annotations

PHASES = (
    "00_foundation", "01_transactions", "02_storage", "03_algorithms",
    "04_language_ast", "05_language_lowering", "06_parser", "07_planner",
    "08_optimizer", "09_executor_tests", "10_assembly",
    "11_build_integration", "12_benchmark_fix", "13_performance_optimization",
)

DEPENDENCIES: dict[str, tuple[str, ...]] = {
    "00_foundation": (),
    "01_transactions": ("00_foundation",),
    "02_storage": ("00_foundation", "01_transactions"),
    "03_algorithms": ("00_foundation", "02_storage"),
    "04_language_ast": ("00_foundation",),
    "05_language_lowering": ("04_language_ast",),
    "06_parser": ("05_language_lowering",),
    "07_planner": ("00_foundation", "06_parser"),
    "08_optimizer": ("00_foundation", "07_planner"),
    "09_executor_tests": (
        "02_storage", "03_algorithms", "07_planner", "08_optimizer",
    ),
    "10_assembly": ("09_executor_tests",),
    "11_build_integration": ("10_assembly",),
    "12_benchmark_fix": ("11_build_integration",),
    "13_performance_optimization": ("12_benchmark_fix",),
}

def ready_phase(phases: list[dict]) -> str | None:
    """Choose the first pending node whose required predecessors have passed."""
    statuses = {row["name"]: row["status"] for row in phases}
    if len(statuses) != len(PHASES) or set(statuses) != set(PHASES):
        raise ValueError("phase state does not match the workflow graph")
    for phase in PHASES:
        if statuses[phase] == "pending" and all(
            statuses[dependency] in {"passed", "completed_with_limitations"}
            for dependency in DEPENDENCIES[phase]
        ):
            return phase
    return None


def affected_phases(root: str) -> set[str]:
    """Invalidate the repaired component and every transitive consumer."""
    if root not in DEPENDENCIES:
        raise ValueError(f"unknown workflow phase: {root}")
    affected = {root}
    while True:
        expanded = affected | {
            phase for phase, dependencies in DEPENDENCIES.items()
            if set(dependencies) & affected
        }
        if expanded == affected:
            return affected
        affected = expanded


def return_target(origin: str, target: str) -> str:
    """Validate an agent-selected runtime edge to an earlier phase."""
    if origin not in PHASES:
        raise ValueError(f"unknown return origin: {origin}")
    if target not in PHASES:
        raise ValueError(f"unknown return target: {target}")
    if PHASES.index(target) >= PHASES.index(origin):
        raise ValueError("return target must precede the active phase")
    return target


def return_affected(origin: str, target: str) -> set[str]:
    """Recheck the traversed interval and every affected graph consumer."""
    return_target(origin, target)
    interval = set(PHASES[PHASES.index(target):PHASES.index(origin) + 1])
    return interval | affected_phases(target) | affected_phases(origin)


def _validate_graph() -> None:
    if set(DEPENDENCIES) != set(PHASES):
        raise ValueError("dependency graph is missing phases")
    order = {phase: index for index, phase in enumerate(PHASES)}
    for phase, dependencies in DEPENDENCIES.items():
        if any(dependency not in order or order[dependency] >= order[phase]
               for dependency in dependencies):
            raise ValueError(f"invalid generation dependency at {phase}")


_validate_graph()
