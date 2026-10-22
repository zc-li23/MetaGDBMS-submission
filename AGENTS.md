# metaGDB coding-agent contract

## Objective

Turn the user's graph-language grammar and system requirements into one runnable
implementation crate without changing the framework that all experiment groups
share.

Use `docs/synthesis/README.md` as the single workflow entry, whether the request
starts from natural language, a paper, or a finalized design vector. It refines
the contract below into reviewed phase transitions and a generation-time
upstream repair loop. Do not ask
the user to supervise routine phase transitions or select benchmark fixes.
A short launch request does not reduce this scope: the repository workflow, not
the wording length of the user's prompt, defines every required phase and gate.

## Read before editing

1. `README.md`
2. `docs/implementation_contract.md`
3. The authoritative traits and IR under `metagdb_src/metagdb_core/src/`
4. The requested `.g4` file and the matching generated parser crate
5. `docs/synthesis/README.md` for the workflow and phase documentation

A grammar defines accepted syntax, not executable semantics. Do not claim full
language support from parser coverage alone. Map each requested feature to a
semantic test, and report unsupported grammar alternatives explicitly.

Use `metagdb_src/implementation/metagdb_default/` only to understand integration
patterns. The candidate must live in its own directory and must satisfy the user's
requirements independently.
Do not inspect external research notes, another generated candidate, its
snapshots, or prior reports while synthesizing a candidate from a finalized
design vector.

## Required workflow

1. For a new run from requirements or a paper, use
   `python3 scripts/synthesis.py start --text <requirements.txt> --crate <crate> --run-id <id>`
   (or `--paper` / `--requirements`). This keeps requirement intake, the checked
   design vector, and implementation in one candidate workflow. When a finalized
   vector is supplied, validate and initialize with
   `python3 scripts/synthesis.py init --design <d.json>`.
2. Follow the current instruction under `docs/synthesis/phases/` when synthesis
   state exists. For new graph-workflow runs, write the current structured
   self-review in `.synthesis/reviews/<phase>.json`, then use `advance --review`.
   The workflow reruns fixed checks before advancing. A failed gate keeps the
   current stage active: inspect the evidence, repair locally, and rerun it.
   If local repair cannot resolve a diagnosed mismatch owned by an earlier
   component, write a revisit review and use `revisit --to` to return to that
   owner and invalidate affected phases. The same mechanism handles
   late integration failures. Existing v1 candidates retain their original phase protocol.
   The supplied `.g4` is already compiled under
   `metagdb_src/implementation/language/`: use that generated ANTLR lexer/parser
   and lower its CST. Do not write a replacement lexer/parser for the benchmark.
   After each gate passes, use `scripts/synthesis.py advance`; it automatically
   records candidate code and `.synthesis` state in the isolated named phase
   snapshot. Read `docs/synthesis/snapshots.md` for intra-phase rollback.
3. Implement only under `metagdb_src/implementation/generated/<crate>/` unless
   the user explicitly requests a framework or grammar change.
4. Keep `metagdb_core`, `metagdb`, language parser crates, and existing tests
   unchanged. Treat them and every suite under `benchmark/` as the experiment's
   fixed interface.
5. Export every type and constructor listed in
   `docs/implementation_contract.md`.
6. Put deterministic unit and integration tests in the candidate crate. Its
   zero-argument `MetagdbImpl::demo_test()` must exercise the requested behavior
   and return an error when an assertion fails.
   Algorithms are open user requirements rather than a framework catalog. For
   each requested algorithm, complete `.synthesis/algorithm_plan.json`, write one
   direct correctness test and one distinct public language-pipeline test, and
   independently reason about their asserted expected results.
7. Run `python3 scripts/build.py verify <crate>`. Continue fixing the candidate
   until this command succeeds.
8. Run `python3 scripts/build.py benchmark <crate> --min-pass-rate 0` to inspect
   every unsupported or incorrect case in `target/benchmark-run/report.tsv`.
   Continue implementing missing semantics, then run the benchmark with its
   threshold declared by `d` before reporting completion. Read the complete
   report, group failures by root cause, and continue without user intervention.
9. Run `python3 scripts/build.py system-benchmark <crate> --design
   <candidate>/.synthesis/design_vector.json --diagnostic`, read the complete
   ACID/storage/algorithm report, and repair every applicable failure. Re-run
   without `--diagnostic`; completion requires 100% of applicable system cases.
10. Run `python3 scripts/build.py algorithm-check <crate> --design
    <candidate>/.synthesis/design_vector.json`. Every declared core and language
    test must exist and pass; a registered algorithm that cannot be called from
    the selected language is incomplete.
11. After every correctness gate passes, run phase 13. Measure all graph scenarios
    selected by `performance.datasets`, optimize according to the product of
    `dataset_weights` and `workload`, including every required algorithm's kernel
    and end-to-end query, and accept a change only when the canonical
    comparison passes every regression guardrail. Final phase evidence requires
    two fresh measurements per side and `.synthesis/performance_evidence.json`
    bound to the current revision. `ACCEPTED` supports a performance claim.
    A result that is solely noise-limited may close as
    `completed_with_limitations` only when no real or tentative failure
    remains and the final report explicitly declines a performance-improvement
    claim. Use `scripts/snapshot.py` for candidate
    snapshots; never commit, reset, or stash experiments in the outer research
    repository.

## Evidence and scope

- Use compiler and test output as evidence; do not report success from code
  inspection alone.
- Do not weaken or delete tests to make the implementation pass.
- Do not edit benchmark queries or expected results to improve a candidate's
  score. Benchmark changes are framework changes and require independent review.
- Do not hide unsupported behavior behind silent fallbacks. Return a precise
  error for functionality outside the requested scope.
- Do not modify `metagdb_default`; it is a stable reference and smoke-test target.
- Keep the generated crate and `scripts/config/<crate>.toml` together as local
  experiment outputs. Both paths are ignored by Git; archive or promote a selected
  result only when the experiment owner explicitly requests it.

## Completion report

Report the generated crate name, requirement coverage, commands executed, test
and demo results, language score, ACID/system category scores, weighted
performance score, per-dataset guardrails, optimization decisions, and known
unsupported cases.
