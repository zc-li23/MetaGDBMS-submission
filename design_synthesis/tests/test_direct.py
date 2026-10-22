from __future__ import annotations

import copy
import json
from pathlib import Path
import tempfile
import unittest

from design_synthesis.direct import TEMPLATE, _prompt, _validate_envelope, synthesize_direct
from design_synthesis.engine import SynthesisError
from design_synthesis.llm import InvalidLlmJson, LlmConfig


SOURCE = (
    "The system stores graph data in columnar CSR adjacency lists. "
    "It executes factorized vectors with ASP-Join and multiway ASP-Join. "
    "Transactions use write-ahead logging."
)


def envelope() -> dict:
    vector = json.loads(TEMPLATE.read_text(encoding="utf-8"))
    vector["candidate"]["crate_name"] = "metagdb_direct_test"
    vector["language"] = {"name": "Cypher", "grammar_path": "data/language/Cypher.g4"}
    vector["storage"]["layout"] = "columnar CSR adjacency lists and factorized vectors"
    vector["storage"]["recovery"] = "write-ahead logging and replay"
    vector["execution"]["model"] = "vectorized"
    vector["capabilities"]["required"].extend([
        "factorized vectors for intermediate query results",
        "ASP-Join and multiway ASP-Join query execution",
    ])
    vector["quality"]["required_tests"].extend([
        "factorized-intermediate correctness test",
        "direct and public-language join correctness tests",
        "direct binary ASP-Join correctness test",
        "public-language binary ASP-Join correctness test",
        "direct multiway join correctness test",
        "public-language multiway join correctness test",
    ])
    vector["algorithms"]["required"] = []
    workload = vector["performance"]["workload"]
    workload["algorithm_kernel"] = workload["algorithm_e2e"] = workload["algorithm_projection_build"] = 0
    total = sum(workload.values())
    for key in workload:
        workload[key] /= total
    return {
        "design_vector": vector,
        "evidence": [
            {"field": "storage.layout", "quote": "stores graph data in columnar CSR adjacency lists"},
            {"field": "storage.recovery", "quote": "Transactions use write-ahead logging"},
            {"field": "capabilities.required", "quote": "factorized vectors with ASP-Join and multiway ASP-Join"},
        ],
        "assumptions": [
            {"field": "transactions.isolation_level", "reason": "source omits isolation"},
            {"field": "execution.model", "reason": "model enum chosen to implement vector execution"},
        ],
    }


class DirectDesignTests(unittest.TestCase):
    def test_join_specific_prompt_is_only_used_for_join_paper(self):
        aster_source = "Poly-LSM adapts pivot and delta updates."
        kuzu_source = SOURCE + " Binary ASP-Join and S-Join are required."
        for source, expected in ((aster_source, False), (kuzu_source, True)):
            initial = _prompt(
                source, crate="metagdb_direct_test", language="Cypher",
                description="", paper_mode=True, prior=None, errors=[], review=False,
            )
            review = _prompt(
                source, crate="metagdb_direct_test", language="Cypher",
                description="", paper_mode=True, prior=envelope(), errors=[], review=True,
            )
            self.assertEqual("For every required binary ASP-Join" in initial, expected)
            self.assertEqual("factorized query representations" in review, expected)

    def test_workload_feedback_gives_exact_arithmetic_repair(self):
        draft = envelope()
        draft["design_vector"]["performance"]["workload"]["storage_disk_size"] += 0.05
        prompt = _prompt(
            SOURCE, crate="metagdb_direct_test", language="Cypher",
            description="", paper_mode=True, prior=draft,
            errors=["performance.workload sums to 1.050000"], review=False,
        )
        self.assertIn("Exact arithmetic repair", prompt)
        self.assertIn("performance.workload.query_point_read", prompt)
        self.assertIn("to 0.172222", prompt)

    def test_invalid_draft_is_repaired_and_reviewed_before_publication(self):
        valid = envelope()
        bad = copy.deepcopy(valid)
        bad["design_vector"]["schema_version"] = 2
        responses = [bad, valid, valid]
        prompts = []

        def completion(_config, *, system_prompt, prompt):
            prompts.append(prompt)
            value = responses.pop(0)
            return value, json.dumps(value)

        with tempfile.TemporaryDirectory() as directory:
            result = synthesize_direct(
                SOURCE, Path(directory) / "run", crate="metagdb_direct_test",
                language="Cypher", description="Kuzu test", paper_mode=True,
                config=LlmConfig("secret", model="mock"), completion=completion,
            )
            run = Path(result["output_dir"])
            provenance = json.loads((run / "provenance.json").read_text())
            self.assertEqual(len(result["attempts"]), 3)
            self.assertIn("schema_version must be 1", prompts[1])
            self.assertIn("Independently review", prompts[2])
            self.assertTrue((run / "design_vector.json").exists())
            self.assertEqual(provenance["status"], "accepted")
            self.assertNotIn("secret", (run / "provenance.json").read_text())

    def test_uncited_paper_claim_cannot_publish(self):
        bad = envelope()
        bad["evidence"][0]["quote"] = "a fabricated paper quote"

        def completion(_config, *, system_prompt, prompt):
            return bad, json.dumps(bad)

        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / "run"
            with self.assertRaisesRegex(SynthesisError, "failed after"):
                synthesize_direct(
                    SOURCE, run, crate="metagdb_direct_test", language="Cypher",
                    description="", paper_mode=True, config=LlmConfig("secret"),
                    max_repair_rounds=1, completion=completion,
                )
            self.assertFalse((run / "design_vector.json").exists())
            self.assertEqual(json.loads((run / "provenance.json").read_text())["status"], "failed")

    def test_join_operator_is_not_accepted_as_graph_algorithm(self):
        bad = envelope()
        bad["design_vector"]["algorithms"]["required"] = [{
            "name": "ASP-Join", "description": "internal join", "performance_weight": 1.0,
        }]
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                bad, Path(directory) / "attempt.json", SOURCE,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("query join operator" in item for item in errors))

    def test_paper_semantic_conflicts_are_repaired_before_publication(self):
        bad = envelope()
        bad["design_vector"]["capabilities"]["required"].append(
            "DDL for pre-defined node and edge relations"
        )
        bad["design_vector"]["storage"]["recovery"] = "committed updates can be rolled back"
        bad["design_vector"]["quality"]["required_tests"] = ["storage restart test"]
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                bad, Path(directory) / "attempt.json", SOURCE,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("schema.mode=schemaless" in item for item in errors))
        self.assertTrue(any("committed updates" in item for item in errors))
        self.assertTrue(any("join correctness" in item for item in errors))
        self.assertTrue(any("factorized-intermediate" in item for item in errors))
        self.assertTrue(any("multiway join correctness" in item for item in errors))

    def test_join_focused_paper_rejects_point_read_performance_profile(self):
        draft = envelope()
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json",
                SOURCE + " Its factorized query processor optimizes many-to-many joins.",
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("performance.profile" in item for item in errors))
        self.assertTrue(any("primary_metrics" in item for item in errors))
        self.assertTrue(any("join-focused workload proxy" in item for item in errors))

    def test_poly_lsm_paper_requires_skew_encoding_and_round_trip_test(self):
        draft = envelope()
        source = (
            SOURCE + " Poly-LSM exploits the skewness of graph data to encode entries. "
            "The design uses partitioned Elias-Fano to encode adjacency lists."
        )
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", source,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("skew-aware entry encoding" in item for item in errors))
        self.assertTrue(any("partitioned Elias-Fano" in item for item in errors))
        self.assertTrue(any("entry-encoding round-trip" in item for item in errors))

        draft["design_vector"]["storage"]["layout"] += "; skew-aware partitioned Elias-Fano entry encoding"
        draft["design_vector"]["quality"]["required_tests"].append(
            "partitioned Elias-Fano entry-encoding round-trip correctness test"
        )
        draft["design_vector"]["quality"]["required_tests"].append(
            "direct pivot and delta edge update, adaptive selection, and degree sketch correctness tests"
        )
        workload = draft["design_vector"]["performance"]["workload"]
        workload["storage_disk_size"] = 0.01
        workload["query_point_read"] -= 0.01
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", source,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertEqual(errors, [])

    def test_poly_lsm_rejects_unsupported_ddl_and_conflated_degree_sketch(self):
        draft = envelope()
        vector = draft["design_vector"]
        vector["schema"]["mode"] = "hybrid"
        vector["capabilities"]["required"].append("DDL-defined node and edge relations")
        vector["storage"]["layout"] += "; skew-aware partitioned Elias-Fano encoding via degree-sketch-guided partitioning"
        vector["quality"]["required_tests"].append("entry-encoding round-trip test")
        vector["performance"]["workload"]["storage_disk_size"] = 0
        source = (
            SOURCE + " Poly-LSM exploits the skewness of graph data to encode entries. "
            "Partitioned Elias-Fano uses edge locality. A degree sketch selects pivot or delta updates."
        )
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", source,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("unsupported DDL" in item for item in errors))
        self.assertTrue(any("schema.mode=hybrid" in item for item in errors))
        self.assertTrue(any("degree sketch selects" in item for item in errors))
        self.assertTrue(any("storage_disk_size needs nonzero" in item for item in errors))

    def test_poly_lsm_graphalytics_list_cannot_omit_cdlp(self):
        draft = envelope()
        vector = draft["design_vector"]
        vector["algorithms"]["required"] = [
            {"name": name, "description": name, "performance_weight": 0.25}
            for name in ("bfs", "pagerank", "wcc", "sssp")
        ]
        source = (
            SOURCE + " Poly-LSM exploits the skewness of graph data to encode entries. "
            "We test five algorithms including BFS, PageRank, WCC, SSSP, and CDLP."
        )
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", source,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("missing cdlp" in item for item in errors))

    def test_unrelated_join_tests_are_rejected(self):
        draft = envelope()
        draft["design_vector"]["quality"]["required_tests"].append(
            "direct S-Join correctness test"
        )
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", SOURCE.replace("ASP-Join", "hash join"),
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("absent from this paper" in item for item in errors))

    def test_each_required_join_operator_needs_both_test_paths(self):
        draft = envelope()
        draft["design_vector"]["capabilities"]["required"].extend([
            "binary ASP-Join query operator", "S-Join query operator",
        ])
        draft["design_vector"]["quality"]["required_tests"] = [
            "direct multiway join correctness test",
            "public-language multiway join correctness test",
            "factorized-intermediate correctness test",
        ]
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", SOURCE,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("direct binary ASP-Join" in item for item in errors))
        self.assertTrue(any("public-language binary ASP-Join" in item for item in errors))
        self.assertTrue(any("direct S-Join" in item for item in errors))
        self.assertTrue(any("public-language S-Join" in item for item in errors))

    def test_malformed_enum_and_workload_return_feedback(self):
        draft = envelope()
        draft["design_vector"]["execution"]["model"] = {"kind": "factorized"}
        draft["design_vector"]["performance"]["workload"]["query_one_hop"] = "large"
        with tempfile.TemporaryDirectory() as directory:
            errors = _validate_envelope(
                draft, Path(directory) / "attempt.json", SOURCE,
                crate="metagdb_direct_test", language="Cypher", paper_mode=True,
            )
        self.assertTrue(any("execution.model" in item for item in errors))

    def test_invalid_model_json_is_recorded_and_retried(self):
        valid = envelope()
        responses = ["invalid", valid, valid]

        def completion(_config, *, system_prompt, prompt):
            value = responses.pop(0)
            if value == "invalid":
                raise InvalidLlmJson('{"design_vector":')
            return value, json.dumps(value)

        with tempfile.TemporaryDirectory() as directory:
            result = synthesize_direct(
                SOURCE, Path(directory) / "run", crate="metagdb_direct_test",
                language="Cypher", description="", paper_mode=True,
                config=LlmConfig("secret"), completion=completion,
            )
            run = Path(result["output_dir"])
            self.assertEqual(len(result["attempts"]), 3)
            self.assertTrue((run / "attempt_01_response.txt").exists())
            self.assertIn("invalid JSON", (run / "attempt_01_feedback.json").read_text())

    def test_failed_run_can_resume_without_repeating_draft(self):
        bad = envelope()
        bad["design_vector"]["schema_version"] = 2
        valid = envelope()

        def bad_completion(_config, *, system_prompt, prompt):
            return bad, json.dumps(bad)

        good_responses = [valid, valid]
        def good_completion(_config, *, system_prompt, prompt):
            value = good_responses.pop(0)
            return value, json.dumps(value)

        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / "run"
            kwargs = dict(
                crate="metagdb_direct_test", language="Cypher", description="",
                paper_mode=True, config=LlmConfig("secret", model="mock"),
            )
            with self.assertRaises(SynthesisError):
                synthesize_direct(SOURCE, run, max_repair_rounds=1,
                                  completion=bad_completion, **kwargs)
            self.assertEqual(len(list(run.glob("attempt_*_feedback.json"))), 3)
            result = synthesize_direct(SOURCE, run, resume=True,
                                       completion=good_completion, **kwargs)
            self.assertEqual(len(result["attempts"]), 5)
            self.assertTrue((run / "design_vector.json").exists())


if __name__ == "__main__":
    unittest.main()
