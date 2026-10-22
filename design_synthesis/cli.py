"""Command line interface for constraint-guided requirement design synthesis."""

from __future__ import annotations

import argparse
from dataclasses import asdict
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys

from .engine import SynthesisError, synthesize
from .extractor import detect_languages, extract_text
from .catalog import load_catalog
from .direct import synthesize_direct
from .llm import LlmConfig, merge_proposal, propose_requirements
from .paper import evidence_window, extract_document_text
from .requirements import Fact, RequirementError, load_requirements, normalize_requirements
from .solver import clarification_candidates, solve


ROOT = Path(__file__).resolve().parent.parent
RUNS = ROOT / "design_runs"
LANGUAGES = ("Cypher", "GQL", "Gremlin")


def _run_dir(run_id: str | None) -> Path:
    value = run_id or datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", value):
        raise RequirementError("run id may contain only letters, digits, dot, underscore, and hyphen")
    return RUNS / value


def _publish(result: dict, path: str | None) -> None:
    if not path:
        return
    destination = Path(path)
    if not destination.is_absolute():
        destination = ROOT / destination
    destination.parent.mkdir(parents=True, exist_ok=True)
    source = Path(result["output_dir"]) / "design_vector.json"
    shutil.copyfile(source, destination)
    print(f"Published finalized design vector: {destination}")


def _backend_check(result: dict) -> None:
    path = Path(result["output_dir"]) / "design_vector.json"
    completed = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "synthesis.py"), "check", "--design", str(path)],
        cwd=ROOT,
    )
    if completed.returncode:
        raise SynthesisError("generated vector failed the existing synthesis backend check")


def _execute(
    ir,
    args,
    *,
    raw_text: str | None = None,
    llm_proposal: dict | None = None,
) -> int:
    output_dir = _run_dir(args.run_id)
    try:
        result = synthesize(ir, output_dir, top_k=args.top_k)
    except SynthesisError:
        if output_dir.is_dir():
            if raw_text is not None:
                (output_dir / "raw_input.txt").write_text(raw_text, encoding="utf-8")
            if llm_proposal is not None:
                (output_dir / "llm_proposal.json").write_text(
                    json.dumps(llm_proposal, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
                    encoding="utf-8",
                )
        raise
    if raw_text is not None:
        (output_dir / "raw_input.txt").write_text(raw_text, encoding="utf-8")
    if llm_proposal is not None:
        (output_dir / "llm_proposal.json").write_text(
            json.dumps(llm_proposal, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        rejected_required = [
            item for item in llm_proposal.get("rejected", [])
            if str(item.get("item", "")).startswith("required component ")
        ]
        if rejected_required:
            (output_dir / "design_vector.json").rename(output_dir / "diagnostic_vector.json")
            raise SynthesisError(
                f"{len(rejected_required)} paper-required components were rejected; "
                f"no finalized vector published. Review {output_dir / 'llm_proposal.json'} "
                "and the decision report"
            )
    _backend_check(result)
    _publish(result, args.output)
    questions = result["questions"]
    print(f"Design run: {output_dir.relative_to(ROOT)}")
    print(f"Selected components: {', '.join(result['trace']['selected']['components'])}")
    print(f"Feasible candidates: {result['trace']['feasible']}")
    if ir.unknowns:
        detail_files = (
            "decision_report.md and llm_proposal.json"
            if llm_proposal is not None
            else "decision_report.md"
        )
        print(
            f"Rejected or unresolved inputs: {len(ir.unknowns)} "
            f"(see {detail_files})"
        )
    if questions:
        print(f"Highest-value optional clarification: {questions[0]['prompt']}")
    print(f"Decision report: {(output_dir / 'decision_report.md').relative_to(ROOT)}")
    return 0


def cmd_compile(args) -> int:
    return _execute(load_requirements(Path(args.requirements)), args)


def _text_ir(text: str, args, *, paper_mode: bool):
    detected = sorted(detect_languages(text))
    selected = args.language
    deterministic_summary: dict[str, object] = {"detected_languages": detected}
    if selected or len(detected) == 1:
        preview = extract_text(
            text,
            crate_name=args.crate,
            description=args.description,
            language=selected,
        )
        deterministic_summary["facts"] = [asdict(fact) for fact in preview.facts]
        selected = preview.language["name"]

    if not args.llm_fallback:
        return extract_text(
            text,
            crate_name=args.crate,
            description=args.description,
            language=selected,
        ), None

    config = LlmConfig.from_env(ROOT, args.env_file)
    proposal, response = propose_requirements(
        text,
        config,
        paper_mode=paper_mode,
        deterministic_summary=deterministic_summary,
    )
    selected = selected or proposal.get("language")
    base = extract_text(
        text,
        crate_name=args.crate,
        description=args.description,
        language=selected,
    )
    if paper_mode:
        # Whole papers discuss baselines and rejected alternatives. Lexical
        # matches are useful prompt hints but must not become target-system
        # constraints unless the evidence-aware paper extractor proposes them.
        base.facts = []
        base.source["paper_lexical_facts_used"] = False
    merged = merge_proposal(
        base,
        proposal,
        model=config.model,
        response_text=response,
        source_kind="paper+llm" if paper_mode else "text+llm",
    )
    return merged, proposal


def cmd_from_text(args) -> int:
    path = Path(args.text)
    text = path.read_text(encoding="utf-8")
    if args.mode == "llm-direct":
        return _execute_direct(text, args, paper_mode=False)
    if args.resume:
        raise RequirementError("--resume is only available with --mode llm-direct")
    ir, proposal = _text_ir(text, args, paper_mode=False)
    ir.source["input_path"] = str(path)
    return _execute(ir, args, raw_text=text, llm_proposal=proposal)


def cmd_from_paper(args) -> int:
    path = Path(args.paper)
    full_text = extract_document_text(path)
    excerpt = evidence_window(full_text, max_chars=args.max_chars)
    if args.mode == "llm-direct":
        return _execute_direct(excerpt, args, paper_mode=True)
    if args.resume:
        raise RequirementError("--resume is only available with --mode llm-direct")
    ir, proposal = _text_ir(excerpt, args, paper_mode=True)
    ir.source.update({
        "input_path": str(path),
        "full_text_sha256": hashlib.sha256(full_text.encode("utf-8")).hexdigest(),
        "evidence_window_sha256": hashlib.sha256(excerpt.encode("utf-8")).hexdigest(),
        "evidence_window_chars": len(excerpt),
    })
    return _execute(ir, args, raw_text=excerpt, llm_proposal=proposal)


def _execute_direct(text: str, args, *, paper_mode: bool) -> int:
    config = LlmConfig.from_env(ROOT, args.env_file)
    output_dir = _run_dir(args.run_id)
    result = synthesize_direct(
        text, output_dir, crate=args.crate, language=args.language,
        description=args.description, paper_mode=paper_mode, config=config,
        max_repair_rounds=args.max_repair_rounds, resume=args.resume,
    )
    _backend_check(result)
    _publish(result, args.output)
    print(f"LLM direct design run: {output_dir.relative_to(ROOT)}")
    print(f"Validated attempts: {len(result['attempts'])}")
    print(f"Decision report: {(output_dir / 'decision_report.md').relative_to(ROOT)}")
    return 0


def _choose(prompt: str, options: list[str], *, default: int = 0) -> str:
    print(prompt)
    for index, value in enumerate(options, 1):
        print(f"  {index}. {value}")
    while True:
        answer = input(f"Choice [{default + 1}]: ").strip()
        if not answer:
            return options[default]
        if answer.isdigit() and 1 <= int(answer) <= len(options):
            return options[int(answer) - 1]
        print("Enter one listed number.")


def _answer_fact(dimension: str, value: str) -> Fact:
    mapping = {
        "schema": ("schema.mode", value),
        "isolation": ("correctness.isolation_level", value),
        "execution": ("preference.execution_model", value),
        "storage_layout": ("preference.storage_layout", f"layout.{value}"),
        "recovery": ("preference.recovery", f"recovery.{value}"),
    }
    field, normalized = mapping[dimension]
    return Fact(field, normalized, "hard", evidence="active clarification answer", source="guided")


def cmd_guided(args) -> int:
    language = _choose("Which graph language is required?", list(LANGUAGES))
    topology = _choose(
        "Which graph topology matters most?",
        ["power_law_social", "sparse_railway", "uniform_regular"],
        default=0,
    )
    focus = _choose(
        "Which workload is the primary optimization target?",
        ["point_read", "traversal", "scan", "write", "analytics"],
        default=1,
    )
    raw = {
        "schema_version": 1,
        "candidate": {"crate_name": args.crate, "description": args.description},
        "language": language,
        "graph": {"topology": topology},
        "workload": {focus: "critical", "recovery": "high"},
        "algorithms": [],
        "source": {"kind": "guided"},
    }
    ir = normalize_requirements(raw)
    asked = 0
    while asked < args.max_questions:
        catalog = load_catalog(ir.custom_components)
        ranked, _ = solve(ir, catalog)
        if len(ranked) < 2 or ranked[0].score - ranked[1].score >= args.stop_margin:
            break
        questions = clarification_candidates(ranked, catalog)
        if not questions:
            break
        question = questions[0]
        keys = list(question["options"])
        labels = [f"{key}: {question['options'][key]}" for key in keys]
        selected = _choose(question["prompt"], labels)
        value = keys[labels.index(selected)]
        ir.facts.append(_answer_fact(question["dimension"], value))
        asked += 1

    ranked, _ = solve(ir, load_catalog(ir.custom_components))
    selected_schema = next(
        component for component in ranked[0].components if component.startswith("schema.")
    )
    if selected_schema == "schema.strict":
        print("Strict schema needs concrete declarations; comma-separated names are accepted.")
        while not ir.schema.get("vertex_labels"):
            ir.schema["vertex_labels"] = [
                item.strip() for item in input("Vertex labels: ").split(",") if item.strip()
            ]
        while not ir.schema.get("edge_types"):
            ir.schema["edge_types"] = [
                item.strip() for item in input("Edge types: ").split(",") if item.strip()
            ]
        ir.schema["property_keys"] = [
            item.strip() for item in input("Property keys (optional): ").split(",") if item.strip()
        ]
    algorithm_names = [
        item.strip()
        for item in input("Required algorithms (comma-separated, optional): ").split(",")
        if item.strip()
    ]
    ir.algorithms = [
        {"name": name, "description": f"Implement the user-requested {name} algorithm"}
        for name in algorithm_names
    ]
    ir.source.update({"kind": "guided", "clarifications_answered": asked})
    return _execute(ir, args)


def add_common(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--run-id", help="stable name under design_runs/; timestamp by default")
    parser.add_argument("--output", help="also copy the finalized vector here, for example design_vector.json")
    parser.add_argument("--top-k", type=int, default=20, help="number of ranked candidates retained as evidence")


def add_text_options(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--crate", required=True)
    parser.add_argument("--description", default="")
    parser.add_argument("--language", choices=LANGUAGES)
    parser.add_argument(
        "--mode", choices=("constraint", "llm-direct"), default="constraint",
        help="constraint solver (default) or full LLM design with validation and repair",
    )
    parser.add_argument(
        "--max-repair-rounds", type=int, default=3,
        help="bounded LLM repair rounds for llm-direct mode (1-10; plus draft and review)",
    )
    parser.add_argument(
        "--resume", action="store_true",
        help="continue a failed llm-direct run with matching source and model",
    )
    parser.add_argument(
        "--llm-fallback",
        action="store_true",
        help="use an evidence-constrained OpenAI-compatible extractor before deterministic solving",
    )
    parser.add_argument(
        "--env-file",
        default=".env",
        help="dotenv file containing LLM_API_KEY/base/model; default: .env",
    )
    add_common(parser)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="MetaGDB design synthesis: constraint solver or LLM direct generation and repair"
    )
    sub = parser.add_subparsers(dest="command", required=True)

    compile_parser = sub.add_parser("compile", help="synthesize from structured requirements JSON")
    compile_parser.add_argument("requirements")
    add_common(compile_parser)
    compile_parser.set_defaults(func=cmd_compile)

    text_parser = sub.add_parser(
        "from-text",
        help="extract common requirement facts from UTF-8 text, then synthesize",
    )
    text_parser.add_argument("text")
    add_text_options(text_parser)
    text_parser.set_defaults(func=cmd_from_text)

    paper_parser = sub.add_parser(
        "from-paper",
        help="derive requirements from a PDF or text paper evidence window",
    )
    paper_parser.add_argument("paper")
    paper_parser.add_argument(
        "--max-chars",
        type=int,
        default=60000,
        help="maximum design-bearing paper text sent to extraction",
    )
    add_text_options(paper_parser)
    paper_parser.set_defaults(func=cmd_from_paper)

    guided = sub.add_parser(
        "guided",
        help="collect a small business-level questionnaire, then synthesize",
    )
    guided.add_argument("--crate", required=True)
    guided.add_argument("--description", default="")
    guided.add_argument("--max-questions", type=int, default=5)
    guided.add_argument("--stop-margin", type=float, default=0.75)
    add_common(guided)
    guided.set_defaults(func=cmd_guided)
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        return args.func(args)
    except (RequirementError, SynthesisError, OSError, ValueError) as exc:
        print(f"Design synthesis failed: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
