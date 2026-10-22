# Regression control

Record the best full verify result and benchmark score before a repair. Work on
one failure cluster, add a focused regression test, then rerun full verification
and benchmark. Revert a change that lowers the score or breaks previously passing
requirements unless it exposes an incorrect oracle assumption. If the same cause
survives twice, stop repeating local patches: trace the full data flow, reconsider
the abstraction, and replace the faulty approach. Never weaken the oracle.

## Performance guardrails are measurements

Correctness regressions are deterministic: a failing case is a failing case. A
performance regression is a *measurement*, and a measurement smaller than the
environment's own run-to-run spread cannot be distinguished from noise. Before
treating a latency difference as a regression:

- **Reproduce it.** A single pair of runs is not evidence. `compare-performance`
  reports `PROVISIONAL` (exit code 3) for measured failures and only makes them
  fatal when a second, independent comparison shows the same dataset and metric
  (`--confirm-with`). Re-measure both sides into new directories; the
  comparator rejects reused directories and mismatched code, design, or data.
- **Compare against the noise floor.** Pass replicate directories for either
  side (`--baseline DIR --baseline DIR ...`). Each metric is compared on the
  median across its side's runs and the baseline spread is that metric's noise
  floor; the tolerance is `max(configured_cap, noise_floor)`. A difference marked
  `~ within noise floor` is not a regression.
- **Say when you cannot tell.** A metric whose same-code spread exceeds the
  configured cap is reported as `noise-limited`: the guardrail could not be
  evaluated for it. Report that as an environment limitation, never as a pass or
  a failure.
- **Respect weights.** A metric the design scores at weight zero only has a
  catastrophic cap; it must not veto an experiment it does not participate in.
  The weighted overall ratio still has to hold, so a local win cannot pay for a
  net loss across the declared workload.
- **Compare one experiment identity at a time.** Every report directory names
  the complete compiled-source fingerprint, exact design, repeat count, crate and
  graph-content hashes in `run.json`; the comparator refuses a mismatch.
  Re-measure instead of reusing an older directory as a replicate.
- **Never accept a carried-over value.** `--reuse-data` reports
  `storage.import.us` as `reused`; such a comparison is exploratory only, and the
  comparator refuses it without `--allow-reused`.

Never obtain a speedup by removing durability, validation, locking, recovery, or
semantics the design requires: the correctness gates catch that before the
performance stage, and a catastrophic regression is still rejected.
