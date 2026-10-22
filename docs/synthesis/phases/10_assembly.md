# Phase 10 — complete candidate assembly

**Goal:** wire every completed component into the exact framework-facing crate
contract.

**Read:** all candidate component outputs; `docs/implementation_contract.md`;
`scripts/templates/assembly.rs.tmpl` for call sites; type names from the actual
source files rather than guessed names.

**Prerequisite:** phase 09 passed.

**Actions:** export `MetagdbImpl`, `MetagdbParser`, `MetagdbPlanner`,
`MetagdbOptimizer`, and `MetagdbExecutor`; implement exact fields, constructors,
persistent open, graph import, algorithm registration and self-checking demo.
Ensure module declarations include the private AST and ANTLR lowering modules.
Do not replace finished components during assembly.

**Artifacts:** `src/lib.rs`, final module exports, candidate README and demo.

**Gate:** `cargo test -p <crate>` passes; all five exports and constructors compile;
the capability matrix has executable evidence for each required row.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py
advance <crate> --phase 10_assembly --review <candidate>/.synthesis/reviews/10_assembly.json`. A
successful transition automatically commits candidate code and `.synthesis`
state as `checkpoints/10_assembly`. For an intra-phase risky change, use
`scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `11_build_integration`.
