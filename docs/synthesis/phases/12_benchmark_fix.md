# Phase 12 — autonomous benchmark repair

**Goal:** meet the external semantic benchmark and finalize reproducible evidence.

**Read:** frozen design, global contract, full language and system reports under
`target/benchmark-run/` and `target/system-benchmark/`, candidate code/tests,
capability matrix, and only symptom-matched playbooks. Stop using synthesis phase
recipes to guess fixes.

**Prerequisite:** phase 11 passed.

**Actions:**

1. Run `python3 scripts/build.py benchmark <crate> --min-pass-rate 0`.
2. Run `python3 scripts/build.py system-benchmark <crate> --design
   <candidate>/.synthesis/design_vector.json --diagnostic`.
3. Run `python3 scripts/build.py algorithm-check <crate> --design
   <candidate>/.synthesis/design_vector.json` and inspect every core/language row.
4. Read all complete reports; group failures by semantic root cause and component.
5. Save the best score/evidence. Fix one coherent group in candidate code and add
   a focused regression test.
6. Run focused tests, full `verify`, and all diagnostic checks. Restore
   or repair regressions rather than accepting a lower best score.
7. Repeat without asking the user what to fix until the language threshold and
   every applicable system case pass.
8. Run the language benchmark at its declared threshold, the system benchmark
   without `--diagnostic`, `algorithm-check`, final `verify`, and all checks once more.
9. Record exact correctness results and honest unsupported behavior in
   `final_report.md`; performance evidence is completed in phase 13.

**Edit scope:** candidate crate and `.synthesis/` reports only.

**Artifacts:** passing benchmark, final verification, capability matrix, report.

**Gate:** every correctness criterion in `docs/synthesis/README.md` passes. Advance
this phase only after the final regression run. Otherwise continue. A concrete missing
tool/service or exhausted explicit nonzero budget may produce a blocker report;
ordinary failures, low scores, and uncertainty are not blockers.

**Triggers:** choose from `playbooks/README.md` using current evidence.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py
advance <crate> --phase 12_benchmark_fix --review <candidate>/.synthesis/reviews/12_benchmark_fix.json`.
A successful transition automatically commits the correct candidate and
`.synthesis` state as `checkpoints/12_benchmark_fix`. Phase 13 then marks this
checkpoint as `best`. For an intra-phase risky repair, use
`scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `13_performance_optimization`.
