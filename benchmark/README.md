# Benchmarks

This directory contains the inputs and helper code used by the framework's
benchmark commands.

| Command | Inputs in this directory |
|---|---|
| `python3 scripts/build.py benchmark <crate>` | `<language>/core.jsonl`, `fixtures.jsonl`, and `syntax_check/` |
| `python3 scripts/build.py system-benchmark <crate> --design <design.json>` | The executable checks are in `metagdb_src/metagdb/src/system_benchmark.rs`; no separate case file is required. |
| `python3 scripts/build.py micro-benchmark <crate> --design <design.json>` | `performance/datasets.json`, `performance/query_templates.json`, and `performance/generate_uniform.py` |
| `python3 scripts/build.py compare-performance ...` | `performance/compare.py` |

The Cypher, GQL, and Gremlin `profile.json` files summarize coverage and
limitations of the corresponding fixed `core.jsonl` files. The Core Language
Profile is a named capability profile, not a complete language standard. The
Gremlin profile cites `legacy/gremlin/tinkergraph_oracle.tsv` as provenance for
some expected answers.

`graded/suite200.jsonl` contains the 200 functional ablation cases and their
expected answers.

See `performance/README.md` for the performance workflow.
