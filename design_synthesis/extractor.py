"""Evidence-preserving deterministic extraction for English requirement terms."""

from __future__ import annotations

import json
from pathlib import Path
import re
from typing import Any

from .catalog import CATALOG_DIR
from .requirements import RequirementIR, RequirementError, normalize_requirements


HARD_MARKERS = re.compile(r"(?:must|required|shall)", re.IGNORECASE)
CLAUSE_SPLIT = re.compile(r"[.!?;\n]+")
STRUCTURAL_FIELDS = {"schema.mode", "correctness.isolation_level"}


def _rules() -> list[dict[str, Any]]:
    return json.loads(
        (CATALOG_DIR / "intent_rules.json").read_text(encoding="utf-8")
    )["rules"]


def detect_languages(text: str) -> set[str]:
    detected: set[str] = set()
    for clause in (value.strip() for value in CLAUSE_SPLIT.split(text)):
        if not clause:
            continue
        for rule in _rules():
            if rule["field"] != "language.name":
                continue
            if any(re.search(pattern, clause, re.IGNORECASE) for pattern in rule["patterns"]):
                detected.add(rule["value"])
    return detected


def extract_text(
    text: str,
    *,
    crate_name: str,
    description: str = "",
    language: str | None = None,
) -> RequirementIR:
    """Extract bounded, evidence-cited facts without selecting components."""
    rules = _rules()
    facts_by_value: dict[tuple[str, str], dict[str, Any]] = {}
    detected_languages: set[str] = set()
    clauses = [clause.strip() for clause in CLAUSE_SPLIT.split(text) if clause.strip()]
    for clause in clauses:
        for rule in rules:
            if not any(re.search(pattern, clause, re.IGNORECASE) for pattern in rule["patterns"]):
                continue
            field = rule["field"]
            if field == "language.name":
                detected_languages.add(rule["value"])
                continue
            # Objectives and topology express optimization tendencies. Schema and
            # isolation terms identify semantic contracts even in declarative prose.
            if field in STRUCTURAL_FIELDS:
                strength = "hard"
            elif field.startswith("objective.") or field == "graph.topology":
                strength = "soft"
            else:
                strength = "hard" if HARD_MARKERS.search(clause) else "soft"
            fact = {
                "field": field,
                "value": rule["value"],
                "strength": strength,
                "confidence": 0.9,
                "weight": 1.0,
                "evidence": clause,
                "source": f"rule:{rule['id']}",
            }
            key = (field, json.dumps(rule["value"], ensure_ascii=False, sort_keys=True))
            previous = facts_by_value.get(key)
            if previous is None or (
                previous["strength"] == "soft" and strength == "hard"
            ):
                facts_by_value[key] = fact

    selected_language = language or (
        next(iter(detected_languages)) if len(detected_languages) == 1 else ""
    )
    if not selected_language:
        raise RequirementError(
            "text does not identify exactly one language; pass --language or use --llm-fallback"
        )
    if language and detected_languages and language not in detected_languages:
        raise RequirementError(
            f"--language {language} conflicts with text evidence {sorted(detected_languages)}"
        )

    raw = {
        "schema_version": 1,
        "candidate": {"crate_name": crate_name, "description": description},
        "language": selected_language,
        "facts": list(facts_by_value.values()),
        "source": {"kind": "text"},
    }
    ir = normalize_requirements(raw)
    ir.source = {
        "kind": "text",
        "matched_facts": len(facts_by_value),
        "detected_languages": sorted(detected_languages),
    }
    if not facts_by_value:
        ir.unknowns.append("no domain or workload fact was recognized")
    return ir
