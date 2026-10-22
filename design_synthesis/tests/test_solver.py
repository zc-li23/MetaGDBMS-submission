from __future__ import annotations

import unittest
import json
from pathlib import Path
import tempfile

from design_synthesis.catalog import load_catalog
from design_synthesis.engine import SynthesisError, synthesize
from design_synthesis.requirements import normalize_requirements
from design_synthesis.solver import UnsatisfiableRequirements, clarification_candidates, solve


def base(**updates):
    value = {
        "candidate": {"crate_name": "metagdb_solver"},
        "language": "Gremlin",
        "schema": {"mode": "schemaless"},
        "workload": {"traversal": "high", "recovery": "high"},
        "correctness": {"isolation_level": "snapshot_isolation"},
    }
    value.update(updates)
    return value


class SolverTests(unittest.TestCase):
    def test_hard_requirements_cannot_be_overridden_by_score(self):
        ir = normalize_requirements(base(schema={"mode": "strict"}, correctness={"isolation_level": "serializable"}))
        catalog = load_catalog()
        ranked, _ = solve(ir, catalog)
        self.assertIn("schema.strict", ranked[0].components)
        self.assertIn("isolation.serializable", ranked[0].components)

    def test_conflicting_hard_facts_report_unsatisfiable(self):
        raw = base(schema={"mode": "strict"})
        raw["facts"] = [{"field": "schema.mode", "value": "schemaless", "strength": "hard"}]
        with self.assertRaises(UnsatisfiableRequirements) as context:
            solve(normalize_requirements(raw), load_catalog())
        self.assertIn("conflicting hard requirements", str(context.exception))

    def test_unknown_required_capability_is_unsatisfiable(self):
        raw = base()
        raw["storage"] = {"required_capabilities": ["impossible_quantum_index"]}
        with self.assertRaises(UnsatisfiableRequirements):
            solve(normalize_requirements(raw), load_catalog())

    def test_unsatisfiable_run_preserves_requirements_and_explains_rejection(self):
        raw = base()
        raw["storage"] = {"required_capabilities": ["impossible_quantum_index"]}
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "run"
            with self.assertRaises(SynthesisError) as context:
                synthesize(normalize_requirements(raw), output)
            trace = json.loads((output / "decision_trace.json").read_text())
            requirements = json.loads((output / "requirements.json").read_text())
            self.assertIn("impossible_quantum_index", str(context.exception))
            self.assertTrue(any("impossible_quantum_index" in reason for reason in trace["reasons"]))
            self.assertEqual(requirements["candidate"]["crate_name"], "metagdb_solver")

    def test_custom_storage_component_extends_the_search_space(self):
        raw = base()
        raw["preferences"] = {"storage_layout_required": "layout.paper_csr"}
        raw["storage"] = {
            "required_components": ["layout.paper_csr", "support.paper_cache"],
            "custom_components": [{
            "id": "layout.paper_csr",
            "dimension": "storage_layout",
            "description": "paper-defined multi-level CSR with delta buffers",
            "emits": {"layout": "paper-defined multi-level CSR with delta buffers"},
            "provides": ["disk_records", "direct_adjacency", "analytics_projection"],
            "compatible_with": {"storage_update": ["update.delta_merge"]},
            "affinity": {"traversal": 4.0, "analytics": 3.0},
            "complexity": 2.0,
            "risk": 0.6,
            "maturity": "experimental"
        }, {
            "id": "support.paper_cache",
            "dimension": "augmentation",
            "description": "paper-defined two-level adjacency cache",
            "provides": ["paper_cache"],
            "complexity": 0.5,
            "risk": 0.2,
            "maturity": "experimental"
        }]}
        ir = normalize_requirements(raw)
        catalog = load_catalog(ir.custom_components)
        ranked, _ = solve(ir, catalog)
        self.assertIn("layout.paper_csr", ranked[0].components)
        self.assertIn("support.paper_cache", ranked[0].components)
        self.assertIn("update.delta_merge", ranked[0].components)

    def test_solver_and_question_order_are_deterministic(self):
        ir = normalize_requirements(base())
        catalog = load_catalog()
        first, first_trace = solve(ir, catalog)
        second, second_trace = solve(ir, catalog)
        self.assertEqual(first[0], second[0])
        self.assertEqual(first_trace, second_trace)
        self.assertEqual(clarification_candidates(first, catalog), clarification_candidates(second, catalog))


if __name__ == "__main__":
    unittest.main()
