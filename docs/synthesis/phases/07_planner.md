# Phase 07 — logical planner and binder

**Goal:** translate the private language AST into framework logical algebra.

**Read:** completed language AST; `schema.rs`; `physical_ops.rs` for DML payload
names; `l1_model.rs`; `l4_compute/{algebra,expression,engine}.rs`.

**Prerequisite:** phases 00 and 06 passed.

**Actions:** implement `MetagdbPlanner`; bind label/type/property names through
`GraphSchema`; manage variable and step-label scopes; lower traversal/patterns,
filters, joins, projection, aggregation, ordering, pagination, DML, and algorithm
calls. Use language-independent `LogicalOp`/`Expr` where available and named
extension payloads where necessary. Cover every required top-level AST variant;
return precise unsupported errors for optional semantics not implemented.
Every required algorithm-call AST must produce a `GraphProjection` input and a
`CallAlgorithm` node using the exact registered name; do not bypass the planner
with query-text matching.

**Artifacts:** `src/compute/planner.rs` and AST-to-plan tests.

**Gate:** every required capability has plan-shape/binding evidence; unknown names
obey the selected schema mode; no plan matches raw query strings.

**Triggers:** binding/cardinality failures → `query_semantics.md`.
**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 07_planner --review <candidate>/.synthesis/reviews/07_planner.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/07_planner`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `08_optimizer`.
