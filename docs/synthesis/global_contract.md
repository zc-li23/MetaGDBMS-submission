# Immutable synthesis contract

## Inputs and authority

The frozen `.synthesis/design_vector.json` is the complete design input. The
Rust traits under `metagdb_src/metagdb_core/src/` are the authoritative component
interfaces. `docs/implementation_contract.md` defines assembly exports. The
selected grammar defines accepted syntax; it does not define execution semantics.
The repository has already compiled each `.g4` into an ANTLR lexer/parser crate
under `metagdb_src/implementation/language/`. That generated crate is the sole
syntax frontend for its language.

When details are absent from `d`, choose the smallest conventional design that
satisfies its stated behavior, record the choice in `decisions.md`, and proceed.
Do not ask the user to make ordinary implementation choices.

## Edit boundary

Edit only `metagdb_src/implementation/generated/<crate>/`. Do not change
`metagdb_core`, `metagdb`, parser crates, grammars, benchmark data, workflow
instructions, activation templates, or fixed tests. Generated activation files
may change only through `scripts/build.py`.

If the fixed framework truly blocks a requirement, first prove it with a minimal
candidate-side attempt and exact compiler/runtime evidence. Record the gap; do
not widen the interface during this experiment.

## Semantic rules

- Construct the repository-generated ANTLR lexer/parser, invoke its language
  entry rule, and lower that CST to a private language AST. Do not interpret the
  `.g4` at runtime or replace it with a handwritten lexer/parser.
- Use actual generated parser APIs and summaries. Never invent ANTLR contexts or
  accessors.
- Keep the generated parser dependency and `antlr4rust` added by `build.py create`.
  The language entry rules are Cypher `oC_Cypher()`, GQL `gqlRequest()`, and
  Gremlin `queryList()`.
- Separate syntax acceptance from semantic coverage: ANTLR decides whether input
  conforms to the grammar; lowering/planning may return a precise unsupported
  error for semantics outside the experiment scope.
- Resolve graph labels, edge types, and property keys through `GraphSchema`.
- Implement required behavior through the public runtime, including error paths.
- Treat every item in `algorithms.required` as an open user-defined algorithm,
  regardless of whether its name appears elsewhere in the repository. Implement
  it through `AnalyticalAlgorithm`, expose it through the selected language, and
  complete the candidate-local algorithm evidence plan.
- Reject unsupported behavior precisely. Silent empty results, fabricated
  defaults, debug-string value encodings, and parser-only claims are failures.
- Storage, recovery, concurrency, isolation, and execution must match `d`.
- ACID is mandatory: multi-operation atomicity, graph invariants, the selected
  isolation level, and crash-safe commit durability must pass the fixed system
  benchmark. A stronger isolation implementation may pass a weaker requested
  profile only when all profile-observable outcomes remain compatible.
- Candidate tests complement the fixed language/system benchmarks and may not
  replace them. For open user-defined algorithm semantics, the two tests named in
  `algorithm_plan.json` are the required executable oracle and must contain real
  expected-value or invariant assertions.

## Experiment integrity

External research notes, sibling directories under `implementation/generated/`,
their `.snapshots/`, and prior run reports are outside the synthesis input. Do
not read, copy, or infer an implementation from them. A candidate must be
derived from its own frozen design, the selected generated parser, fixed
contracts, and evidence created during the current run.

Never weaken, delete, special-case, or rewrite tests or benchmark cases to raise
the score. Do not recognize benchmark query text and return canned answers. Do
not copy the reference implementation wholesale. Reuse public framework types
and conventional algorithms; implement the selected design independently.
Passing a visible benchmark with a benchmark-specific parser is also a failure,
because it disconnects the generated system from the supplied grammar.

Keep `capability_matrix.tsv` current. A capability becomes supported only after
its semantic test passes. Keep the last known good code when a repair regresses
the full test or either benchmark result.

Performance optimization starts only after all correctness gates pass. Treat
`dataset_weights` as the deployment graph mix and `workload` as the operation mix.
Never trade away ACID, recovery, validation, or required semantics for a better
measurement. Use only `scripts/snapshot.py` for candidate snapshots; do not run
experimental commits, resets, or stashes against the outer research repository.
Normal phase snapshots are created automatically by `scripts/synthesis.py advance`;
manual checkpoints are reserved for useful intra-phase rollback points.
