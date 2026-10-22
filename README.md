# metaGDB

metaGDB is a framework for generating graph database implementations from a
language grammar and a system requirement. It defines stable component
interfaces, a query execution path, and a workflow for building and checking
generated implementations.

This repository contains the framework, source snapshots of the three generated
systems evaluated in the paper, a runnable reference implementation, parser
crates for Cypher, GQL, and Gremlin, fixed benchmarks, graph inputs, and
tables of the reported results.

## Repository layout

| Path | Purpose |
|---|---|
| `metagdb_src/metagdb_core/` | Stable traits and intermediate representation. |
| `metagdb_src/metagdb/` | Runtime and command-line interface. |
| `metagdb_src/implementation/metagdb_default/` | Runnable reference implementation. |
| `metagdb_src/implementation/generated/` | Source snapshots of MG-Read, MG-Update, and MG-Alg. |
| `metagdb_src/implementation/language/` | Pre-generated parser crates. |
| `data/language/` | Source grammars. |
| `data/raw/*.graph.gz` | Exact Uniform, RN, and YT evaluation inputs; see `data/README.md`. |
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

The first record of `benchmark/graded/suite240.jsonl` defines the audit graph;
the remaining 240 records contain requests and expected answers.

## Evaluation artifacts

The evaluated source snapshots are `metagdb_exp_lat_01` (MG-Read),
`metagdb_exp_thr_02` (MG-Update), and `metagdb_exp_alg_01` (MG-Alg). Each
contains its Rust source and Cargo manifest. Generation records, design
vectors, and experiment-specific activation files are not part of these
source snapshots. To run a snapshot, first create its local Cypher binding and
then run the framework's build, test, and demo checks:

```bash
python3 scripts/language.py configure metagdb_exp_lat_01 Cypher
python3 scripts/build.py verify metagdb_exp_lat_01
```

Replace `metagdb_exp_lat_01` with either of the other two crate names to
check them. The generated language binding and active Cargo manifests are
local files ignored by Git.

Extract the bundled graphs following `data/README.md`; that page also links
to the original sources for the larger LJ and Orkut graphs. The values shown
in the paper and the measurement context are in `artifacts/results.md`.
