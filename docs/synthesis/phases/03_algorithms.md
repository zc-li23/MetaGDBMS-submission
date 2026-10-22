# Phase 03 — analytical algorithms

**Goal:** implement every algorithm required by `d` over the phase-00 graph
projection.

**Read:** completed `projection.rs`; `l4_compute/{algebra,engine,olap}.rs`; design
`algorithms`; `.synthesis/algorithm_plan.json`; `algorithm_evidence.md`.

**Prerequisite:** phase 02 passed and phase-00 projection is available.

**Actions:** implement one `AnalyticalAlgorithm` per required algorithm without
assuming a framework catalog, register its exact stable name, and validate its
arguments. Keep algorithms independent of query-language syntax. Independently
reason about expected results, tolerances, invariants and failure behavior, then
write a deterministic core test on hand-checkable graphs. Cover applicable empty,
disconnected, directed, weighted, tie and seeded-random cases. Fill `core_test`,
`correctness_summary`, and the performance `arguments` in the matching algorithm
plan row; do not record a passing claim without executable assertions.

If the current algorithm cannot be implemented correctly using the passed
storage/projection contract, diagnose the specific missing operation and use
`revisit --to 02_storage` or `--to 00_foundation` now. Revise that earlier component,
then revalidate and regenerate this algorithm stage; do not defer the mismatch
until final integration.

**Artifacts:** `src/compute/algorithms/` or an equivalent module and tests.

**Gate:** every required algorithm has a distinct executable core test that passes
using the candidate projection and returns typed `ResultStream` values; phase
advance also checks the plan fields.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 03_algorithms --review <candidate>/.synthesis/reviews/03_algorithms.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/03_algorithms`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `04_language_ast`.
