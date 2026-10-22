# Phase 00 — input freeze and independent foundations

**Goal:** freeze the experiment and implement the historical batch-0 components:
schema, physical operator types, and graph projection.

**Read:** frozen design vector; capability matrix; `global_contract.md`;
`docs/implementation_contract.md`; common property/ID types; `l1_model.rs`;
`l2_storage.rs`; `l3_txn.rs`; `l4_compute/{algebra,expression,engine,olap}.rs`.
Also inspect the selected generated parser summaries to plan later front-end files,
without implementing parsing in this phase.

**Prerequisite:** `scripts/synthesis.py init` completed.

**Actions:** map every required capability to a component and test; define the
candidate module tree; implement `GraphSchema` according to `d`; define all
candidate physical operator and DML payload types needed downstream; implement a
storage-backed `GraphProjection`. Preserve the generated parser dependencies in
the candidate `Cargo.toml`. Replace the shell `compile_error!` with honest,
compile-valid modules rather than successful placeholder behavior.

**Edit scope:** candidate schema, physical-operator, projection, module-map and
focused-test files; synthesis metadata.

**Artifacts:** `src/schema.rs`, `src/compute/physical_ops.rs`,
`src/compute/projection.rs`, module map, capability rows, initial decisions.

**Gate:** schema tests pass; physical payloads use framework types rather than
redefining them; projection has a known-answer test; `cargo check -p <crate>`
succeeds; every requirement has an owner and proposed oracle.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 00_foundation --review <candidate>/.synthesis/reviews/00_foundation.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/00_foundation`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `01_transactions`.
