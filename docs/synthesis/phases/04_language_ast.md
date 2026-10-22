# Phase 04 — private language AST

**Goal:** represent the required language semantics without ANTLR lifetimes or
framework physical types.

**Read:** selected `.g4`; generated parser/lexer summaries; capability matrix;
`l4_compute/engine.rs`; `l5_interaction.rs`; common result/property types. Read
actual generated source only when a public signature is absent from its summary.

**Prerequisite:** phase 00 passed; the language AST does not depend on algorithm implementation.

**Actions:** define the top-level language AST and typed nodes for required
statements, patterns/traversals, clauses/steps, expressions, projections,
modifiers, DML, and calls. Preserve optional grammar parts as `Option<T>` and
source distinctions needed by planning. Gremlin uses a traversal-step hierarchy;
Cypher/GQL use their actual grammar structure. Do not put parsing logic here.
Include an algorithm-call form that can carry any registered name and scalar
arguments required by the open algorithm descriptions in `d`.

**Artifacts:** `src/compute/<language>_ast.rs` implementing `LanguageAst`, plus
AST construction tests.

**Gate:** every required semantic capability maps to an AST form, optional grammar
parts are representable, and AST tests compile and pass.

**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 04_language_ast --review <candidate>/.synthesis/reviews/04_language_ast.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/04_language_ast`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `05_language_lowering`.
