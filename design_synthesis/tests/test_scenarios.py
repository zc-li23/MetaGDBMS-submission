from __future__ import annotations

import unittest

from design_synthesis.catalog import load_catalog
from design_synthesis.emitter import compile_design
from design_synthesis.extractor import extract_text
from design_synthesis.requirements import normalize_requirements
from design_synthesis.solver import solve
from scripts.synthesis import validate_design


def compile_ir(ir):
    catalog = load_catalog(ir.custom_components)
    ranked, _ = solve(ir, catalog)
    design = compile_design(ir, ranked[0], catalog)
    validate_design_data = design
    return design, ranked[0]


class ScenarioE2ETests(unittest.TestCase):
    def test_security_event_business_request(self):
        ir = extract_text(
            "Build an ISO GQL security event graph. New labels and dynamic properties "
            "must be accepted without schema DDL. The database must guarantee "
            "serializable transactions, crash-safe durability, and fast multi-hop "
            "investigation traversals over a power-law social graph.",
            crate_name="metagdb_security_scenario",
        )
        design, selected = compile_ir(ir)
        self.assertEqual(design["language"]["name"], "GQL")
        self.assertEqual(design["schema"]["mode"], "schemaless")
        self.assertEqual(design["transactions"]["isolation_level"], "serializable")
        self.assertIn("schema.schemaless", selected.components)
        self.assertIn("isolation.serializable", selected.components)

    def test_social_graph_business_request(self):
        ir = extract_text(
            "Build a Gremlin social graph system with dynamic properties and labels. "
            "Multi-hop traversal must be fast. Use snapshot isolation and durable commits.",
            crate_name="metagdb_social_scenario",
        )
        design, _ = compile_ir(ir)
        self.assertEqual(design["schema"]["mode"], "schemaless")
        self.assertEqual(design["transactions"]["isolation_level"], "snapshot_isolation")
        self.assertEqual(max(design["performance"]["dataset_weights"], key=design["performance"]["dataset_weights"].get), "yt")

    def test_structured_railway_request_finalizes_strict_schema(self):
        ir = normalize_requirements({
            "candidate": {"crate_name": "metagdb_rail_scenario"},
            "language": "Cypher",
            "schema": {
                "mode": "strict",
                "vertex_labels": ["Station", "Train"],
                "edge_types": ["CONNECTS", "SERVES"],
                "property_keys": ["name", "distance"],
            },
            "graph": {"topology": "sparse_railway"},
            "workload": {"point_read": "critical", "traversal": "high"},
            "correctness": {"isolation_level": "serializable"},
        })
        design, selected = compile_ir(ir)
        self.assertEqual(design["schema"]["vertex_labels"], ["Station", "Train"])
        self.assertEqual(max(design["performance"]["dataset_weights"], key=design["performance"]["dataset_weights"].get), "rn")
        self.assertIn("schema.strict", selected.components)

    def test_repeated_paper_terms_do_not_multiply_facts(self):
        ir = extract_text(
            "A Gremlin graph traversal system. Graph traversal is important. "
            "Graph traversal appears in the evaluation again.",
            crate_name="metagdb_dedup_scenario",
        )
        traversal = [fact for fact in ir.facts if fact.field == "objective.traversal"]
        self.assertEqual(len(traversal), 1)


if __name__ == "__main__":
    unittest.main()
