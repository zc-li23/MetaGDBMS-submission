# Phase 08 — physical-plan lowering and optimization

**Goal:** convert framework logical plans into candidate physical operators while
preserving semantics.

**Read:** completed `physical_ops.rs`; `l4_compute/{algebra,expression,engine}.rs`;
`l4_compute/olap.rs`; design `execution`.

**Prerequisite:** phase 07 passed.

**Actions:** implement `MetagdbOptimizer`; recursively lower every required
logical operator and recognized extension payload to physical operators. Apply
only semantics-preserving rewrites. An identity optimization policy is acceptable,
but a missing physical lowering path is not.
This includes projection construction and runtime lookup/execution for every
required `CallAlgorithm`; unknown registered names must produce a precise error.

**Artifacts:** `src/compute/optimizer.rs` and logical-to-physical tests.

**Gate:** all physical variants required by the planner can be produced; nested
trees retain predicates, projections, ordering, DML payloads, and algorithm input.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 08_optimizer --review <candidate>/.synthesis/reviews/08_optimizer.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/08_optimizer`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `09_executor_tests`.
