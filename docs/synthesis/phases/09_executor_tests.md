# Phase 09 — executor and requirement tests

**Goal:** execute candidate physical plans and build independent requirement
tests before final assembly.

**Read for executor:** completed physical operators, transactions, projection,
schema and storage; `l1_model.rs`; `l2_storage.rs`; `l3_txn.rs`;
`l4_compute/{algebra,expression,engine,olap}.rs`.

**Read for tests:** completed schema, transaction and storage modules; frozen
design; capability matrix; fixed benchmark categories. Tests must exercise the
public Parser → Planner → Optimizer → Executor path, not internal shortcuts.

**Prerequisite:** phases 02, 03, 07, and 08 passed.

If executor implementation exposes an incompatible upstream storage, algorithm,
plan, or operator contract, use `revisit --to <earlier-phase>` during this stage and
revalidate its consumers before continuing.

**Actions:** implement `MetagdbExecutor` using the selected execution model and
typed results. Preserve scope, duplicates, ordering, nulls, path labels,
side-effects, mutation atomicity, and transaction errors. In parallel component
terms, create deterministic parser/lowering, storage restart, transaction,
semantic integration, algorithm and demo cases. Do not derive expected results
from the implementation under test.
For every requested algorithm, add a public-pipeline language test distinct from
its phase-03 core test. Complete `language_test` and a runnable
`language_query_template` in `.synthesis/algorithm_plan.json`; use the documented
dataset placeholders where an input vertex is needed. Check the language result
against independently reasoned values or invariants rather than merely checking
for successful execution.

**Artifacts:** `src/compute/executor.rs`, `src/compute/mod.rs`, `src/test.rs` and
focused tests.

**Gate:** representative end-to-end public-pipeline tests pass for every required
capability, including empty, duplicate, ordered, mutation, error, restart and
transaction cases. The algorithm plan must be complete and every required
algorithm must have distinct core and language tests.

**Triggers:** semantics → `query_semantics.md`; persistence →
`storage_transactions.md`.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py
advance <crate> --phase 09_executor_tests --review <candidate>/.synthesis/reviews/09_executor_tests.json`.
A successful transition automatically commits candidate code and `.synthesis`
state as `checkpoints/09_executor_tests`. For an intra-phase risky change, use
`scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `10_assembly`.
