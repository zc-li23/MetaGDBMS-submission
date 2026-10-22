"""Optional OpenAI-compatible extraction adapter for unresolved requirements.

The model may propose evidence-cited requirement facts and paper-specific
components. It never emits the final design vector; the deterministic catalog,
constraint solver, and backend validator remain authoritative.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
import hashlib
import json
import os
from pathlib import Path
import re
import unicodedata
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

from .catalog import Component, load_catalog
from .requirements import (
    Fact,
    LEVELS,
    OBJECTIVES,
    RequirementError,
    RequirementIR,
    normalize_requirements,
)


LANGUAGES = {"Cypher", "GQL", "Gremlin"}
SCHEMA_MODES = {"strict", "hybrid", "schemaless"}
GRAPH_TOPOLOGIES = {"power_law_social", "sparse_railway", "uniform_regular"}
ISOLATION_LEVELS = {
    "read_committed", "repeatable_read", "snapshot_isolation", "serializable",
}
ALLOWED_FACT_FIELDS = {
    "schema.mode",
    "graph.topology",
    "correctness.isolation_level",
    "preference.concurrency_control",
} | {f"objective.{name}" for name in OBJECTIVES}


@dataclass(frozen=True)
class LlmConfig:
    api_key: str
    api_base: str = "https://api.deepseek.com"
    model: str = "deepseek-chat"
    temperature: float = 0.0
    timeout_seconds: float = 120.0
    max_tokens: int | None = None

    @classmethod
    def from_env(cls, root: Path, env_file: str | Path = ".env") -> "LlmConfig":
        path = Path(env_file)
        if not path.is_absolute():
            path = root / path
        load_dotenv(path)
        api_key = os.getenv("LLM_API_KEY", "").strip()
        if not api_key:
            raise RequirementError(
                f"LLM mode requested but LLM_API_KEY is empty; fill {path} or export the variable"
            )
        try:
            temperature = float(os.getenv("LLM_TEMPERATURE", "0"))
            timeout = float(os.getenv("LLM_TIMEOUT_SECONDS", "120"))
        except ValueError as exc:
            raise RequirementError("LLM_TEMPERATURE and LLM_TIMEOUT_SECONDS must be numbers") from exc
        return cls(
            api_key=api_key,
            api_base=os.getenv("LLM_API_BASE", "").strip() or "https://api.deepseek.com",
            model=os.getenv("LLM_MODEL", "").strip() or "deepseek-chat",
            temperature=temperature,
            timeout_seconds=timeout,
        )


class InvalidLlmJson(RequirementError):
    def __init__(self, response_text: str):
        super().__init__("LLM response does not contain one valid JSON object")
        self.response_text = response_text


def load_dotenv(path: Path) -> None:
    """Load a small dotenv file without adding a runtime dependency."""
    if not path.exists():
        return
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", key):
            continue
        if key not in os.environ:
            os.environ[key] = value.strip().strip('"').strip("'")


def _json_object(text: str) -> dict[str, Any]:
    stripped = text.strip()
    attempts = [stripped]
    fenced = re.search(r"```(?:json)?\s*(\{.*\})\s*```", stripped, re.DOTALL | re.IGNORECASE)
    if fenced:
        attempts.append(fenced.group(1))
    first, last = stripped.find("{"), stripped.rfind("}")
    if first >= 0 and last > first:
        attempts.append(stripped[first : last + 1])
    for value in attempts:
        try:
            parsed = json.loads(value)
        except json.JSONDecodeError:
            continue
        if isinstance(parsed, dict):
            return parsed
    # Some JSON-mode endpoints append a single unmatched closing brace after
    # an otherwise complete object. This fallback accepts only a brace suffix,
    # never a second JSON value.
    try:
        parsed, end = json.JSONDecoder().raw_decode(stripped)
    except json.JSONDecodeError:
        pass
    else:
        if isinstance(parsed, dict) and re.fullmatch(r"\}+", stripped[end:].strip()):
            return parsed
    raise InvalidLlmJson(text)


def call_chat_completion(config: LlmConfig, *, system_prompt: str, prompt: str) -> tuple[dict[str, Any], str]:
    """Call an OpenAI-compatible chat/completions endpoint."""
    endpoint = config.api_base.rstrip("/") + "/chat/completions"
    request_body = {
        "model": config.model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": prompt},
        ],
        "temperature": config.temperature,
        "response_format": {"type": "json_object"},
    }
    if config.max_tokens is not None:
        request_body["max_tokens"] = config.max_tokens
    body = json.dumps(request_body).encode("utf-8")
    request = Request(
        endpoint,
        data=body,
        method="POST",
        headers={
            "Authorization": f"Bearer {config.api_key}",
            "Content-Type": "application/json",
        },
    )
    try:
        with urlopen(request, timeout=config.timeout_seconds) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")[:1000]
        raise RequirementError(f"LLM API returned HTTP {exc.code}: {detail}") from exc
    except URLError as exc:
        raise RequirementError(f"LLM API connection failed: {exc.reason}") from exc
    except TimeoutError as exc:
        raise RequirementError("LLM API request timed out") from exc
    try:
        content = payload["choices"][0]["message"]["content"]
    except (KeyError, IndexError, TypeError) as exc:
        raise RequirementError("LLM API response has no assistant message") from exc
    if not isinstance(content, str) or not content.strip():
        raise InvalidLlmJson(content if isinstance(content, str) else "")
    try:
        return _json_object(content), content
    except InvalidLlmJson as exc:
        reason = payload.get("choices", [{}])[0].get("finish_reason", "unknown")
        exc.finish_reason = reason
        raise


def _normalized(value: str, *, punctuation: bool = False) -> str:
    value = unicodedata.normalize("NFKC", value).replace("\u00ad", "")
    value = re.sub(r"-\s*\n\s*", "", value)
    if punctuation:
        value = "".join(
            character if character.isalnum() else " "
            for character in value
        )
    return re.sub(r"\s+", " ", value).strip().casefold()


def _without_acm_page_furniture(source_text: str) -> str:
    """Remove a journal footer, running title, and page marker as one unit.

    pdftotext can place this three-line block between the two halves of a
    sentence. Requiring the complete block avoids skipping ordinary paper text.
    """
    lines = source_text.splitlines()
    kept: list[str] = []
    index = 0
    while index < len(lines):
        if re.match(r"^Proc\. ACM Manag\. Data,", lines[index].strip(), re.I):
            title = index + 1
            while title < len(lines) and not lines[title].strip():
                title += 1
            marker = title + 1
            while marker < len(lines) and not lines[marker].strip():
                marker += 1
            if (
                title < len(lines)
                and 8 <= len(lines[title].strip()) <= 160
                and marker < len(lines)
                and re.fullmatch(r"\d{1,3}:\d{1,3}", lines[marker].strip())
            ):
                index = marker + 1
                continue
        kept.append(lines[index])
        index += 1
    return "\n".join(kept)


def _evidence_present(evidence: str, source_text: str) -> bool:
    exact = _normalized(evidence)
    source = _normalized(source_text)
    if exact and exact in source:
        return True
    words = _normalized(evidence, punctuation=True)
    source_words = _normalized(source_text, punctuation=True)
    if words and words in source_words:
        return True
    # PDF extraction can split a printed hyphenated word across lines while a
    # model quotes the normally hyphenated or unhyphenated spelling. Compare a
    # substantial alphanumeric form as a final literal-quote normalization.
    compact = "".join(character for character in words if character.isalnum())
    source_compact = "".join(
        character for character in source_words if character.isalnum()
    )
    if len(compact) >= 16 and compact in source_compact:
        return True
    # Models sometimes preserve exact text on both sides of a printed ellipsis.
    # Accept only when every substantial segment occurs in source order.
    segments = [
        _normalized(value, punctuation=True)
        for value in re.split(r"(?:\.\.\.|…+)", evidence)
    ]
    segments = [value for value in segments if len(value) >= 12]
    if len(segments) >= 2:
        offset = 0
        for segment in segments:
            found = source_words.find(segment, offset)
            if found < 0:
                break
            offset = found + len(segment)
        else:
            return True
    without_furniture = _without_acm_page_furniture(source_text)
    return without_furniture != source_text and _evidence_present(evidence, without_furniture)


def _validate_fact_value(field: str, value: Any) -> None:
    if field == "schema.mode" and value not in SCHEMA_MODES:
        raise RequirementError(f"unsupported schema mode {value!r}")
    if field == "graph.topology" and value not in GRAPH_TOPOLOGIES:
        raise RequirementError(f"unsupported graph topology {value!r}")
    if field == "correctness.isolation_level" and value not in ISOLATION_LEVELS:
        raise RequirementError(f"unsupported isolation level {value!r}")
    if field == "preference.concurrency_control" and value not in {"mvcc", "2pl", "occ", "single_writer"}:
        raise RequirementError(f"unsupported concurrency control {value!r}")
    if field.startswith("objective."):
        if isinstance(value, str) and value in LEVELS:
            return
        if (
            isinstance(value, (int, float))
            and not isinstance(value, bool)
            and value >= 0
        ):
            return
        raise RequirementError(
            f"objective value must be one of {sorted(LEVELS)} or a non-negative number"
        )


def _require_evidence(item: dict[str, Any], source_text: str, label: str) -> None:
    evidence = item.get("evidence")
    if not isinstance(evidence, str) or len(evidence.strip()) < 8:
        raise RequirementError(f"{label} requires an evidence quote of at least 8 characters")
    if not _evidence_present(evidence, source_text):
        raise RequirementError(f"{label} evidence is not present in the supplied source text")


def _rejected(label: str, reason: Exception | str, item: Any) -> dict[str, Any]:
    evidence = item.get("evidence", "") if isinstance(item, dict) else ""
    return {
        "item": label,
        "reason": str(reason),
        "evidence": str(evidence)[:240],
    }


def _cited_values(
    values: Any,
    source_text: str,
    label: str,
    rejected: list[dict[str, Any]],
) -> list[str]:
    if values is None:
        return []
    if not isinstance(values, list):
        rejected.append(_rejected(label, "must be an array", values))
        return []
    result: list[str] = []
    for index, item in enumerate(values):
        item_label = f"{label}[{index}]"
        try:
            if not isinstance(item, dict) or not isinstance(item.get("value"), str):
                raise RequirementError("must contain string value and evidence")
            _require_evidence(item, source_text, item_label)
        except RequirementError as exc:
            rejected.append(_rejected(item_label, exc, item))
            continue
        result.append(item["value"].strip())
    return result


def validate_proposal(raw: dict[str, Any], source_text: str, *, allow_custom_components: bool) -> dict[str, Any]:
    """Drop uncited model claims while preserving every rejection for review."""
    allowed_root = {
        "language", "facts", "schema", "graph", "algorithms",
        "capabilities", "storage", "unknowns",
    }
    unknown_root = set(raw) - allowed_root
    if unknown_root:
        raise RequirementError(f"LLM proposal has unsupported fields: {sorted(unknown_root)}")

    rejected: list[dict[str, Any]] = []
    normalizations: list[str] = []
    language_raw = raw.get("language")
    language = None
    language_evidence = None
    if language_raw is not None:
        try:
            if not isinstance(language_raw, dict):
                raise RequirementError("must contain name and evidence")
            _require_evidence(language_raw, source_text, "LLM language")
            candidate_language = language_raw.get("name")
            if candidate_language not in LANGUAGES:
                raise RequirementError("must be Cypher, GQL, or Gremlin")
            language = candidate_language
            language_evidence = language_raw["evidence"].strip()
        except RequirementError as exc:
            rejected.append(_rejected("LLM language", exc, language_raw))

    facts: list[dict[str, Any]] = []
    raw_facts = raw.get("facts", [])
    if not isinstance(raw_facts, list):
        rejected.append(_rejected("LLM facts", "must be an array", raw_facts))
        raw_facts = []
    for index, item in enumerate(raw_facts):
        label = f"LLM fact {index}"
        try:
            if not isinstance(item, dict):
                raise RequirementError("must be an object")
            _require_evidence(item, source_text, label)
            field = item.get("field")
            if field not in ALLOWED_FACT_FIELDS:
                raise RequirementError(f"has unsupported field {field!r}")
            _validate_fact_value(field, item.get("value"))
            confidence = min(float(item.get("confidence", 0.7)), 0.8)
            if not 0 <= confidence <= 0.8:
                raise RequirementError("confidence must be between 0 and 0.8")
            fact = {
                "field": field,
                "value": item.get("value"),
                "strength": item.get("strength", "soft"),
                "weight": float(item.get("weight", 1.0)),
                "confidence": confidence,
                "evidence": item["evidence"].strip(),
                "source": "llm:evidence_extractor",
            }
            Fact(**fact)
        except (RequirementError, TypeError, ValueError) as exc:
            rejected.append(_rejected(label, exc, item))
            continue
        facts.append(fact)

    schema_raw = raw.get("schema", {})
    graph_raw = raw.get("graph", {})
    capabilities = raw.get("capabilities", {})
    storage = raw.get("storage", {})
    for label, value in (
        ("schema", schema_raw), ("graph", graph_raw),
        ("capabilities", capabilities), ("storage", storage),
    ):
        if not isinstance(value, dict):
            rejected.append(_rejected(f"LLM {label}", "must be an object", value))
    schema_raw = schema_raw if isinstance(schema_raw, dict) else {}
    graph_raw = graph_raw if isinstance(graph_raw, dict) else {}
    capabilities = capabilities if isinstance(capabilities, dict) else {}
    storage = storage if isinstance(storage, dict) else {}

    schema: dict[str, Any] = {}
    if set(schema_raw) - {"mode", "vertex_labels", "edge_types", "property_keys", "evidence"}:
        rejected.append(_rejected("LLM schema", "contains unsupported fields", schema_raw))
    elif any(key in schema_raw for key in ("mode", "vertex_labels", "edge_types", "property_keys")):
        try:
            _require_evidence(schema_raw, source_text, "LLM schema declaration")
            if "mode" in schema_raw and not any(
                fact["field"] == "schema.mode" and fact["value"] == schema_raw["mode"]
                for fact in facts
            ):
                raise RequirementError(
                    "schema.mode requires a matching accepted schema.mode fact"
                )
            schema = dict(schema_raw)
        except RequirementError as exc:
            rejected.append(_rejected("LLM schema declaration", exc, schema_raw))

    graph: dict[str, Any] = {}
    if set(graph_raw) - {"topology", "evidence"}:
        rejected.append(_rejected("LLM graph", "contains unsupported fields", graph_raw))
    elif "topology" in graph_raw:
        try:
            _require_evidence(graph_raw, source_text, "LLM graph topology")
            if not any(
                fact["field"] == "graph.topology" and fact["value"] == graph_raw["topology"]
                for fact in facts
            ):
                raise RequirementError(
                    "graph.topology requires a matching accepted graph.topology fact"
                )
            graph = dict(graph_raw)
        except RequirementError as exc:
            rejected.append(_rejected("LLM graph topology", exc, graph_raw))

    algorithms: list[dict[str, Any]] = []
    raw_algorithms = raw.get("algorithms", [])
    if not isinstance(raw_algorithms, list):
        rejected.append(_rejected("LLM algorithms", "must be an array", raw_algorithms))
        raw_algorithms = []
    for index, item in enumerate(raw_algorithms):
        label = f"LLM algorithm {index}"
        try:
            if not isinstance(item, dict):
                raise RequirementError("must be an object")
            _require_evidence(item, source_text, label)
            name = str(item.get("name", "")).strip()
            description = str(item.get("description", "")).strip()
            if not name or not description:
                raise RequirementError("requires name and description")
            if allow_custom_components and "join" in name.casefold():
                raise RequirementError("query join operator is an execution component, not a graph-analysis algorithm")
            if not re.fullmatch(r"[A-Za-z][A-Za-z0-9_.-]*", name):
                raise RequirementError("name must start with a letter and contain only letters, digits, '_', '-' or '.'")
        except RequirementError as exc:
            rejected.append(_rejected(label, exc, item))
            continue
        algorithms.append({
            "name": name,
            "description": description,
            **({"performance_weight": item["performance_weight"]} if "performance_weight" in item else {}),
            "evidence": item["evidence"].strip(),
        })

    algorithm_weights = [item.get("performance_weight") for item in algorithms]
    if any(weight is not None for weight in algorithm_weights):
        valid_weights = (
            all(
                isinstance(weight, (int, float))
                and not isinstance(weight, bool)
                and weight >= 0
                for weight in algorithm_weights
            )
            and all(weight is not None for weight in algorithm_weights)
            and abs(sum(float(weight) for weight in algorithm_weights) - 1.0) <= 1e-6
        )
        if not valid_weights:
            rejected.append(_rejected(
                "LLM algorithm performance weights",
                "must be non-negative, supplied for every accepted algorithm, and sum to 1.0",
                {},
            ))
            for item in algorithms:
                item.pop("performance_weight", None)

    required_capabilities = _cited_values(
        capabilities.get("required", []), source_text,
        "LLM required capability", rejected,
    )
    optional_capabilities = _cited_values(
        capabilities.get("optional", []), source_text,
        "LLM optional capability", rejected,
    )
    storage_capabilities = _cited_values(
        storage.get("required_capabilities", []), source_text,
        "LLM storage capability", rejected,
    )

    custom = storage.get("custom_components", [])
    required_raw = storage.get("required_components", [])
    preferred_raw = storage.get("preferred_components", [])
    if not isinstance(custom, list):
        rejected.append(_rejected("custom components", "must be an array", custom))
        custom = []
    if not isinstance(required_raw, list):
        rejected.append(_rejected("required components", "must be an array", required_raw))
        required_raw = []
    if not isinstance(preferred_raw, list):
        rejected.append(_rejected("preferred components", "must be an array", preferred_raw))
        preferred_raw = []

    custom_ids: set[str] = set()
    clean_custom: list[dict[str, Any]] = []
    for index, item in enumerate(custom):
        label = f"custom component {index}"
        try:
            if not allow_custom_components:
                raise RequirementError("business-text fallback may not invent storage components")
            if not isinstance(item, dict):
                raise RequirementError("must be an object")
            _require_evidence(item, source_text, label)
            component_id = str(item.get("id", ""))
            if not re.fullmatch(r"paper\.[a-z0-9_.-]+", component_id):
                raise RequirementError("id must start with 'paper.'")
            if component_id in custom_ids:
                raise RequirementError(f"duplicate id {component_id}")
            item = dict(item)
            compatibility = item.get("compatible_with", {})
            if isinstance(compatibility, dict):
                dependency_tokens = {
                    key: value for key, value in compatibility.items()
                    if key.startswith("paper.") and value in ("required", "optional")
                }
                if dependency_tokens:
                    item["compatible_with"] = {
                        key: value for key, value in compatibility.items()
                        if key not in dependency_tokens
                    }
                    item["requires_components"] = list(dict.fromkeys([
                        *item.get("requires_components", []),
                        *(key for key, value in dependency_tokens.items() if value == "required"),
                    ]))
                    normalizations.append(
                        f"Normalized {component_id} named component dependencies from compatible_with"
                    )
            if item.get("dimension") == "execution" and "join" in component_id:
                item["dimension"] = "augmentation"
                normalizations.append(
                    f"Normalized {component_id} query operator to augmentation dimension"
                )
            if item.get("dimension") != "augmentation" and "buffer" in component_id:
                item["dimension"] = "augmentation"
                normalizations.append(
                    f"Normalized {component_id} buffer manager to augmentation dimension"
                )
            if (
                item.get("dimension") == "storage_layout"
                and any(token in component_id for token in ("adjacency", "index"))
                and "disk_records" not in item.get("provides", [])
            ):
                item["dimension"] = "augmentation"
                normalizations.append(
                    f"Normalized {component_id} coexisting adjacency/index layer to augmentation"
                )
            if (
                item.get("dimension") == "storage_layout"
                and isinstance(item.get("provides", []), list)
                and "disk_records" not in item.get("provides", [])
            ):
                item["provides"] = [*item.get("provides", []), "disk_records"]
                normalizations.append(
                    f"Normalized {component_id} storage layout to provide disk_records"
                )
            if (
                item.get("dimension") == "recovery"
                and isinstance(item.get("emits"), dict)
                and item.get("emits", {}).get("recovery") == "wal"
                and isinstance(item.get("provides", []), list)
                and all(isinstance(value, str) for value in item.get("provides", []))
            ):
                missing = {"crash_safe", "atomic_commit"} - set(item["provides"])
                if missing:
                    item["provides"] = [*item["provides"], *sorted(missing)]
                    normalizations.append(
                        f"Normalized {component_id} WAL recovery to provide atomic_commit and crash_safe"
                    )
            if item.get("maturity") == "prototype":
                item["maturity"] = "experimental"
                normalizations.append(
                    f"Normalized {component_id} maturity from prototype to experimental"
                )
            Component.from_dict(item, custom=True)
        except (RequirementError, TypeError, ValueError) as exc:
            detail = _rejected(label, exc, item)
            if isinstance(item, dict):
                detail["raw_component"] = item
            rejected.append(detail)
            continue
        custom_ids.add(component_id)
        clean_custom.append(dict(item))

    builtin_ids = set(load_catalog().by_id)
    while True:
        known_ids = builtin_ids | custom_ids
        invalid = [
            item for item in clean_custom
            if set(item.get("requires_components", [])) - known_ids
        ]
        if not invalid:
            break
        for item in invalid:
            missing = sorted(set(item["requires_components"]) - known_ids)
            detail = _rejected(
                f"custom component {item['id']}",
                f"requires unavailable component {missing[0]}", item,
            )
            detail["raw_component"] = item
            rejected.append(detail)
            clean_custom.remove(item)
            custom_ids.remove(item["id"])

    while True:
        available = {
            capability
            for component in load_catalog().components
            for capability in component.provides
        }
        for item in clean_custom:
            available.update(Component.from_dict(item, custom=True).provides)
        invalid = [
            (item, sorted(Component.from_dict(item, custom=True).requires_capabilities - available))
            for item in clean_custom
        ]
        invalid = [(item, missing) for item, missing in invalid if missing]
        if not invalid:
            break
        for item, missing in invalid:
            detail = _rejected(
                f"custom component {item['id']}",
                f"no accepted component provides required capability {missing[0]!r}", item,
            )
            detail["raw_component"] = item
            rejected.append(detail)
            clean_custom.remove(item)
            custom_ids.remove(item["id"])
    while True:
        known_ids = builtin_ids | custom_ids
        invalid = [
            item for item in clean_custom
            if set(item.get("requires_components", [])) - known_ids
        ]
        if not invalid:
            break
        for item in invalid:
            missing = sorted(set(item["requires_components"]) - known_ids)
            detail = _rejected(
                f"custom component {item['id']}",
                f"requires unavailable component {missing[0]}", item,
            )
            detail["raw_component"] = item
            rejected.append(detail)
            clean_custom.remove(item)
            custom_ids.remove(item["id"])

    available_capabilities = {
        capability
        for component in load_catalog().components
        for capability in component.provides
    }
    for item in clean_custom:
        available_capabilities.update(Component.from_dict(item, custom=True).provides)
    checked_storage_capabilities: list[str] = []
    for capability in storage_capabilities:
        if capability in available_capabilities:
            checked_storage_capabilities.append(capability)
        else:
            rejected.append(_rejected(
                "LLM storage capability",
                f"no catalog or accepted paper component provides {capability!r}",
                {},
            ))

    required: list[str] = []
    preferred: list[str] = []
    for label, values, destination in (
        ("required component", required_raw, required),
        ("preferred component", preferred_raw, preferred),
    ):
        for index, component_id in enumerate(values):
            if isinstance(component_id, dict):
                component_id = component_id.get("id")
            if not isinstance(component_id, str):
                rejected.append(_rejected(
                    f"{label} {index}", "must be a component id string", {},
                ))
                continue
            if component_id not in custom_ids:
                rejected.append(_rejected(
                    f"{label} {index}",
                    f"{component_id!r} has no accepted evidence-cited paper component",
                    {},
                ))
                continue
            destination.append(component_id)

    raw_unknowns = raw.get("unknowns", [])
    if not isinstance(raw_unknowns, list):
        rejected.append(_rejected("LLM unknowns", "must be an array", raw_unknowns))
        raw_unknowns = []
    unknowns = [str(value) for value in raw_unknowns]
    unknowns.extend(normalizations)
    unknowns.extend(
        f"Rejected {item['item']}: {item['reason']}"
        for item in rejected
    )
    return {
        "language": language,
        "language_evidence": language_evidence,
        "facts": facts,
        "schema": schema,
        "graph": graph,
        "algorithms": algorithms,
        "capabilities": {
            "required": required_capabilities,
            "optional": optional_capabilities,
        },
        "storage": {
            "required_capabilities": checked_storage_capabilities,
            "required_components": required,
            "preferred_components": preferred,
            "custom_components": clean_custom,
        },
        "unknowns": unknowns,
        "rejected": rejected,
    }


SYSTEM_PROMPT = """You extract graph-database requirements. Return JSON only.
You do not choose a final architecture. Every proposed fact, algorithm, and
paper-specific component must quote exact evidence from the supplied text.
Do not infer unsupported guarantees. Unknown or ambiguous items go in unknowns.
The deterministic MetaGDB constraint solver will reject incompatible designs."""

JSON_SHAPE = """Return this shape:
{
  "language": {"name": "Cypher|GQL|Gremlin", "evidence": "exact quote"} or null,
  "facts": [{"field": "schema.mode|graph.topology|correctness.isolation_level|preference.concurrency_control|objective.<name>",
             "value": "...", "strength": "hard|soft", "confidence": 0.0,
             "evidence": "exact quote"}],
  "schema": {"mode": "strict|hybrid|schemaless", "vertex_labels": [], "edge_types": [],
             "property_keys": [], "evidence": "exact quote"},
  "graph": {"topology": "power_law_social|sparse_railway|uniform_regular",
            "evidence": "exact quote"},
  "algorithms": [{"name": "stable_identifier", "description": "...", "evidence": "exact quote"}],
  "capabilities": {
      "required": [{"value": "...", "evidence": "exact quote"}],
      "optional": [{"value": "...", "evidence": "exact quote"}]
  },
  "storage": {"required_capabilities": [{"value": "exact capability token", "evidence": "exact quote"}],
              "required_components": ["paper.component_id"],
              "preferred_components": ["paper.component_id"], "custom_components": []},
  "unknowns": []
}
Use only fields supported by this shape. Objective names are point_read,
traversal, scan, write, analytics, recovery, schema_rigidity,
write_flexibility, read_concurrency, high_contention, low_contention,
implementation_simplicity, and disk_efficiency."""


def propose_requirements(
    source_text: str,
    config: LlmConfig,
    *,
    paper_mode: bool,
    deterministic_summary: dict[str, Any] | None = None,
) -> tuple[dict[str, Any], str]:
    custom = """
For a paper, represent genuinely paper-specific storage or execution layers as
custom_components. Each needs id paper.<slug>, dimension, description, emits,
provides, required dependencies, affinity, complexity, risk, maturity, and an
exact evidence quote. Put every architecture-defining paper component in
required_components. Do not rename an ordinary built-in idea merely to create a
custom component. Component dimension must be exactly one of storage_layout,
storage_update, recovery, execution, or augmentation; a storage engine belongs in
storage_layout. Do not create a database_system component. In required_components
and preferred_components use component-id strings, not objects. Use
storage.required_capabilities only for exact capability tokens provided by a
built-in or accepted custom component. A natural-language paper feature belongs
in a cited custom component, not in required_capabilities. Maturity must be one
of validated, supported, or experimental. Only list algorithms with a public
graph-analysis result that a query can return and a correctness test can assert;
do not list internal storage mechanisms such as update selection or degree
estimation as algorithms. Each custom component must be a JSON object with:
{"id":"paper.slug","dimension":"storage_layout","description":"...",
 "emits":{"layout":"..."},"provides":["disk_records"],
 "requires_capabilities":[],"requires_components":[],"conflicts":[],
 "compatible_with":{},"affinity":{"write":2.0,"traversal":2.0},
 "complexity":2.0,"risk":0.8,"maturity":"experimental",
 "evidence":"exact quote"}.
Use JSON objects for emits, compatible_with, and affinity. Use arrays of strings
for provides and dependencies. The example values are illustrative; use only
paper-supported content. Choose at most one required component in each exclusive
dimension (storage_layout, storage_update, recovery, execution). Coexisting
indexes, CSR adjacency layers, buffer managers, projections, and query operators belong
in augmentation; a disk-based base storage_layout must provide disk_records.
An execution component must emit model as volcano, vectorized, or push_based;
factorized vectors and ASP-Join operators are augmentations. Standard WAL is
already available as a built-in recovery option; do not rename it as a paper
component. Do not make a baseline, related-work system, or future roadmap
feature required for the paper's target system. compatible_with maps an
exclusive dimension name to an array of allowed component IDs; use
requires_components for a mandatory named dependency. Join and storage
operators are implementation components, not algorithms.required, even if a
paper calls them algorithms. Every requires_capabilities token must have a
provider among accepted built-in or paper components. If joins require
columnar_storage or csr_adjacency, define an evidence-cited paper storage
component that provides those tokens. If the paper explicitly allows one writer
with concurrent readers, propose a hard preference.concurrency_control fact
with value single_writer and the exact paper quote.""" if paper_mode else """
This is a business request. Leave custom_components, required_components, and
preferred_components empty. Extract needs, not implementation choices."""
    prompt = (
        JSON_SHAPE + "\n" + custom + "\n\n"
        + "Deterministic extraction already found:\n"
        + json.dumps(deterministic_summary or {}, ensure_ascii=False, indent=2)
        + "\n\nSOURCE TEXT:\n" + source_text
    )
    raw, response = call_chat_completion(config, system_prompt=SYSTEM_PROMPT, prompt=prompt)
    return validate_proposal(raw, source_text, allow_custom_components=paper_mode), response


def _dedupe_facts(facts: list[Fact]) -> list[Fact]:
    chosen: dict[tuple[str, str], Fact] = {}
    for fact in facts:
        key = (fact.field, json.dumps(fact.value, ensure_ascii=False, sort_keys=True))
        previous = chosen.get(key)
        rank = (fact.strength == "hard", fact.confidence, fact.weight)
        previous_rank = (
            previous.strength == "hard", previous.confidence, previous.weight
        ) if previous else None
        if previous is None or rank > previous_rank:
            chosen[key] = fact
    return list(chosen.values())


def merge_proposal(
    base: RequirementIR,
    proposal: dict[str, Any],
    *,
    model: str,
    response_text: str,
    source_kind: str,
) -> RequirementIR:
    """Merge extraction output, then re-run normal Requirement IR validation."""
    proposed_language = proposal.get("language")
    if proposed_language and proposed_language != base.language["name"]:
        raise RequirementError(
            f"LLM language {proposed_language} conflicts with selected {base.language['name']}"
        )
    raw = {
        "schema_version": 1,
        "candidate": dict(base.candidate),
        "language": dict(base.language),
        "facts": [asdict(fact) for fact in base.facts] + proposal["facts"],
        "schema": proposal["schema"] or dict(base.schema),
        "graph": proposal["graph"] or dict(base.graph),
        "algorithms": [
            {key: value for key, value in item.items() if key != "evidence"}
            for item in (proposal["algorithms"] or list(base.algorithms))
        ],
        "capabilities": proposal["capabilities"],
        "storage": proposal["storage"],
        "unknowns": list(dict.fromkeys(base.unknowns + proposal["unknowns"])),
        "source": {
            **base.source,
            "kind": source_kind,
            "model_api_used": True,
            "model": model,
            "proposal_sha256": hashlib.sha256(response_text.encode("utf-8")).hexdigest(),
        },
    }
    merged = normalize_requirements(raw)
    merged.facts = _dedupe_facts(merged.facts)
    return merged
