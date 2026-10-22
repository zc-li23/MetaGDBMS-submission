# Open algorithm synthesis and evidence

`algorithms.required` is an open extension point. The framework never restricts
algorithm names to a catalog. Prefer an object with a stable registered name and
a concrete semantic description; a string remains a legacy shorthand.

For every requested algorithm, initialization creates one row in
`.synthesis/algorithm_plan.json`. This file is saved with candidate snapshots but
is authored by the synthesis agent because an arbitrary user algorithm has no
framework-owned correctness oracle. The agent must complete:

- `core_test`: the exact Rust test name that executes the algorithm directly on
  a hand-checkable graph and asserts independently reasoned expected results;
- `language_test`: a distinct exact test name that invokes the algorithm through
  the public Parser → Planner → Optimizer → Executor path;
- `correctness_summary`: the expected result, invariant, tolerance, and important
  edge cases used to judge correctness;
- `arguments`: scalar parameters for performance execution;
- `language_query_template`: a runnable query in the selected language;
- `allow_empty_result`: whether an empty performance result is legitimate.

Performance arguments may use `$hotspot_vertex`, `$first_vertex`, or
`$vertex_count`. Query templates use `{{hotspot_vertex}}`, `{{first_vertex}}`,
and `{{vertex_count}}`. All other strings are passed literally.

The fixed command below verifies that both declared tests really exist and runs
each one. It does not interpret an algorithm name or replace the agent-authored
semantic assertions:

```bash
python3 scripts/build.py algorithm-check <crate> --design <design.json>
```

The micro-benchmark then discovers every completed plan entry and reports
`algorithm.<name>.kernel` and `algorithm.<name>.e2e`. The first metric reuses a
storage-backed graph projection and calls `AnalyticalAlgorithm::execute`; the
second runs the language query through the public runtime. A missing registration,
failed query, unsupported argument, or unexpected empty result is an explicit
`ERROR`, never a zero-duration success.

This protocol deliberately permits algorithms that the current projection or
argument contracts cannot express. In that case the implementation must return a
precise error and the required-algorithm gate remains failed. The synthesis agent
may improve the candidate but may not edit the framework during a finalized run.
