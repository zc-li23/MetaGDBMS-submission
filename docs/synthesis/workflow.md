# Graph-guided synthesis workflow

One persistent coding agent performs 14 implementation nodes. Ordinary code
generation follows prerequisite edges. **At any active generation node C**, a
newly discovered incompatibility with an earlier node A can follow a diagnosed
return edge `C -> A`. A and every affected consumer, including C, lose their
old passing evidence. The agent revises A, then runs the normal gates through
B, C, and the remaining nodes again. This is the generation-time cycle;
integration and performance diagnosis use the same upstream repair mechanism.
Normal prerequisite edges are fixed in `scripts/workflow_graph.py`. Return
edges are selected by the agent only when a concrete incompatibility appears;
`synthesis.py` checks and records each edge at runtime.

```text
00 foundation: schema + physical operators + graph projection
 ├─01 transactions(schema) ─02 storage(transactions, schema)
 ├─03 algorithms(projection, storage after phase 02)
 └─04 language AST
      └─05 ANTLR CST lowering(AST, generated parser summaries)
           └─06 parser(lowering, generated lexer/parser)
                └─07 planner(AST, schema, physical operators)
00 physical operators ────────────└─08 optimizer
00/01/02/03/07/08 ──────────────────09 executor + requirement tests
all component outputs ───────────────10 assembly
                                        └─11 full build integration
                                             └─12 benchmark repair
                                                  └─13 measured performance optimization

Example generation cycle:
02 storage --advance--> 03 algorithms finds storage/OLAP mismatch
02 storage <--revisit-- 03 algorithms (invalidate stale downstream evidence)
02 revised storage --advance--> 03 revised algorithms --> 04 ...
```

The ordering intentionally separates three language-front-end artifacts:

1. the private language AST describes semantics without ANTLR lifetimes;
2. lowering converts the repository's generated ANTLR CST into that AST;
3. `MetagdbParser` constructs the generated lexer/parser, invokes the language's
   top-level rule, and delegates to lowering.

The `.g4` file is the grammar source of truth, but a candidate does not interpret
or reimplement it. It uses the already generated crate under
`metagdb_src/implementation/language/<Language>/`. A handwritten replacement
lexer/parser is outside this workflow, even when it passes the visible benchmark.
Candidate lifecycle and rollback are defined in `snapshots.md`; phase instructions
repeat the required transition so the agent records each completed module even
when it loads only the current phase file.

For each phase in a new graph-workflow run:

1. Confirm all prerequisite phases are passed in `.synthesis/state.json`.
2. Read the design vector, current phase, and only the exact contracts and
   upstream artifacts listed by the phase.
3. Implement the phase outputs without changing upstream interfaces casually.
4. Run focused tests and inspect the artifact itself; command success alone does
   not prove that the required implementation path was used.
5. If the current generated node reveals a defect or mismatch in an earlier
   component, write a `revisit` review naming the earlier target and cross-layer
   diagnosis. `revisit --to` returns directly to the agent-selected earlier
   owner, invalidates the traversed interval and dependent consumers, and
   snapshots the return. Change the earlier
   implementation and re-run each required gate. This is available during
   component generation, not only after final integration.
6. Update the capability matrix and decisions. Write a structured review in
   `.synthesis/reviews/<phase>.json` with observations, self-assessment, remaining
   risks, and next action. `advance --review` reruns a fixed gate: candidate tests
   for component stages, full verification for integration, and correctness suites
   for benchmark/performance stages. It stores the output hash and source revision.
   A failed gate leaves the state unchanged. Inspect the failure and repair in
   this phase first; if the blocking cause belongs to an earlier component,
   diagnose it in a revisit review and return to that owner with step 5.
7. In phase 12 or 13, use the same diagnostic return when a system failure
   belongs to an upstream owner. Correctness and performance stages also
   require their own review; performance hypotheses and accepted/rejected
   measurements are recorded in `decisions.md`.

Existing schema-v1 candidates retain their historical `advance --evidence`
protocol. Their old evidence is not retroactively represented as a v2 review.

Phase 12 uses the correctness feedback loop below. Preserve the immutable design, framework
contracts, generated ANTLR frontend, and benchmark while repairing candidate code.

```text
verify → language + system + algorithm reports → root-cause grouping → focused repair
      → focused tests → full verify → all correctness gates → compare with best
```

`max_fix_iterations: 0` means no iteration limit. A phase passes because its
artifact and evidence satisfy the written gate, not merely because a file exists.

Phase 13 begins only after correctness passes. Performance intent has two axes:
`dataset_weights` represents the expected graph topology and `workload` represents
the operation mix. Their product drives the aggregate score, while per-dataset
guardrails prevent a weighted average from hiding a severe regression. Candidate
algorithm plans add one kernel and one public-language metric for every freely
named algorithm in `d`; optional per-algorithm weights divide those workload
classes without imposing a catalog. Snapshots use `scripts/snapshot.py`; their
Git directory and work tree are both
inside the ignored generated area and all commands explicitly name them, so the
outer research branch, index, and stash remain untouched.
