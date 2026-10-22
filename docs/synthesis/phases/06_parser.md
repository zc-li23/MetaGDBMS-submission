# Phase 06 — parser wrapper

**Goal:** connect source text to the generated ANTLR frontend and phase-05
lowering through the framework `Parser` trait.

**Read:** completed AST and lowering files; generated lexer/parser summaries;
`antlr_and_lowering.md`; `l4_compute/engine.rs`; candidate `Cargo.toml`.

**Prerequisite:** phase 05 passed.

**Actions:** keep the generated language crate and `antlr4rust` dependencies made
by `build.py create`. Implement only this pipeline:

```text
InputStream → generated Lexer → CommonTokenStream → generated Parser
            → language top-level rule → lower_<language>(root.as_ref())
```

Use `BailErrorStrategy` and map ANTLR failures to `Error::Parse`. Top-level rules
are `oC_Cypher()` for Cypher, `gqlRequest()` for GQL, and `queryList()` for
Gremlin. Do not create candidate token kinds, a candidate lexer, a recursive-
descent parser, regex parsing, or a benchmark-specific fallback parser.

**Artifacts:** `src/compute/parser.rs`; module exports; parser integration tests.

**Gate:** positive, negative, and grammar-valid queries outside the visible
benchmark pass the parser integration tests, with precise `Error::Parse` failures
for invalid input.

**Triggers:** generated API errors → `antlr_and_lowering.md`.
**Phase snapshot:** after the gate passes, run `python3 scripts/synthesis.py advance <crate> --phase 06_parser --review <candidate>/.synthesis/reviews/06_parser.json`. A successful transition automatically commits candidate code and `.synthesis` state as `checkpoints/06_parser`. For an intra-phase risky change, use `scripts/snapshot.py checkpoint`; never use the outer research Git.

**Next:** `07_planner`.
