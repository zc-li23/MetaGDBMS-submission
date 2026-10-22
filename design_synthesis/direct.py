"""LLM-owned design-vector generation with bounded validation and revision."""

from __future__ import annotations

from dataclasses import replace
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
from typing import Any, Callable

from scripts.synthesis import DesignError, ENUMS, validate_design

from .engine import SynthesisError
from .llm import InvalidLlmJson, LlmConfig, _evidence_present, call_chat_completion


ROOT = Path(__file__).resolve().parent.parent
TEMPLATE = ROOT / "docs" / "synthesis" / "templates" / "design_vector.example.json"
SYSTEM_PROMPT = (
    "You are designing a faithful, runnable MetaGDB research reproduction. "
    "Return one JSON object with design_vector, evidence, and assumptions. "
    "You own the complete design vector. Never invoke or mimic a component catalog solver. "
    "Preserve the fixed framework, benchmark, grammar, ACID and quality gates. "
    "Do not invent evidence or pretend a paper implements features it does not describe. "
    "Query execution operators belong in storage.layout or capabilities.required. "
    "algorithms.required is for standalone graph algorithms such as BFS, "
    "PageRank, WCC, or a specifically requested algorithm."
)


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + "\n").encode("utf-8")


def _sha256(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def _validate_envelope(
    envelope: dict[str, Any], candidate_path: Path, source_text: str,
    *, crate: str, language: str | None, paper_mode: bool,
) -> list[str]:
    errors: list[str] = []
    vector = envelope.get("design_vector")
    if not isinstance(vector, dict):
        return ["response.design_vector must be a complete JSON object"]
    candidate_path.write_bytes(_json_bytes(vector))
    try:
        validate_design(candidate_path)
    except (DesignError, TypeError, AttributeError) as exc:
        errors.append(f"backend design validator: {exc}")
    for field, choices in ENUMS.items():
        value = _field_value(vector, field)
        if value is not None and (not isinstance(value, str) or value not in choices):
            errors.append(f"{field} must be exactly one of: {', '.join(sorted(choices))}; describe paper-specific details in storage.layout or capabilities.required")
    algorithms = vector.get("algorithms")
    required_algorithms = algorithms.get("required") if isinstance(algorithms, dict) else None
    performance = vector.get("performance")
    workload = performance.get("workload") if isinstance(performance, dict) else None
    if required_algorithms == [] and isinstance(workload, dict):
        for field in ("algorithm_kernel", "algorithm_e2e", "algorithm_projection_build"):
            if workload.get(field, 0) != 0:
                errors.append(f"performance.workload.{field} must be zero when algorithms.required is empty; redistribute its weight and keep sum 1.0")
    if isinstance(workload, dict) and all(
        isinstance(value, (int, float)) and not isinstance(value, bool)
        for value in workload.values()
    ) and abs(sum(workload.values()) - 1.0) > 1e-6:
        errors.append(f"performance.workload sums to {sum(workload.values()):.6f}; change a non-algorithm weight so it sums to 1.0")
    candidate = vector.get("candidate")
    selected_language = vector.get("language")
    if not isinstance(candidate, dict) or candidate.get("crate_name") != crate:
        errors.append(f"candidate.crate_name must equal {crate!r}")
    if language and (not isinstance(selected_language, dict) or selected_language.get("name") != language):
        errors.append(f"language.name must equal {language!r}")
    if paper_mode:
        for item in required_algorithms if isinstance(required_algorithms, list) else []:
            name = item.get("name", "") if isinstance(item, dict) else item
            if isinstance(name, str) and "join" in name.casefold():
                errors.append(f"{name!r} is a query join operator, not an analytical graph algorithm")
    evidence = envelope.get("evidence")
    assumptions = envelope.get("assumptions")
    if not isinstance(evidence, list):
        errors.append("evidence must be an array of {field, quote} objects")
        evidence = []
    if not isinstance(assumptions, list):
        errors.append("assumptions must be an array of {field, reason} objects")
        assumptions = []
    fields_with_evidence: set[str] = set()
    fields_with_assumption: set[str] = set()
    for index, item in enumerate(evidence):
        if not isinstance(item, dict) or not isinstance(item.get("field"), str) or not isinstance(item.get("quote"), str):
            errors.append(f"evidence[{index}] must have string field and quote")
            continue
        field, quote = item["field"], item["quote"]
        if not _field_exists(vector, field):
            errors.append(f"evidence[{index}].field {field!r} is not a design-vector field")
        elif not _evidence_present(quote, source_text):
            errors.append(f"evidence[{index}].quote is absent from supplied source: {quote[:100]!r}")
        else:
            fields_with_evidence.add(field)
    for index, item in enumerate(assumptions):
        if not isinstance(item, dict) or not isinstance(item.get("field"), str) or not isinstance(item.get("reason"), str) or not item["reason"].strip():
            errors.append(f"assumptions[{index}] must have string field and nonempty reason")
            continue
        if not _field_exists(vector, item["field"]):
            errors.append(f"assumptions[{index}].field {item['field']!r} is not a design-vector field")
        else:
            fields_with_assumption.add(item["field"])
    if paper_mode:
        for field in ("storage.layout", "storage.recovery", "transactions.isolation_level", "execution.model", "capabilities.required"):
            if field not in fields_with_evidence | fields_with_assumption:
                errors.append(f"{field} needs a paper quote or an explicit assumption")
        if "storage.layout" not in fields_with_evidence:
            errors.append("paper reproduction requires source evidence for storage.layout")
        capabilities = vector.get("capabilities")
        required_caps = capabilities.get("required", []) if isinstance(capabilities, dict) else []
        descriptions = " ".join(required_caps) if isinstance(required_caps, list) and all(isinstance(x, str) for x in required_caps) else ""
        schema = vector.get("schema")
        schema_mode = schema.get("mode") if isinstance(schema, dict) else None
        mixed_schema_in_paper = bool(
            re.search(r"node and edge relations with pre-defined", source_text, re.I)
            and re.search(r"semi-structured properties", source_text, re.I)
        )
        if mixed_schema_in_paper and schema_mode != "hybrid":
            errors.append("schema.mode must be hybrid: the paper combines DDL-defined node/edge relations with semi-structured properties")
        if mixed_schema_in_paper and "ddl" not in descriptions.casefold():
            errors.append("capabilities.required must include paper-defined DDL for node and edge relations")
        if schema_mode == "schemaless" and re.search(r"\b(?:DDL|pre-defined|defined schema)\b", descriptions, re.I):
            errors.append("schema.mode=schemaless conflicts with required DDL/pre-defined relation schema; use hybrid when structured declarations coexist with semi-structured properties")
        storage = vector.get("storage")
        recovery = storage.get("recovery", "") if isinstance(storage, dict) else ""
        if isinstance(recovery, str) and re.search(
            r"\bcommitted\b[^;.!?]{0,60}(?:rolled\s+back|roll\s*back|rollback)",
            recovery, re.I,
        ):
            errors.append("storage.recovery must not claim committed updates can be rolled back; rollback applies to uncommitted transactions")
        quality = vector.get("quality")
        tests = quality.get("required_tests", []) if isinstance(quality, dict) else []
        test_text = " ".join(tests).casefold() if isinstance(tests, list) and all(isinstance(x, str) for x in tests) else ""
        if "asp-join" not in source_text.casefold() and isinstance(tests, list):
            unrelated = [
                item for item in tests
                if isinstance(item, str) and re.search(r"\b(?:asp-join|s-join|multiway join)\b", item, re.I)
            ]
            if unrelated:
                errors.append(
                    "quality.required_tests includes join operators absent from this paper: "
                    + "; ".join(unrelated)
                )
        if "join" in descriptions.casefold() and "join" not in test_text:
            errors.append("quality.required_tests needs direct and public-language join correctness tests for required join operators")
        if "factorized" in descriptions.casefold() and "factorized" not in test_text:
            errors.append("quality.required_tests needs a factorized-intermediate correctness test")
        required_operators: list[tuple[str, tuple[str, ...]]] = []
        if "binary asp-join" in descriptions.casefold():
            required_operators.append(("binary ASP-Join", ("binary", "asp-join")))
        if "s-join" in descriptions.casefold():
            required_operators.append(("S-Join", ("s-join",)))
        if "multiway" in descriptions.casefold() and "join" in descriptions.casefold():
            required_operators.append(("multiway join", ("multiway", "join")))
        if isinstance(tests, list):
            for operator, operator_tokens in required_operators:
                for kind, kind_token in (("direct", "direct"), ("public-language", "public")):
                    if not any(
                        isinstance(item, str)
                        and kind_token in item.casefold()
                        and all(token in item.casefold() for token in operator_tokens)
                        for item in tests
                    ):
                        errors.append(
                            f"quality.required_tests needs a {kind} {operator} correctness test"
                        )
        join_focused_paper = (
            "factorized query processor" in source_text.casefold()
            and "asp-join" in descriptions.casefold()
        )
        if join_focused_paper and isinstance(performance, dict):
            if performance.get("profile") == "point_read_optimized":
                errors.append("performance.profile must reflect the paper's factorized join workload; use traversal_optimized or scan_analytics instead of point_read_optimized")
            primary = performance.get("primary_metrics", [])
            if isinstance(primary, list) and not set(primary) & {
                "query.one_hop.p95", "storage.out_adjacency.p95",
                "storage.hotspot_out_adjacency.p95",
            }:
                errors.append("performance.primary_metrics needs a query or adjacency metric for the paper's join-focused workload")
            if isinstance(workload, dict) and all(
                isinstance(workload.get(name, 0), (int, float))
                and not isinstance(workload.get(name, 0), bool)
                for name in (
                    "query_one_hop", "storage_out_adjacency", "storage_hotspot_out_adjacency",
                    "query_point_read", "storage_vertex_point_read", "storage_edge_point_read",
                )
            ):
                join_proxy = sum(float(workload.get(name, 0)) for name in (
                    "query_one_hop", "storage_out_adjacency", "storage_hotspot_out_adjacency",
                ))
                point_proxy = sum(float(workload.get(name, 0)) for name in (
                    "query_point_read", "storage_vertex_point_read", "storage_edge_point_read",
                ))
                if join_proxy <= point_proxy:
                    errors.append(
                        f"join-focused workload proxy weight {join_proxy:.3f} must exceed point-read proxy weight {point_proxy:.3f}; increase one-hop and adjacency weights while keeping sum 1.0"
                    )
        poly_lsm_paper = (
            "poly-lsm" in source_text.casefold()
            and "skewness of graph" in source_text.casefold()
            and "encode" in source_text.casefold()
        )
        if poly_lsm_paper:
            layout = storage.get("layout", "") if isinstance(storage, dict) else ""
            architecture = " ".join((layout, descriptions)).casefold()
            if "ddl" not in source_text.casefold():
                if "ddl" in descriptions.casefold():
                    errors.append(
                        "Poly-LSM source does not describe DDL-defined node/edge relations; remove the unsupported DDL requirement"
                    )
                if schema_mode == "hybrid":
                    errors.append(
                        "schema.mode=hybrid is not supported by the Poly-LSM source without structured DDL evidence; use schemaless or cite a source passage"
                    )
            if not ("skew" in architecture and "encod" in architecture):
                errors.append(
                    "Poly-LSM paper requires skew-aware entry encoding in storage.layout or capabilities.required"
                )
            if "partitioned elias-fano" in source_text.casefold() and "partitioned elias-fano" not in architecture:
                errors.append(
                    "Poly-LSM paper specifies partitioned Elias-Fano entry encoding; include it in storage.layout or capabilities.required"
                )
            if "encod" not in test_text and "elias-fano" not in test_text:
                errors.append(
                    "quality.required_tests needs an entry-encoding round-trip correctness test for Poly-LSM"
                )
            for term, description in (
                ("pivot", "pivot edge update"),
                ("delta", "delta edge update"),
                ("adaptive", "adaptive update selection"),
                ("degree sketch", "degree sketch approximation"),
            ):
                if term not in test_text:
                    errors.append(f"quality.required_tests needs {description} correctness test for Poly-LSM")
            if re.search(r"degree[- ]sketch.{0,30}partition", architecture):
                errors.append(
                    "Poly-LSM degree sketch selects pivot/delta updates; partitioned Elias-Fano encodes neighbor IDs by edge locality, not degree-sketch-guided partitioning"
                )
            if "partitioned elias-fano" in source_text.casefold() and isinstance(workload, dict) and workload.get("storage_disk_size") == 0:
                errors.append(
                    "performance.workload.storage_disk_size needs nonzero weight to assess the paper's entry-compression mechanism; rebalance weights to sum 1.0"
                )
            if re.search(r"\bwe\s+test\s+five\s+algorithms\b", source_text, re.I) and isinstance(required_algorithms, list):
                names = {
                    item.get("name", "").casefold() if isinstance(item, dict) else item.casefold()
                    for item in required_algorithms if isinstance(item, (dict, str))
                }
                for name, aliases in (
                    ("bfs", {"bfs"}), ("pagerank", {"pagerank"}),
                    ("wcc", {"wcc"}), ("sssp", {"sssp"}),
                    ("cdlp", {"cdlp", "label_propagation"}),
                ):
                    if not names & aliases:
                        errors.append(
                            f"algorithms.required is missing {name} from the paper's five LDBC Graphalytics evaluation algorithms"
                        )
    return errors


def _field_exists(value: dict[str, Any], field: str) -> bool:
    current: Any = value
    for part in field.split("."):
        if not isinstance(current, dict) or part not in current:
            return False
        current = current[part]
    return True


def _field_value(value: dict[str, Any], field: str) -> Any:
    current: Any = value
    for part in field.split("."):
        if not isinstance(current, dict) or part not in current:
            return None
        current = current[part]
    return current


def _prompt(
    source_text: str, *, crate: str, language: str | None, description: str,
    paper_mode: bool, prior: dict[str, Any] | None, errors: list[str], review: bool,
) -> str:
    join_focused_source = "asp-join" in source_text.casefold()
    if prior is not None and errors:
        parts = [
            "Repair only the listed errors in the previous response. Return the COMPLETE "
            "JSON object with design_vector, evidence, and assumptions. Keep all valid "
            "paper architecture and source quotes unchanged. Do not copy an invalid "
            "numeric field unchanged. Calculate every weight sum before returning.",
            "Validation errors:\n" + "\n".join(f"- {error}" for error in errors),
        ]
        if any("workload" in error and "sum" in error for error in errors):
            vector = prior.get("design_vector", {})
            performance = vector.get("performance", {}) if isinstance(vector, dict) else {}
            workload = performance.get("workload", {}) if isinstance(performance, dict) else {}
            if isinstance(workload, dict) and all(
                isinstance(value, (int, float)) and not isinstance(value, bool)
                for value in workload.values()
            ):
                excess = sum(workload.values()) - 1.0
                for key in ("query_point_read", "storage_vertex_point_read", "query_one_hop"):
                    current = workload.get(key)
                    if isinstance(current, (int, float)) and 0 <= current - excess <= 1:
                        parts.append(
                            f"Exact arithmetic repair: workload currently sums to "
                            f"{1.0 + excess:.6f}. Set performance.workload.{key} "
                            f"from {current:.6f} to {current - excess:.6f}; keep the "
                            "other workload numbers unchanged. Return a complete JSON object."
                        )
                        break
        parts.append("Previous response:\n" + json.dumps(prior, ensure_ascii=False))
        if any("quote" in error or "evidence" in error or "paper reproduction" in error for error in errors):
            parts.append("Source text for quote repair:\n" + source_text)
        return "\n\n".join(parts)
    if prior is not None and review:
        review_focus = (
            "paper-specific storage layers, factorized query representations, join "
            "operators, transaction semantics, and fabricated analytical algorithms"
            if join_focused_source else
            "paper-specific storage and query mechanisms, transaction semantics, "
            "missing evaluated algorithms, and fabricated capabilities"
        )
        return "\n\n".join([
            "Independently review this complete design against the paper. Find missing "
            f"{review_focus}. "
            "Return the corrected COMPLETE JSON object with design_vector, evidence, "
            "and assumptions. Recalculate performance weight sums. Keep the backend "
            "enum values and fixed gates valid.",
            "Previous response:\n" + json.dumps(prior, ensure_ascii=False),
            "Source text:\n" + source_text,
        ])
    example = json.loads(TEMPLATE.read_text(encoding="utf-8"))
    example["candidate"]["crate_name"] = crate
    example["candidate"]["description"] = description or f"Research reproduction {crate}"
    if language:
        example["language"] = {"name": language, "grammar_path": f"data/language/{language}.g4"}
    parts = [
        "Produce a complete design vector in the exact backend schema shown below. "
        "The example is a schema/guardrail reference, not evidence about the target system. "
        "Keep all required keys and valid enum values. Keep performance weights normalized, "
        "minimum benchmark pass rate >= 0.9, ACID and fixed boundaries. "
        "transactions.concurrency_control must be one enum value, e.g. "
        "single_writer (never single_writer_multiple_readers). execution.model "
        "must be exactly volcano, vectorized, or push_based. If "
        "algorithms.required is empty, set all three algorithm workload weights "
        "to zero and redistribute their weight so workload still sums to 1. "
        "If DDL-defined node and edge relations coexist with semi-structured "
        "properties, choose schema.mode=hybrid and keep DDL in required capabilities. "
        "If the paper explicitly evaluates a numbered set of graph algorithms, "
        "include the complete set rather than a subset in algorithms.required. "
        "Do not copy operators or tests from unrelated systems into this paper's vector. "
        "Use paper-specific storage and execution descriptions without reducing them to "
        "generic defaults. Include every central architecture mechanism in storage.layout, "
        "storage.indexes, or capabilities.required. The vector must be implementable "
        "through the selected fixed grammar and public framework. "
        "If the source does not specify a detail, make a reasoned choice and list it as "
        "an assumption; do not claim it is in the paper.",
        f"Target crate: {crate}. Target language: {language or 'infer one supported language from the input'}. "
        f"Input type: {'paper' if paper_mode else 'requirements'}. Description hint: {description!r}.",
        "Response shape: {\"design_vector\": <complete object>, \"evidence\": "
        "[{\"field\": \"storage.layout\", \"quote\": \"exact source excerpt\"}], "
        "\"assumptions\": [{\"field\": \"schema.mode\", \"reason\": \"source does not specify ...\"}]}.",
        "Evidence quotes must occur literally in the supplied text (minor PDF line-break "
        "normalization is accepted). Cite storage.layout with a paper quote. For "
        "storage.recovery, transactions.isolation_level, execution.model, and "
        "capabilities.required, give a quote or an explicit assumption. Do not invent "
        "schema labels from an example query. Do not put joins in algorithms.required.",
        "Backend schema example:\n" + json.dumps(example, ensure_ascii=False),
        "Source text:\n" + source_text,
    ]
    if join_focused_source:
        parts.insert(1,
            "Put factorization and join details in capabilities.required or "
            "storage.layout. For every required binary ASP-Join, S-Join, and "
            "multiway join operator, include its own direct and public-language "
            "correctness tests."
        )
    if errors:
        parts.append("Previous generation failed; regenerate complete JSON. Errors:\n" + "\n".join(errors))
    return "\n\n".join(parts)


def synthesize_direct(
    source_text: str, output_dir: Path, *, crate: str, language: str | None,
    description: str, paper_mode: bool, config: LlmConfig,
    max_repair_rounds: int = 3,
    resume: bool = False,
    completion: Callable[..., tuple[dict[str, Any], str]] = call_chat_completion,
) -> dict[str, Any]:
    if max_repair_rounds < 1 or max_repair_rounds > 10:
        raise SynthesisError("max_repair_rounds must be from 1 to 10")
    if config.max_tokens is None:
        config = replace(config, max_tokens=16384)
    prior: dict[str, Any] | None = None
    errors: list[str] = []
    reviewed = False
    attempts: list[dict[str, Any]] = []
    if resume:
        provenance_path = output_dir / "provenance.json"
        if not provenance_path.is_file():
            raise SynthesisError(f"cannot resume: missing {provenance_path}")
        provenance = json.loads(provenance_path.read_text(encoding="utf-8"))
        if (
            provenance.get("mode") != "llm-direct"
            or provenance.get("status") != "failed"
            or provenance.get("source_sha256") != _sha256(source_text)
            or provenance.get("model") != config.model
        ):
            raise SynthesisError("cannot resume: mode, status, source, or model differs from the failed run")
        attempts = provenance.get("attempts", [])
        if not isinstance(attempts, list):
            raise SynthesisError("cannot resume: malformed attempt history")
        for index in range(len(attempts), 0, -1):
            response_path = output_dir / f"attempt_{index:02d}_response.json"
            if response_path.is_file():
                prior = json.loads(response_path.read_text(encoding="utf-8"))
                break
        if attempts:
            errors = list(attempts[-1].get("errors", []))
        if prior is not None:
            errors = _validate_envelope(
                prior, output_dir / f"attempt_{len(attempts):02d}_vector.json",
                source_text, crate=crate, language=language, paper_mode=paper_mode,
            )
    else:
        output_dir.mkdir(parents=True, exist_ok=False)
        (output_dir / "raw_input.txt").write_text(source_text, encoding="utf-8")
    start_index = len(attempts)
    for index in range(start_index, start_index + max_repair_rounds + 2):
        review = prior is not None and not errors and not reviewed
        prompt = _prompt(
            source_text, crate=crate, language=language, description=description,
            paper_mode=paper_mode, prior=prior, errors=errors, review=review,
        )
        try:
            proposal, response = completion(config, system_prompt=SYSTEM_PROMPT, prompt=prompt)
        except InvalidLlmJson as exc:
            (output_dir / f"attempt_{index + 1:02d}_response.txt").write_text(
                exc.response_text, encoding="utf-8",
            )
            errors = [
                "LLM returned invalid JSON; regenerate a complete JSON object "
                f"(finish_reason={getattr(exc, 'finish_reason', 'unknown')})"
            ]
            (output_dir / f"attempt_{index + 1:02d}_feedback.json").write_bytes(_json_bytes({
                "review": review, "errors": errors, "valid": False,
            }))
            attempts.append({
                "number": index + 1, "review": review, "valid": False,
                "errors": errors, "prompt_sha256": _sha256(prompt),
                "response_sha256": _sha256(exc.response_text),
            })
            continue
        except Exception as exc:
            _write_provenance(output_dir, config, source_text, attempts, status="failed", error=str(exc))
            raise
        candidate = output_dir / f"attempt_{index + 1:02d}_vector.json"
        errors = _validate_envelope(
            proposal, candidate, source_text, crate=crate, language=language,
            paper_mode=paper_mode,
        )
        (output_dir / f"attempt_{index + 1:02d}_response.json").write_bytes(_json_bytes(proposal))
        (output_dir / f"attempt_{index + 1:02d}_feedback.json").write_bytes(_json_bytes({
            "review": review, "errors": errors, "valid": not errors,
        }))
        attempts.append({
            "number": index + 1, "review": review, "valid": not errors,
            "errors": errors, "prompt_sha256": _sha256(prompt),
            "response_sha256": _sha256(response),
        })
        prior = proposal
        if review:
            reviewed = True
        if not errors and reviewed:
            (output_dir / "design_vector.json").write_bytes(_json_bytes(proposal["design_vector"]))
            _write_report(output_dir, proposal, attempts)
            _write_provenance(output_dir, config, source_text, attempts, status="accepted")
            return {"design": proposal["design_vector"], "output_dir": str(output_dir), "attempts": attempts}
    _write_provenance(output_dir, config, source_text, attempts, status="failed", error="validation exhausted")
    raise SynthesisError(
        f"LLM direct design failed after {len(attempts)} attempts; "
        f"see {output_dir / f'attempt_{len(attempts):02d}_feedback.json'}"
    )


def _write_provenance(
    output_dir: Path, config: LlmConfig, source_text: str,
    attempts: list[dict[str, Any]], *, status: str, error: str | None = None,
) -> None:
    (output_dir / "provenance.json").write_bytes(_json_bytes({
        "schema_version": 1,
        "created_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat(),
        "mode": "llm-direct", "model_api_used": True, "model": config.model,
        "source_sha256": _sha256(source_text), "status": status,
        "attempts": attempts, "error": error,
    }))


def _write_report(output_dir: Path, proposal: dict[str, Any], attempts: list[dict[str, Any]]) -> None:
    design = proposal["design_vector"]
    lines = [
        "# LLM direct design decision report", "",
        f"- Candidate: {design['candidate']['crate_name']}",
        f"- Language: {design['language']['name']}",
        f"- Model attempts: {len(attempts)} (includes independent self-review)",
        "- Backend design schema: passed", "",
        "## Source evidence", "",
    ]
    lines += [f"- {item['field']}: {item['quote']}" for item in proposal["evidence"]]
    lines += ["", "## Assumptions", ""]
    lines += [f"- {item['field']}: {item['reason']}" for item in proposal["assumptions"]]
    lines += ["", "## Limit", "", "Schema validation and source-quote matching do not prove complete paper fidelity. Review the design against the paper before starting code synthesis.", ""]
    (output_dir / "decision_report.md").write_text("\n".join(lines), encoding="utf-8")
