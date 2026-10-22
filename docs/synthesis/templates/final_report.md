# Final synthesis report: `{{CRATE_NAME}}`

- Language: `{{LANGUAGE}}`
- Grammar: `{{GRAMMAR_PATH}}`
- Required benchmark threshold: `{{BENCHMARK_THRESHOLD}}`
- `verify` result: pending
- Language benchmark result: pending
- System benchmark result (A/C/I/D and algorithms): pending
- Agent-authored algorithm core/language evidence: pending
- Per-algorithm kernel/end-to-end performance: pending
- Performance datasets and weights: pending
- Weighted primary improvement and guardrails: pending
- Comparison status and two-or-more replicates used per side: pending
- Performance conclusion: accepted, or inconclusive with no performance claim
- Optimization iterations (accepted/provisional/rejected): pending
- Compiler warnings: pending

## Performance

State the measured result; `synthesis.py advance --phase 13_performance_optimization`
refuses a placeholder section. Required content:

- dataset weights and operation weights actually used;
- one full measurement run per side with its `run.json` revision, and the
  replicates the comparison consumed;
- accepted, provisional, and rejected hypotheses, with the measured numbers;
- per-dataset guardrail outcome, including every metric reported as
  `within noise floor` or `noise-limited`, and the measured spread behind them;
- remaining performance limits and the largest absolute cost each dataset pays
  (for example import or a projection rebuild).

## Requirement coverage

Complete from `capability_matrix.tsv` after all gates pass.

## Validation evidence

Record exact commands and results.

## Known unsupported behavior

List explicit unsupported errors. Do not describe parsed-only syntax as supported.
