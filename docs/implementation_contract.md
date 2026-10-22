# metaGDB implementation crate contract

This document defines the boundary between the fixed framework (`metagdb` and
`metagdb_core`) and a concrete graph database implementation. The Rust traits in
`metagdb_src/metagdb_core/src/` are authoritative if this guide and code differ.

## Location and naming

- Stable reference implementation: `metagdb_src/implementation/metagdb_default/`
- Agent-generated implementation:
  `metagdb_src/implementation/generated/<crate_name>/`
- The directory name and `[package].name` must be identical.
- Use a lowercase Rust crate name with a `metagdb_` prefix.
- Generated implementations and `scripts/config/<crate_name>.toml` are paired local
  experiment outputs. Both paths are ignored by Git; archive or promote a selected
  result only after its evaluation evidence has been reviewed.

Create an implementation shell with:

```bash
python3 scripts/build.py create metagdb_candidate_01 --language Cypher
```

The shell deliberately contains `compile_error!` until the agent implements the
contract. It never copies the reference implementation.

For a reproducible `.g4 + finalized design vector d -> code` run, use
`python3 scripts/synthesis.py init --design <d.json>` instead. It calls the same
shell creator and adds frozen candidate-local state under `.synthesis/`; follow
`docs/synthesis/README.md` through every phase gate.

## Cargo requirements

The generated Cargo file follows this shape:

```toml
[package]
name = "metagdb_candidate_01"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
metagdb_core = { workspace = true }
metagdb_cypher_antlr = { workspace = true }
antlr4rust = "0.5.2"
```

An implementation depends on `metagdb_core`, never on `metagdb`. The selected
language parser dependency is created automatically by `build.py create`.

## Required public exports

The crate root must export these exact names:

| Public name | Required role |
|---|---|
| `MetagdbImpl` | Owns the schema, storage, transaction manager, and algorithms |
| `MetagdbParser` | Implements `l4_compute::engine::Parser` |
| `MetagdbPlanner` | Implements `l4_compute::engine::Planner` |
| `MetagdbOptimizer` | Implements `l4_compute::engine::Optimizer` |
| `MetagdbExecutor` | Implements `l4_compute::engine::Executor` |
| `CsrProjection` | Builds the standard projection used by the fixed algorithm benchmark |

The four stage types must be unit structs because the generated assembly constructs
them as `Box::new(MetagdbParser)`, `Box::new(MetagdbPlanner)`, and so on.

`CsrProjection` must implement `GraphProjection` and expose:

```rust
pub fn build(
    storage: &dyn StorageEngine,
    ctx: &TxnContext,
    spec: &ProjectionSpec,
) -> Result<Self>;
```

`MetagdbImpl` must expose these fields:

```rust
pub struct MetagdbImpl {
    pub schema: Arc<dyn GraphSchema>,
    pub storage: Arc<dyn StorageEngine>,
    pub txn_manager: Arc<dyn TxnManager>,
    pub algorithms: Vec<Arc<dyn AnalyticalAlgorithm>>,
}
```

It must provide these constructors and entry points:

```rust
impl MetagdbImpl {
    pub fn new() -> Self;

    pub fn with_base_dir(base_dir: PathBuf) -> Self;

    pub fn build_from_graph_file(
        graph_path: PathBuf,
        base_dir: PathBuf,
    ) -> metagdb_core::Result<Self>;

    pub fn demo_test() -> metagdb_core::Result<()>;
}
```

- `with_base_dir` backs the persistent interactive shell.
- `build_from_graph_file` backs `cargo run -- --build <file.graph>`.
- `.graph` files use zero-based external vertex IDs. Import maps vertex `v` to
  `VertexId(v + 1)` because `VertexId(0)` is reserved for storage-assigned IDs;
  edge endpoints must use the same mapping.
- `demo_test` must create an isolated runtime, insert deterministic data, execute
  representative requirements, assert results, and return an error on failure.

The framework supplies `LanguageFrontend` for the language selected in
`scripts/config/<crate_name>.toml`; implementations do not export a frontend.

## Assembly behavior

`python3 scripts/build.py use <crate_name>` renders three ignored activation files:

```text
Cargo.toml
metagdb_src/metagdb/Cargo.toml
metagdb_src/metagdb/src/assembly/active.rs
```

The assembly injects the implementation components into the stable runtime,
registers every item in `algorithms`, and provides the selected language frontend.
It also delegates demo, persistent-open, and graph-file import to the three methods
above. Do not edit rendered activation files directly.

## End-to-end workflow

```bash
python3 scripts/build.py create metagdb_candidate_01 --language Cypher
# implement metagdb_src/implementation/generated/metagdb_candidate_01/
python3 scripts/build.py verify metagdb_candidate_01
python3 scripts/build.py benchmark metagdb_candidate_01 --min-pass-rate 0
python3 scripts/build.py system-benchmark metagdb_candidate_01 \
  --design metagdb_src/implementation/generated/metagdb_candidate_01/.synthesis/design_vector.json \
  --diagnostic
python3 scripts/build.py micro-benchmark metagdb_candidate_01 \
  --design metagdb_src/implementation/generated/metagdb_candidate_01/.synthesis/design_vector.json \
  --profile smoke
```

`verify` performs all required repository checks:

1. activates the implementation;
2. checks the complete workspace;
3. runs all workspace tests;
4. runs `cargo run -- --demo` with a bounded timeout.

After `verify` succeeds, run the independent language benchmark. Its complete
machine-readable report is written to `target/benchmark-run/report.tsv`; use that
report to repair the candidate without changing benchmark files. The default
benchmark command requires a 90% pass rate. Candidate-authored tests and demo
output are development evidence, not the final oracle. The system benchmark
requires every applicable ACID, graph consistency, recovery, and algorithm case;
`--diagnostic` only suppresses the nonzero exit while the agent repairs failures.

The micro-benchmark uses the same public constructors to measure storage point
reads, adjacency, scans, transaction commit, projection/open algorithms, reopen, disk size,
highest-degree hotspot adjacency, and language-specific point and one-hop queries.
It imports graph files through
`MetagdbImpl::build_from_graph_file`, so a candidate does not provide benchmark-
specific results. Synthetic data is regenerated from the tracked manifest; large
real graphs remain local and are verified by SHA-256. See
`benchmark/performance/README.md` for scoring and
`docs/synthesis/snapshots.md` for the isolated candidate lifecycle.
