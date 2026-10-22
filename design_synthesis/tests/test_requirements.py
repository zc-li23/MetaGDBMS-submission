from __future__ import annotations

import unittest

from design_synthesis.extractor import extract_text
from design_synthesis.requirements import RequirementError, normalize_requirements


class RequirementTests(unittest.TestCase):
    def test_structured_requirements_preserve_hard_and_soft_facts(self):
        ir = normalize_requirements({
            "candidate": {"crate_name": "metagdb_unit"},
            "language": "GQL",
            "schema": {"mode": "strict"},
            "workload": {"point_read": "critical"},
            "correctness": {"isolation_level": "serializable"},
        })
        facts = {(fact.field, fact.value, fact.strength) for fact in ir.facts}
        self.assertIn(("schema.mode", "strict", "hard"), facts)
        self.assertIn(("objective.point_read", "critical", "soft"), facts)
        self.assertIn(("correctness.isolation_level", "serializable", "hard"), facts)

    def test_text_extraction_keeps_evidence_and_does_not_choose_components(self):
        ir = extract_text(
            "Gremlin must support dynamic properties. Multi-hop traversal of a social graph must be fast, and committed data must survive crashes.",
            crate_name="metagdb_text",
        )
        self.assertEqual(ir.language["name"], "Gremlin")
        self.assertTrue(any(fact.evidence for fact in ir.facts))
        self.assertTrue(any(fact.field == "graph.topology" for fact in ir.facts))
        self.assertFalse(any(fact.field == "storage.layout" for fact in ir.facts))

    def test_unknown_workload_term_is_rejected(self):
        with self.assertRaises(RequirementError):
            normalize_requirements({
                "candidate": {"crate_name": "metagdb_bad"},
                "language": "Cypher",
                "workload": {"magic_speed": "high"},
            })

    def test_open_algorithm_is_not_checked_against_a_whitelist(self):
        ir = normalize_requirements({
            "candidate": {"crate_name": "metagdb_open_algorithm"},
            "language": "Cypher",
            "algorithms": [{"name": "domain_rank", "description": "User-defined domain ranking semantics"}],
        })
        self.assertEqual(ir.algorithms[0]["name"], "domain_rank")


if __name__ == "__main__":
    unittest.main()
