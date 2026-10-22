from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from design_synthesis.engine import synthesize
from design_synthesis.extractor import extract_text
from design_synthesis.llm import (
    LlmConfig,
    _evidence_present,
    _json_object,
    InvalidLlmJson,
    call_chat_completion,
    merge_proposal,
    validate_proposal,
)


class _Response:
    def __init__(self, body: dict):
        self.body = json.dumps(body).encode("utf-8")

    def __enter__(self):
        return self

    def __exit__(self, *_):
        return False

    def read(self):
        return self.body


class LlmAdapterTests(unittest.TestCase):
    def test_json_mode_accepts_only_stray_closing_braces(self):
        self.assertEqual(_json_object('{"design_vector": {}}}'), {"design_vector": {}})
        with self.assertRaises(InvalidLlmJson):
            _json_object('{"design_vector": {}}{"other": 1}')

    def test_quote_across_acm_page_furniture_requires_exact_text_on_both_sides(self):
        source = (
            "The storage engine checks conflicts at commit.\n"
            "Proc. ACM Manag. Data, Vol. 3, No. 1 (SIGMOD), Article 12.\n\n"
            "Aster: Enhancing LSM-structures for Scalable Graph Database\n\n"
            "12:15\n\n"
            "A transaction fails when another write conflicts."
        )
        self.assertTrue(_evidence_present(
            "The storage engine checks conflicts at commit. "
            "A transaction fails when another write conflicts.", source,
        ))
        self.assertFalse(_evidence_present(
            "The storage engine checks conflicts at commit. "
            "A transaction succeeds when another write conflicts.", source,
        ))
        self.assertFalse(_evidence_present(
            "The storage engine checks conflicts at commit. "
            "A transaction fails when another write conflicts.",
            source.replace("Proc. ACM Manag. Data,", "Unrelated text,"),
        ))

    def test_openai_compatible_response_is_parsed(self):
        api = {
            "choices": [{
                "message": {
                    "content": json.dumps({
                        "language": "GQL",
                        "facts": [],
                        "schema": {},
                        "graph": {},
                        "algorithms": [],
                        "capabilities": {},
                        "storage": {},
                        "unknowns": [],
                    })
                }
            }]
        }
        with patch("design_synthesis.llm.urlopen", return_value=_Response(api)) as mocked:
            result, _ = call_chat_completion(
                LlmConfig("secret", api_base="https://example.invalid", model="test"),
                system_prompt="system",
                prompt="prompt",
            )
        self.assertEqual(result["language"], "GQL")
        request = mocked.call_args.args[0]
        self.assertEqual(request.full_url, "https://example.invalid/chat/completions")
        self.assertEqual(request.get_header("Authorization"), "Bearer secret")

    def test_uncited_model_fact_is_rejected(self):
        proposal = validate_proposal({
            "language": None,
            "facts": [{
                "field": "schema.mode",
                "value": "schemaless",
                "evidence": "not in the source",
            }],
        }, "The source says nothing about schema.", allow_custom_components=False)
        self.assertEqual(proposal["facts"], [])
        self.assertEqual(proposal["rejected"][0]["item"], "LLM fact 0")
        self.assertIn("Rejected LLM fact 0", proposal["unknowns"][0])

    def test_schema_choice_requires_cited_fact(self):
        source = "The system uses a strict schema for Account vertices."
        proposal = validate_proposal({
            "language": None,
            "facts": [],
            "schema": {
                "mode": "strict",
                "vertex_labels": ["Account"],
                "edge_types": ["TRANSFER"],
                "evidence": "uses a strict schema for Account vertices",
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["schema"], {})
        self.assertIn("matching accepted schema.mode fact", proposal["rejected"][0]["reason"])

    def test_pdf_hyphenation_and_unicode_punctuation_are_normalized(self):
        source = "The graph-\ndatabase uses ﬂat—records for point lookups."
        proposal = validate_proposal({
            "language": None,
            "facts": [{
                "field": "objective.point_read",
                "value": "high",
                "evidence": "graph-database uses flat records for point lookups",
            }],
        }, source, allow_custom_components=False)
        self.assertEqual(len(proposal["facts"]), 1)
        self.assertEqual(proposal["rejected"], [])

    def test_one_bad_fact_does_not_discard_good_facts(self):
        source = "The system uses a strict schema for Account vertices."
        proposal = validate_proposal({
            "language": None,
            "facts": [
                {
                    "field": "schema.mode",
                    "value": "strict",
                    "evidence": "uses a strict schema for Account vertices",
                },
                {
                    "field": "objective.write",
                    "value": "high",
                    "evidence": "claims a write-optimized architecture",
                },
            ],
        }, source, allow_custom_components=True)
        self.assertEqual([fact["field"] for fact in proposal["facts"]], ["schema.mode"])
        self.assertEqual(len(proposal["rejected"]), 1)

    def test_invalid_evidence_cited_fact_value_is_rejected(self):
        source = "The paper calls its layout an impossible schema."
        proposal = validate_proposal({
            "language": None,
            "facts": [{
                "field": "schema.mode",
                "value": "impossible",
                "evidence": "calls its layout an impossible schema",
            }],
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["facts"], [])
        self.assertIn("unsupported schema mode", proposal["rejected"][0]["reason"])

    def test_paper_single_writer_fact_can_force_concurrency_component(self):
        source = "Transactions are serializable as we allow one writer transaction with multiple read transactions."
        proposal = validate_proposal({
            "facts": [{
                "field": "preference.concurrency_control", "value": "single_writer",
                "strength": "hard", "evidence": "one writer transaction with multiple read transactions",
            }],
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["facts"][0]["value"], "single_writer")
        base = extract_text(source, crate_name="metagdb_single_writer", language="Cypher")
        base.facts = []
        merged = merge_proposal(
            base, proposal, model="mock", response_text=json.dumps(proposal),
            source_kind="paper+llm",
        )
        with tempfile.TemporaryDirectory() as directory:
            result = synthesize(merged, Path(directory) / "run")
        self.assertEqual(result["design"]["transactions"]["concurrency_control"], "single_writer")

    def test_rejected_custom_component_cannot_remain_required(self):
        source = "The paper introduces Poly LSM records for graph storage."
        proposal = validate_proposal({
            "language": None,
            "facts": [],
            "storage": {
                "custom_components": [{
                    "id": "paper.poly_lsm",
                    "dimension": "storage_layout",
                    "description": "Poly LSM",
                    "evidence": "not present in the paper text",
                }],
                "required_components": ["paper.poly_lsm"],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["custom_components"], [])
        self.assertEqual(proposal["storage"]["required_components"], [])
        self.assertEqual(
            [item["item"] for item in proposal["rejected"]],
            ["custom component 0", "required component 0"],
        )

    def test_unprovided_paper_capability_is_reported_without_blocking_valid_component(self):
        source = "Poly-LSM uses pivot and delta entries for graph storage."
        proposal = validate_proposal({
            "storage": {
                "required_capabilities": [{
                    "value": "LSM-tree storage",
                    "evidence": "pivot and delta entries for graph storage",
                }],
                "required_components": [{"id": "paper.poly_lsm"}],
                "custom_components": [{
                    "id": "paper.poly_lsm",
                    "dimension": "storage_layout",
                    "description": "Poly-LSM pivot and delta entries",
                    "emits": {"layout": "Poly-LSM pivot and delta entries"},
                    "provides": ["disk_records"],
                    "maturity": "prototype",
                    "evidence": "Poly-LSM uses pivot and delta entries for graph storage",
                }],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["required_capabilities"], [])
        self.assertEqual(proposal["storage"]["required_components"], ["paper.poly_lsm"])
        self.assertEqual(proposal["storage"]["custom_components"][0]["maturity"], "experimental")
        self.assertTrue(any("LSM-tree storage" in item for item in proposal["unknowns"]))
        self.assertTrue(any("prototype to experimental" in item for item in proposal["unknowns"]))

    def test_invalid_algorithm_identifier_is_rejected_before_backend_check(self):
        source = "The system uses adaptive edge updates for storage."
        proposal = validate_proposal({
            "algorithms": [{
                "name": "adaptive edge update",
                "description": "choose an edge storage update method",
                "evidence": "adaptive edge updates for storage",
            }],
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["algorithms"], [])
        self.assertIn("name must start", proposal["unknowns"][0])

    def test_malformed_component_affinity_is_reported(self):
        source = "Poly-LSM stores graph records in a hybrid layout."
        proposal = validate_proposal({
            "storage": {"custom_components": [{
                "id": "paper.poly_lsm",
                "dimension": "storage_layout",
                "description": "hybrid graph layout",
                "affinity": "write-heavy",
                "evidence": "Poly-LSM stores graph records in a hybrid layout",
            }]},
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["custom_components"], [])
        self.assertIn("affinity must be an object", proposal["rejected"][0]["reason"])
        self.assertEqual(proposal["rejected"][0]["raw_component"]["id"], "paper.poly_lsm")

    def test_invalid_exclusive_component_and_dependent_are_rejected(self):
        source = "The database uses write ahead logging and a custom join operator."
        proposal = validate_proposal({
            "storage": {
                "required_components": ["paper.wal", "paper.join"],
                "custom_components": [{
                    "id": "paper.wal", "dimension": "recovery",
                    "description": "write ahead logging", "emits": {"log": "wal"},
                    "evidence": "database uses write ahead logging",
                }, {
                    "id": "paper.join", "dimension": "augmentation",
                    "description": "custom join operator",
                    "requires_components": ["paper.wal"],
                    "evidence": "a custom join operator",
                }],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["custom_components"], [])
        self.assertEqual(proposal["storage"]["required_components"], [])
        self.assertTrue(any("must emit recovery" in item["reason"] for item in proposal["rejected"]))
        self.assertTrue(any("requires unavailable component paper.wal" in item["reason"] for item in proposal["rejected"]))

    def test_paper_component_flows_through_deterministic_solver(self):
        source = (
            "Aster supports Gremlin. Poly-LSM uses a hybrid graph-oriented LSM "
            "layout with pivot and delta entries for graph updates and lookups."
        )
        base = extract_text(source, crate_name="metagdb_paper_unit", language="Gremlin")
        proposal = validate_proposal({
            "language": {
                "name": "Gremlin",
                "evidence": "Aster supports Gremlin",
            },
            "facts": [{
                "field": "objective.write",
                "value": "high",
                "strength": "soft",
                "confidence": 0.7,
                "evidence": "pivot and delta entries for graph updates and lookups",
            }],
            "schema": {},
            "graph": {},
            "algorithms": [],
            "capabilities": {"required": [], "optional": []},
            "storage": {
                "required_components": ["paper.poly_lsm"],
                "custom_components": [{
                    "id": "paper.poly_lsm",
                    "dimension": "storage_layout",
                    "description": "paper-defined Poly-LSM hybrid graph layout",
                    "emits": {"layout": "Poly-LSM pivot and delta graph entries"},
                    "provides": ["disk_records", "outgoing_adjacency", "incoming_adjacency"],
                    "affinity": {"write": 3.0, "traversal": 2.0},
                    "complexity": 2.0,
                    "risk": 0.8,
                    "maturity": "experimental",
                    "evidence": "Poly-LSM uses a hybrid graph-oriented LSM layout with pivot and delta entries",
                }],
            },
            "unknowns": [],
        }, source, allow_custom_components=True)
        merged = merge_proposal(
            base,
            proposal,
            model="mock-paper-model",
            response_text=json.dumps(proposal),
            source_kind="paper+llm",
        )
        with tempfile.TemporaryDirectory() as directory:
            result = synthesize(merged, Path(directory) / "run")
            provenance = json.loads(
                (Path(result["output_dir"]) / "provenance.json").read_text(encoding="utf-8")
            )
        self.assertIn("paper.poly_lsm", result["trace"]["selected"]["components"])
        self.assertIn("paper-defined Poly-LSM hybrid graph layout", result["design"]["storage"]["layout"])
        self.assertTrue(any(
            "implement paper.poly_lsm: paper-defined Poly-LSM hybrid graph layout" in item
            for item in result["design"]["capabilities"]["required"]
        ))
        self.assertTrue(provenance["model_api_used"])
        self.assertEqual(provenance["model"], "mock-paper-model")

    def test_kuzu_join_operators_and_component_dependencies_are_normalized(self):
        source = (
            "Kuzu uses factorized vectors and ASP-Join for query processing. "
            "Multiway ASP-Join handles cyclic joins."
        )
        def component(component_id, dimension, evidence, compatibility):
            return {
                "id": component_id, "dimension": dimension,
                "description": component_id.replace("paper.", ""),
                "emits": {"model": "push_based"} if dimension == "execution" else {},
                "provides": [component_id.replace("paper.", "")],
                "compatible_with": compatibility, "evidence": evidence,
            }
        proposal = validate_proposal({
            "algorithms": [{
                "name": "ASP-Join", "description": "binary query join",
                "evidence": "ASP-Join for query processing",
            }],
            "storage": {
                "required_components": [
                    "paper.factorized_vectors", "paper.asp_join", "paper.multiway_asp_join",
                ],
                "custom_components": [
                    component("paper.factorized_vectors", "augmentation",
                              "uses factorized vectors", {}),
                    component("paper.asp_join", "execution",
                              "ASP-Join for query processing",
                              {"paper.factorized_vectors": "required"}),
                    component("paper.multiway_asp_join", "execution",
                              "Multiway ASP-Join handles cyclic joins",
                              {"paper.factorized_vectors": "required"}),
                ],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["algorithms"], [])
        self.assertEqual(len(proposal["storage"]["required_components"]), 3)
        self.assertFalse(any(x["item"].startswith("required component") for x in proposal["rejected"]))
        for item in proposal["storage"]["custom_components"]:
            self.assertEqual(item["dimension"], "augmentation")
        self.assertEqual(
            proposal["storage"]["custom_components"][1]["requires_components"],
            ["paper.factorized_vectors"],
        )
        base = extract_text(source, crate_name="metagdb_kuzu_test", language="Cypher")
        merged = merge_proposal(
            base, proposal, model="mock", response_text=json.dumps(proposal),
            source_kind="paper+llm",
        )
        with tempfile.TemporaryDirectory() as directory:
            result = synthesize(merged, Path(directory) / "run")
        self.assertTrue(all(
            item in result["trace"]["selected"]["components"]
            for item in ("paper.factorized_vectors", "paper.asp_join", "paper.multiway_asp_join")
        ))
        self.assertEqual(result["design"]["algorithms"]["required"], [])

    def test_missing_custom_dependency_capability_rejects_required_component(self):
        source = "Kuzu uses a factorized ASP-Join operator."
        proposal = validate_proposal({
            "storage": {
                "required_components": ["paper.asp_join"],
                "custom_components": [{
                    "id": "paper.asp_join", "dimension": "execution",
                    "description": "factorized ASP-Join query operator",
                    "emits": {"model": "vectorized"},
                    "provides": ["factorized_join"],
                    "requires_capabilities": ["columnar_storage"],
                    "evidence": "factorized ASP-Join operator",
                }],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["required_components"], [])
        self.assertTrue(any(
            "no accepted component provides required capability 'columnar_storage'" in item["reason"]
            for item in proposal["rejected"]
        ))
        self.assertTrue(any(
            item["item"] == "required component 0" for item in proposal["rejected"]
        ))

    def test_kuzu_columnar_and_csr_layers_compose(self):
        source = (
            "Kuzu stores columnar node records and CSR adjacency lists. "
            "ASP-Join uses both columnar records and CSR adjacency lists."
        )
        proposal = validate_proposal({
            "storage": {
                "required_components": [
                    "paper.columnar_storage", "paper.csr_adjacency", "paper.asp_join",
                ],
                "custom_components": [
                    {
                        "id": "paper.columnar_storage", "dimension": "storage_layout",
                        "description": "columnar node records", "emits": {"layout": "columnar"},
                        "provides": ["columnar_storage"],
                        "evidence": "stores columnar node records",
                    },
                    {
                        "id": "paper.csr_adjacency", "dimension": "storage_layout",
                        "description": "CSR adjacency lists", "emits": {"layout": "CSR"},
                        "provides": ["csr_adjacency"],
                        "evidence": "CSR adjacency lists",
                    },
                    {
                        "id": "paper.asp_join", "dimension": "execution",
                        "description": "ASP-Join using columnar and CSR data",
                        "emits": {"model": "vectorized"},
                        "provides": ["asp_join"],
                        "requires_capabilities": ["columnar_storage", "csr_adjacency"],
                        "evidence": "ASP-Join uses both columnar records and CSR adjacency lists",
                    },
                ],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["required_components"], [
            "paper.columnar_storage", "paper.csr_adjacency", "paper.asp_join",
        ])
        components = proposal["storage"]["custom_components"]
        self.assertEqual([item["dimension"] for item in components], [
            "storage_layout", "augmentation", "augmentation",
        ])
        self.assertIn("disk_records", components[0]["provides"])
        base = extract_text(source, crate_name="metagdb_kuzu_compose", language="Cypher")
        base.facts = []
        merged = merge_proposal(
            base, proposal, model="mock", response_text=json.dumps(proposal),
            source_kind="paper+llm",
        )
        with tempfile.TemporaryDirectory() as directory:
            result = synthesize(merged, Path(directory) / "run")
        self.assertTrue(all(
            component in result["trace"]["selected"]["components"]
            for component in proposal["storage"]["required_components"]
        ))

    def test_kuzu_wal_and_buffer_manager_get_structural_capabilities(self):
        source = (
            "Kuzu uses write ahead logging to achieve atomicity and durability. "
            "Storage structures use a buffer manager with GClock eviction."
        )
        proposal = validate_proposal({
            "storage": {
                "required_components": ["paper.wal", "paper.buffer_manager"],
                "custom_components": [
                    {
                        "id": "paper.wal", "dimension": "recovery",
                        "description": "write ahead logging",
                        "emits": {"recovery": "wal"}, "provides": ["wal"],
                        "evidence": "uses write ahead logging to achieve atomicity and durability",
                    },
                    {
                        "id": "paper.buffer_manager", "dimension": "storage_update",
                        "description": "buffer manager with GClock eviction",
                        "emits": {"buffer": "gclock"},
                        "provides": ["buffer_manager"],
                        "evidence": "buffer manager with GClock eviction",
                    },
                ],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(proposal["storage"]["required_components"], [
            "paper.wal", "paper.buffer_manager",
        ])
        self.assertEqual(proposal["rejected"], [])
        components = proposal["storage"]["custom_components"]
        self.assertTrue({"atomic_commit", "crash_safe"}.issubset(components[0]["provides"]))
        self.assertEqual(components[1]["dimension"], "augmentation")
        misplaced = dict(components[1], dimension="storage_layout")
        reparsed = validate_proposal({
            "storage": {
                "required_components": ["paper.buffer_manager"],
                "custom_components": [misplaced],
            },
        }, source, allow_custom_components=True)
        self.assertEqual(reparsed["storage"]["required_components"], ["paper.buffer_manager"])
        self.assertEqual(reparsed["storage"]["custom_components"][0]["dimension"], "augmentation")


if __name__ == "__main__":
    unittest.main()
