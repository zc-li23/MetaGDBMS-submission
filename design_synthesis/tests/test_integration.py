from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest
from types import SimpleNamespace
from unittest.mock import patch

from design_synthesis.cli import _execute
from design_synthesis.catalog import load_catalog
from design_synthesis.emitter import compile_design
from design_synthesis.engine import SynthesisError, synthesize
from design_synthesis.requirements import normalize_requirements
from design_synthesis.solver import solve
from scripts.synthesis import validate_design


ROOT = Path(__file__).resolve().parents[2]


def financial_ir(algorithms=True):
    raw = {
        "candidate": {"crate_name": "metagdb_integration", "description": "integration test"},
        "language": "GQL",
        "schema": {"mode": "strict", "vertex_labels": ["Account"], "edge_types": ["TRANSFER"], "property_keys": ["amount"]},
        "graph": {"topology": "power_law_social"},
        "workload": {"point_read": "critical", "traversal": "high", "recovery": "high"},
        "correctness": {"isolation_level": "serializable"},
        "algorithms": [{"name": "fraud_ring", "description": "Find transfer rings"}] if algorithms else [],
    }
    return normalize_requirements(raw)


class IntegrationTests(unittest.TestCase):
    def test_generated_vector_passes_existing_backend_validator(self):
        ir = financial_ir()
        catalog = load_catalog()
        ranked, _ = solve(ir, catalog)
        design = compile_design(ir, ranked[0], catalog)
        with tempfile.NamedTemporaryFile("w", suffix=".json", dir=ROOT, encoding="utf-8", delete=False) as handle:
            json.dump(design, handle)
            path = Path(handle.name)
        try:
            validated = validate_design(path)
        finally:
            path.unlink(missing_ok=True)
        self.assertEqual(validated["language"]["name"], "GQL")
        self.assertEqual(validated["transactions"]["isolation_level"], "serializable")
        self.assertAlmostEqual(sum(validated["performance"]["workload"].values()), 1.0)
        self.assertAlmostEqual(sum(validated["performance"]["dataset_weights"].values()), 1.0)

    def test_no_algorithm_gives_algorithm_metrics_zero_weight(self):
        ir = financial_ir(algorithms=False)
        catalog = load_catalog()
        ranked, _ = solve(ir, catalog)
        design = compile_design(ir, ranked[0], catalog)
        self.assertEqual(design["performance"]["workload"]["algorithm_kernel"], 0.0)
        self.assertEqual(design["performance"]["workload"]["algorithm_e2e"], 0.0)
        self.assertFalse(any("algorithm." in metric for metric in design["performance"]["primary_metrics"]))


    def test_strict_schema_cannot_finalize_without_declarations(self):
        ir = normalize_requirements({
            "candidate": {"crate_name": "metagdb_empty_strict"},
            "language": "Cypher",
            "schema": {"mode": "strict"},
        })
        catalog = load_catalog()
        ranked, _ = solve(ir, catalog)
        with self.assertRaisesRegex(ValueError, "strict schema requires"):
            compile_design(ir, ranked[0], catalog)

    def test_evidence_bundle_contains_every_required_artifact(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "run"
            result = synthesize(financial_ir(), output, top_k=5)
            expected = {
                "requirements.json", "candidates.json", "clarification.jsonl",
                "decision_trace.json", "design_vector.json", "decision_report.md", "provenance.json",
            }
            self.assertEqual({path.name for path in output.iterdir()}, expected)
            self.assertFalse(json.loads((output / "provenance.json").read_text())["model_api_used"])
            self.assertEqual(len(json.loads((output / "candidates.json").read_text())["candidates"]), 5)
            self.assertEqual(result["design"], json.loads((output / "design_vector.json").read_text()))

    def test_unresolved_input_is_visible_in_report_and_provenance(self):
        ir = financial_ir()
        ir.unknowns.append(
            "Rejected LLM fact 12: evidence is not present in the supplied source text"
        )
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "run"
            synthesize(ir, output)
            report = (output / "decision_report.md").read_text(encoding="utf-8")
            provenance = json.loads(
                (output / "provenance.json").read_text(encoding="utf-8")
            )
        self.assertIn("## Rejected or unresolved input", report)
        self.assertIn("Rejected LLM fact 12", report)
        self.assertEqual(provenance["unresolved_count"], 1)

    def test_rejected_required_paper_component_blocks_publication(self):
        with tempfile.TemporaryDirectory() as directory:
            destination = Path(directory) / "published.json"
            args = SimpleNamespace(run_id="paper", top_k=5, output=str(destination))
            proposal = {"rejected": [{"item": "required component 0", "reason": "invalid component"}]}
            with patch("design_synthesis.cli.RUNS", Path(directory)):
                with self.assertRaisesRegex(SynthesisError, "paper-required components were rejected"):
                    _execute(financial_ir(algorithms=False), args, raw_text="paper evidence", llm_proposal=proposal)
            self.assertFalse(destination.exists())
            self.assertFalse((Path(directory) / "paper" / "design_vector.json").exists())
            self.assertTrue((Path(directory) / "paper" / "diagnostic_vector.json").exists())
            self.assertTrue((Path(directory) / "paper" / "decision_report.md").exists())
            self.assertTrue((Path(directory) / "paper" / "llm_proposal.json").exists())


if __name__ == "__main__":
    unittest.main()
