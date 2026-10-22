"""End-to-end deterministic design synthesis and evidence bundle creation."""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
from typing import Any

from .catalog import Catalog, load_catalog
from .emitter import compile_design
from .requirements import RequirementIR
from .solver import UnsatisfiableRequirements, clarification_candidates, solve


class SynthesisError(RuntimeError):
    pass


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + "\n").encode("utf-8")


def _report(ir: RequirementIR, selected: dict[str, Any], trace: dict[str, Any], questions: list[dict[str, Any]], catalog: Catalog) -> str:
    lines = [
        "# MetaGDB design decision report", "",
        "## Result", "",
        f"- Candidate: {ir.candidate['crate_name']}",
        f"- Language: {ir.language['name']}",
        f"- Feasible compositions: {trace['feasible']} / {trace['enumerated']}",
        f"- Pareto candidates: {trace['pareto_size']}",
        f"- Selected score: {selected['score']:.6f}", "",
        "## Selected component composition", "",
    ]
    for component_id in selected["components"]:
        component = catalog.by_id[component_id]
        lines.append(f"- {component.id} ({component.dimension}): {component.description}")
    lines.extend(["", "## Requirement evidence", ""])
    if ir.facts:
        for fact in ir.facts:
            lines.append(f"- {fact.field} = {fact.value} [{fact.strength}, confidence={fact.confidence:.2f}] — {fact.evidence or 'structured input'}")
    else:
        lines.append("- No explicit fact; documented research defaults and objective priors were used.")
    lines.extend(["", "## Assumptions", ""])
    for assumption in selected["assumptions"] or ["No implicit design assumption."]:
        lines.append(f"- {assumption}")
    lines.extend(["", "## Rejected or unresolved input", ""])
    if ir.unknowns:
        for unknown in ir.unknowns:
            lines.append(f"- {unknown}")
    else:
        lines.append("- None.")
    lines.extend(["", "## Highest-value unresolved questions", ""])
    if questions:
        for question in questions[:5]:
            lines.append(f"- **{question['prompt']}** ({question['dimension']}, value={question['value']:.3f})")
    else:
        lines.append("- The leading candidates do not differ on a question-bank dimension.")
    reproducibility = (
        "A model API proposed evidence-cited Requirement Facts. The final vector was still produced "
        "by deterministic catalog expansion, compatibility filtering, capability closure, Pareto "
        "analysis, and stable tie-breaking; the model did not emit or select the final design."
        if ir.source.get("model_api_used")
        else
        "The final vector was produced by deterministic catalog expansion, compatibility filtering, "
        "capability closure, Pareto analysis, and stable tie-breaking. No model API was called."
    )
    lines.extend(["", "## Reproducibility", "", reproducibility, ""])
    return "\n".join(lines)


def synthesize(ir: RequirementIR, output_dir: Path, *, top_k: int = 20) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=False)
    (output_dir / "requirements.json").write_bytes(_json_bytes(ir.as_dict()))
    catalog = load_catalog(ir.custom_components)
    try:
        ranked, trace = solve(ir, catalog)
    except UnsatisfiableRequirements as exc:
        conflict = {"status": "unsatisfiable", "reasons": exc.reasons}
        (output_dir / "decision_trace.json").write_bytes(_json_bytes(conflict))
        raise SynthesisError("requirements are unsatisfiable: " + "; ".join(exc.reasons)) from exc
    selected = ranked[0]
    questions = clarification_candidates(ranked, catalog)
    design = compile_design(ir, selected, catalog)
    candidate_rows = [candidate.as_dict() for candidate in ranked[:top_k]]
    selected_row = selected.as_dict()
    now = datetime.now(timezone.utc).replace(microsecond=0).isoformat()
    requirements = ir.as_dict()
    provenance = {
        "schema_version": 1,
        "created_at": now,
        "algorithm": "constraint-guided-gdbms-design-synthesis-v1",
        "model_api_used": bool(ir.source.get("model_api_used", False)),
        "model": ir.source.get("model"),
        "proposal_sha256": ir.source.get("proposal_sha256"),
        "catalog_sha256": hashlib.sha256((Path(__file__).resolve().parent / "catalog" / "components.json").read_bytes()).hexdigest(),
        "requirements_sha256": hashlib.sha256(_json_bytes(requirements)).hexdigest(),
        "selected_components": list(selected.components),
        "unresolved_count": len(ir.unknowns),
    }
    (output_dir / "candidates.json").write_bytes(_json_bytes({"schema_version": 1, "candidates": candidate_rows}))
    with (output_dir / "clarification.jsonl").open("w", encoding="utf-8") as handle:
        for question in questions:
            handle.write(json.dumps(question, ensure_ascii=False, sort_keys=True) + "\n")
    trace_out = dict(trace)
    trace_out.update({"status": "selected", "selected": selected_row})
    (output_dir / "decision_trace.json").write_bytes(_json_bytes(trace_out))
    (output_dir / "design_vector.json").write_bytes(_json_bytes(design))
    (output_dir / "decision_report.md").write_text(_report(ir, selected_row, trace, questions, catalog), encoding="utf-8")
    (output_dir / "provenance.json").write_bytes(_json_bytes(provenance))
    return {"design": design, "trace": trace_out, "questions": questions, "output_dir": str(output_dir)}
