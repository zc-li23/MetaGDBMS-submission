# metaGDB

metaGDB is a framework for generating graph database implementations from a
language grammar and a system requirement. It defines stable component
interfaces, a query execution path, and a workflow for building and checking
generated implementations.

This repository contains the framework, the three generated systems evaluated
in the paper, parser crates for Cypher, GQL, and Gremlin, benchmark cases,
graph inputs, and tables of the reported results.

## Repository layout

| Path | Purpose |
|---|---|
| `metagdb_src/metagdb_core/` | Stable traits and intermediate representation. |
| `metagdb_src/metagdb/` | Runtime and command-line interface. |
| `metagdb_src/implementation/metagdb_default/` | Reference implementation. |
| `metagdb_src/implementation/generated/` | Source code for MG-Read, MG-Update, and MG-Alg. |
| `metagdb_src/implementation/language/` | Pre-generated parser crates. |
| `data/language/` | Source grammars. |
| `data/raw/*.graph.gz` | Uniform, RN, and YT evaluation inputs; see `data/README.md`. |
| `artifacts/results.md` | Tables of the paper's reported experimental results. |
| `benchmark/` | Fixed language, system, and performance benchmarks. |
| `benchmark/graded/suite240.jsonl` | The 240 functional ablation cases and expected answers. |
| `scripts/build.py` | Select, build, verify, and benchmark an implementation. |
| `scripts/synthesis.py` | Manage the staged generation workflow. |
| `docs/` | Framework contracts and workflow documentation. |

## Build and run

The active Cargo manifests are generated when an implementation is selected.
From a fresh clone, run:

```bash
python3 scripts/build.py use metagdb_default
cargo test --workspace
cargo run -- --demo
```

Run `cargo run` without `--demo` for the interactive query shell. The reference
implementation provides an integration example.

## Generate an implementation

The workflow can start from a requirement file or a finalized design vector:

```bash
python3 scripts/synthesis.py start --text requirements.txt --crate my_graph_db --language Cypher --run-id example-01
python3 scripts/synthesis.py check --design design_vector.json
python3 scripts/synthesis.py init --design design_vector.json
```

The coding agent reads `AGENTS.md` and `docs/synthesis/README.md`, implements a
new crate under `metagdb_src/implementation/generated/`, and runs the checks.
See `docs/implementation_contract.md` for the crate contract and
`benchmark/README.md` for benchmark inputs and commands.

The functional ablation cases and their expected answers are recorded in
`benchmark/graded/suite240.jsonl`.

## Evaluation artifacts

The generated systems are `metagdb_exp_lat_01` (MG-Read),
`metagdb_exp_thr_02` (MG-Update), and `metagdb_exp_alg_01` (MG-Alg). Their
Rust source code is under `metagdb_src/implementation/generated/`. Graph
provenance and checksums are in `data/README.md`; the paper's numerical results
are in `artifacts/results.md`.
