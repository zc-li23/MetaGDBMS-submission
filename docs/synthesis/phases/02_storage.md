# Phase 02 — storage, indexes, and recovery

**Goal:** implement the physical storage design and integrate the concrete
transaction state from phase 01.

**Read:** completed `schema.rs` and `txn.rs`; `l2_storage.rs`; `l3_txn.rs`; design
`storage`; graph-file import contract.

**Prerequisite:** phase 01 passed.

**Actions:** implement vertex/edge records, stable ID allocation, both adjacency
directions, declared indexes, scans, mutations, and import. Implement the specified
persistence/recovery protocol and read-your-writes/visibility integration. Helper
files such as WAL, page, index, or cache modules are allowed when required by `d`.

**Artifacts:** `src/storage.rs` or `src/storage/mod.rs` plus helpers and tests.

**Gate:** CRUD, referential and adjacency consistency, conflict integration,
clean reopen, committed recovery, and incomplete-transaction recovery tests
pass; `cargo check -p <crate>` succeeds. Edge endpoints/types are immutable;
vertex deletion must reject incident edges or remove them without dangling
references.

**Triggers:** persistence/visibility failures → `storage_transactions.md`.
**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 02_storage --review <candidate>/.synthesis/reviews/02_storage.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/02_storage`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `03_algorithms`.
