# Phase 05 — generated ANTLR CST to private AST

**Goal:** convert real contexts from the repository's generated ANTLR parser into
the phase-04 AST.

**Read:** completed `<language>_ast.rs`; `antlr_and_lowering.md`; selected `.g4`;
generated lexer/parser summaries; `l4_compute/engine.rs`. Use the summary's exact
Context, `ContextAll`, `*Attrs`, token, and accessor names. Inspect generated source
for uncertain signatures instead of inventing APIs.

**Prerequisite:** phase 04 passed.

**Actions:** create `<language>_lowering.rs` with a root lowering function and
helpers for required CST alternatives. Convert `Option<Rc<Context>>` with
`.as_ref()` and preserve optional fields. Handle every relevant root branch. For
GQL, traverse `gqlRequest → gqlProgram → program/transaction activity → procedure
body → statement`, including pure queries routed through the data-modifying branch.
For Gremlin, dispatch `ContextAll` variants explicitly and recurse through the
preceding `chainedTraversal` before appending the current step. Unsupported
semantics must produce a precise error after the generated grammar has parsed the
source; never introduce a replacement tokenizer or parser.
For every algorithm required by `d`, lower the language's callable syntax into a
private algorithm-call AST node carrying the registered name and arguments. A
grammar accepting `CALL`/`call()` is not evidence until this lowering exists.

**Artifacts:** `src/compute/<language>_lowering.rs` and focused CST-to-AST tests.

**Gate:** representative queries are first accepted by the generated top-level
grammar rule and then lower to the expected AST; root alternatives, nesting,
precedence, DML, optional constructs, and unsupported paths have tests.

**Triggers:** missing context/accessor → `antlr_and_lowering.md`.
**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 05_language_lowering --review <candidate>/.synthesis/reviews/05_language_lowering.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/05_language_lowering`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `06_parser`.
