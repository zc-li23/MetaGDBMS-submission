# Phase 01 — transaction manager

**Goal:** implement transaction lifecycle, concurrency control, and isolation
selected by `d`.

**Read:** completed `schema.rs`; `l2_storage.rs`; `l3_txn.rs`; design
`transactions`; phase-00 decisions. Do not load language-front-end instructions.

**Prerequisite:** phase 00 passed.

**Actions:** implement begin, read/write visibility, commit, rollback, conflict
handling, and invalid lifecycle errors. Define the concrete transaction state the
storage phase will consume. Do not label a global mutex as MVCC/2PL/OCC unless its
observable semantics satisfy that choice.

**Artifacts:** `src/txn.rs` and deterministic visibility/conflict tests.

**Gate:** multi-operation atomicity, selected isolation behavior, read-only
behavior, commit, abort, and conflict tests pass; exact framework trait
signatures compile. ACID is mandatory in the final fixed system benchmark.

**Triggers:** visibility/conflict failures → `storage_transactions.md`.
**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 01_transactions --review <candidate>/.synthesis/reviews/01_transactions.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/01_transactions`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `02_storage`.
