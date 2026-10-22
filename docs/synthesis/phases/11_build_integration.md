# Phase 11 — full build integration

**Goal:** prove the candidate works inside the fixed metaGDB runtime.

**Read:** candidate code, implementation contract, current state, and actual
compiler/test/demo output. Do not reread unrelated phase recipes.

**Prerequisite:** phase 10 passed.

**Actions:** run `python3 scripts/build.py verify <crate>`. Fix candidate-local
compile, workspace test, demo, warning, persistence-path, and assembly failures.
Rerun the full command after focused checks. Do not edit rendered activation
files directly.

**Edit scope:** candidate crate and synthesis metadata only.

**Artifacts:** clean full verification evidence and updated capability matrix.

**Gate:** `verify` exits zero and compiler output satisfies the warning policy.

**Triggers:** Rust/API → `rust_interfaces.md`; storage/demo → relevant playbook;
repeated regressions → `regression_control.md`.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py
advance <crate> --phase 11_build_integration --review <candidate>/.synthesis/reviews/11_build_integration.json`.
A successful transition automatically commits candidate code and `.synthesis`
state as `checkpoints/11_build_integration`. For an intra-phase risky change, use
`scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `12_benchmark_fix`.
