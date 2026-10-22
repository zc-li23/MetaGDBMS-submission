"""Load and validate the extensible component capability graph."""

from __future__ import annotations

from dataclasses import dataclass, field
import json
from pathlib import Path
from typing import Any

from .requirements import RequirementError


CATALOG_DIR = Path(__file__).resolve().parent / "catalog"
EXCLUSIVE_DIMENSIONS = (
    "schema", "concurrency_control", "isolation", "execution",
    "storage_layout", "storage_update", "recovery",
)
MATURITY_RISK = {"validated": 0.0, "supported": 0.25, "experimental": 1.0}


@dataclass(frozen=True)
class Component:
    id: str
    dimension: str
    description: str
    emits: dict[str, Any] = field(default_factory=dict)
    provides: frozenset[str] = frozenset()
    requires_capabilities: frozenset[str] = frozenset()
    requires_components: frozenset[str] = frozenset()
    conflicts: frozenset[str] = frozenset()
    compatible_with: dict[str, tuple[str, ...]] = field(default_factory=dict)
    affinity: dict[str, float] = field(default_factory=dict)
    complexity: float = 0.0
    risk: float = 0.0
    maturity: str = "experimental"

    @classmethod
    def from_dict(cls, raw: dict[str, Any], *, custom: bool = False) -> "Component":
        if not isinstance(raw, dict):
            raise RequirementError("component must be an object")
        required = {"id", "dimension", "description"}
        missing = required - set(raw)
        if missing:
            raise RequirementError("component is missing: " + ", ".join(sorted(missing)))
        dimension = str(raw["dimension"])
        if dimension not in set(EXCLUSIVE_DIMENSIONS) | {"augmentation"}:
            raise RequirementError(f"component {raw['id']} has unknown dimension {dimension}")
        maturity = str(raw.get("maturity", "experimental" if custom else "supported"))
        if maturity not in MATURITY_RISK:
            raise RequirementError(f"component {raw['id']} has unknown maturity {maturity}")
        for key in ("emits", "compatible_with", "affinity"):
            if not isinstance(raw.get(key, {}), dict):
                raise RequirementError(f"component {raw['id']} {key} must be an object")
        for key in ("provides", "requires_capabilities", "requires", "requires_components", "conflicts"):
            value = raw.get(key, [])
            if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
                raise RequirementError(f"component {raw['id']} {key} must be an array of strings")
        for key, value in raw.get("compatible_with", {}).items():
            if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
                raise RequirementError(f"component {raw['id']} compatible_with.{key} must be an array of strings")
        if custom:
            required_emits = {
                "schema": "mode",
                "concurrency_control": "concurrency_control",
                "isolation": "isolation_level",
                "execution": "model",
                "storage_layout": "layout",
                "storage_update": "update",
                "recovery": "recovery",
            }
            field_name = required_emits.get(dimension)
            emitted = raw.get("emits", {}).get(field_name) if field_name else None
            if field_name and (not isinstance(emitted, str) or not emitted.strip()):
                raise RequirementError(f"component {raw['id']} must emit {field_name}")
            if dimension == "execution" and raw["emits"]["model"] not in {"volcano", "vectorized", "push_based"}:
                raise RequirementError(f"component {raw['id']} has unsupported execution model")
        return cls(
            id=str(raw["id"]),
            dimension=dimension,
            description=str(raw["description"]),
            emits=dict(raw.get("emits", {})),
            provides=frozenset(map(str, raw.get("provides", []))),
            requires_capabilities=frozenset(map(str, raw.get("requires_capabilities", raw.get("requires", [])))),
            requires_components=frozenset(map(str, raw.get("requires_components", []))),
            conflicts=frozenset(map(str, raw.get("conflicts", []))),
            compatible_with={key: tuple(map(str, value)) for key, value in raw.get("compatible_with", {}).items()},
            affinity={key: float(value) for key, value in raw.get("affinity", {}).items()},
            complexity=float(raw.get("complexity", 1.5 if custom else 0.0)),
            risk=float(raw.get("risk", 0.8 if custom else 0.0)) + MATURITY_RISK[maturity],
            maturity=maturity,
        )


class Catalog:
    def __init__(self, components: list[Component]):
        self.components = tuple(components)
        self.by_id = {component.id: component for component in components}
        if len(self.by_id) != len(components):
            raise RequirementError("component ids must be unique")
        self.by_dimension = {
            dimension: tuple(component for component in components if component.dimension == dimension)
            for dimension in (*EXCLUSIVE_DIMENSIONS, "augmentation")
        }
        for dimension in EXCLUSIVE_DIMENSIONS:
            if not self.by_dimension[dimension]:
                raise RequirementError(f"catalog has no options for {dimension}")
        for component in components:
            unknown = (component.requires_components | component.conflicts) - set(self.by_id)
            if unknown:
                raise RequirementError(f"component {component.id} references unknown components: {sorted(unknown)}")

    def providers(self, capability: str) -> list[Component]:
        return [component for component in self.components if capability in component.provides]


def load_catalog(custom_components: list[dict[str, Any]] | None = None) -> Catalog:
    raw = json.loads((CATALOG_DIR / "components.json").read_text(encoding="utf-8"))
    components = [Component.from_dict(item) for item in raw["components"]]
    components.extend(Component.from_dict(item, custom=True) for item in (custom_components or []))
    return Catalog(components)
