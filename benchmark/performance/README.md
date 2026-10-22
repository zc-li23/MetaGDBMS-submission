# Performance datasets and profiles

The evaluated Uniform, RN, and YT graphs are distributed as compressed inputs
under `data/raw/`. Extract them before running a benchmark; see `data/README.md`
for commands, provenance, and SHA-256 checksums. LJ and Orkut are not bundled
because of their size. Their upstream download pages and the hashes of the
exact evaluated local inputs are also listed there.

`rn.graph` is a road network with about 1.09M vertices and 3.08M edges. Its
degree is bounded (mean 2.83, maximum 9), so it represents sparse traversal and
large scans. `yt.graph` is a social network with about 1.16M vertices and 2.99M
edges. It is highly skewed (maximum out-degree 28,576), so it covers hotspots and
power-law behavior. The generated regular graphs provide controlled degree and
size curves.

The tracked `.graph.gz` files restore to the manifest paths. The exact
uncompressed files are ignored by Git.

## Scenario and operation weights

The design vector has two independent levels of performance intent:

- `dataset_weights` says which graph shape represents the expected deployment.
  A social service can assign most weight to `yt`; a road-network workload can favor
  `rn`; a topology-neutral experiment can spread weight across uniform graphs.
- `workload` says which operations matter inside every selected graph, such as
  point reads, adjacency expansion, transactions, an algorithm kernel, or the
  same algorithm invoked through the public query language.
  Import, reopen, disk footprint, and projection construction can also receive
  nonzero weight when the user's deployment values startup, space, or analytics.

Both maps sum to `1.0`. The comparator multiplies the two weights and computes a
weighted geometric mean of baseline/candidate latency ratios. It also checks
every dataset separately against regression, disk-size, and reopen guardrails.
Consequently, emphasizing `yt` creates room for social-graph optimization without
allowing an extreme regression on `rn` to disappear inside the aggregate score.

### Sustained edge-insert objective

Designs with `transaction_edge_insert_window` in their workload run an additional
fixed single-thread write window after the ordinary transaction latency probe.
Each window inserts 1,000 new edges into existing vertices, 100 edges per durable
transaction. Endpoint pairs are selected deterministically from the imported
vertex IDs, with distinct pairs across the five windows on the current graphs.
The design's `repeat` gives the number of consecutive windows (five
for `thr_03`). The timer covers begin, edge insertion, and commit; the runner
reports total committed edges per second and elapsed microseconds per edge.
The comparator scores the latter, which is the reciprocal of throughput for the
fixed operation count. It checks every inserted edge's ID and endpoints after
closing and reopening the database; failed readback invalidates the metric.
Import, verification, and reopen are outside the write timer. This is an
internal storage/transaction objective for generation-time tuning. The final
cross-system throughput experiment must use a separate common public-interface
driver with the same 100-edge commit boundary. The old 40/30/30 mixed workload
is not part of this objective.

For a baseline/candidate metric pair, let `r[g,o] = baseline[g,o] / candidate[g,o]`
(larger is faster), graph weight `p[g]`, and operation weight `q[o]`. The
per-graph log gain is `u[g] = sum_o q[o] * ln(r[g,o])`. The new
**topology-robust log gain** is
`S = sum_g p[g] * u[g] - lambda * sum_g p[g] * min(u[g], 0)^2`.
The first term is the log of the existing weighted geometric ratio because
both weight maps sum to one. The second term is a one-sided downside penalty:
an aggregate gain cannot hide an entire graph scenario that gets slower without
paying for the loss. `lambda` is `topology_downside_penalty`; new vectors set it
to `1`, while old vectors without this field retain `lambda = 0`.
`minimum_topology_robust_gain` defaults to zero. When the new score is enabled,
falling below this threshold is a measured, provisionally rejected result that
needs independent confirmation. The report keeps the ordinary geometric ratio,
each `u[g]`, the unpenalized log gain, and the downside term for audit and a
`lambda = 0` ablation. This score is a workload-specific adaptation of log
speedups and downside-risk penalties, not a claim that geometric aggregation or
the penalty idea itself is new.
Passing the threshold means the candidate is acceptable relative to the fixed
baseline; the optimization loop still compares its score with the saved best
before replacing that best. Comparisons need the same frozen design, datasets,
and operation definitions.

## Guardrails are stated against measurement noise

A per-metric "no slower than 1.2x" rule cannot be applied to this benchmark as
written: on a normal developer machine, the *same binary* measured twice spreads
further than that for most metrics, so such a rule fires at random. The
comparator therefore treats a latency guardrail as a measurement problem:

- **Scope.** Hard caps cover the protected metrics (`storage.disk_bytes`,
  `storage.reopen`) and metrics with a nonzero `workload` weight
  (`weighted_latency_regression_max`, default 2.0). A metric the design scores at
  weight zero only has a catastrophic cap
  (`unweighted_latency_regression_max`, default 5.0).
- **Noise floor.** Pass a directory more than once per side
  (`compare-performance ... --baseline B1 --baseline B2 --candidate C1
  --candidate C2`) to measure replicate runs of the same code. Each metric is
  compared on the median across its side's directories, and the larger of the
  baseline and candidate spreads becomes that metric's noise floor. The tolerance is
  `max(configured_cap, noise_floor)`, and rows above the cap but inside the floor
  are reported as `~ within noise floor` instead of as failures.
- **Noise-limited reporting.** When a required metric's same-code spread exceeds
  its cap, the report lists it under `noise_limited` and returns
  `INCONCLUSIVE` (exit 4). This is never a performance pass. After two
  independent runs per side, the workflow may close as
  `completed_with_limitations` only if there are no real or tentative
  failures and the report explicitly says it does not establish an improvement.
- **Reproduce before rejecting.** A measured failure is tentative on the first
  comparison: the exit status is `PROVISIONAL` (3) and the failure appears under
  `tentative_failures`. Re-measure both baseline and candidate into new
  directories, then pass `--confirm-with <previous comparison.json>`; only
  failures reproduced for the same code, design, and dataset contents become
  fatal (`REJECT`, exit 1). Reusing any prior measurement directory is refused.
  Failures that are not measurements -- a metric that did not run, an absolute
  target miss -- are fatal immediately.
- **No net regression.** The weighted geometric mean over scored metrics may not
  fall below `minimum_overall_ratio` (default 1.0), which is what makes a loose
  per-metric cap safe.

Exit codes: `0` accepted, `1` rejected (confirmed failures), `2`
comparison error, `3` provisional (re-measure and confirm), and `4`
inconclusive because a required guardrail is noise-limited.

### A report belongs to one revision

`scripts/build.py micro-benchmark` fingerprints the source it is about to
compile and writes `run.json` into the report directory. The comparator reads it
and refuses a side whose directories carry different fingerprints, because the
median of two different binaries measures neither one. The fingerprint covers every compiled input: the candidate, its algorithm plan,
the stable runner, `metagdb_core`, the selected generated ANTLR crate,
generated Cargo manifests, the resolved dependency lock, and the fixed
per-language query templates. `run.json` also records the exact design hash,
query-template hash, and SHA-256 of every graph input. The comparator refuses mixed
revisions, designs, data contents, repeat counts, or crate names. Directories
written before complete metadata existed are refused by default; use
`--allow-unstamped` only to inspect a legacy result that will not be used as
acceptance evidence.

Replicates of one revision can skip the import with `--reuse-data`: when the
graph, the fingerprint and the design hash all match the stamp beside the scratch
database, the runner reuses it and reports `storage.import.us` as `reused`
instead of re-measuring. The comparator refuses a comparison containing a reused
metric unless `--allow-reused` is passed, so an acceptance decision never rests on
a value carried over from another run.

The candidate code and each scratch database have one owner at a time. The
candidate lock is acquired atomically before activation or compilation and held
until every selected dataset report is written; `snapshot.py restore` and
`restore-best` acquire that same lock before rewriting the work tree. Dataset
scratch locks are also atomic, so two candidates cannot delete one another's
temporary database. The operating system releases both locks automatically if a process exits or
crashes; the lock files persist only as human-readable owner diagnostics.

### Choosing the performance intent

The design vector decides how long phase 13 takes, so pick it deliberately:

- **Weights are the only incentive.** A metric with weight `0` cannot veto and
  barely scores; a metric at `0.02` will not be optimized. Give weight to what the
  research claim is about.
- **`repeat` multiplies the most expensive metric.** Every metric measured with
  one operation per repeat gets exactly `repeat` samples, so raising `repeat` from
  5 to 12 to make `p95` a real percentile multiplies a 20-second metric by 12.
  Raise it when the metrics are cheap; leave it low when the primary metric is a
  whole-graph analysis.
- **A primary metric is re-measured on every run.** Naming a ~20-second metric
  (an end-to-end algorithm call, for instance) as primary means every hypothesis,
  every replicate and every confirmation pays for it. Prefer a cheap primary
  metric and let an expensive one carry score weight instead.
- **Know what an end-to-end metric contains.** `algorithm.<name>.e2e` includes
  projection construction, which can dominate it; quote the split rather than
  attributing the whole number to the algorithm.

### Why the single-operation metrics stay noisy

The runner reports percentiles by nearest rank, so `p95` *is* the slowest sample
until a metric has 12 or more samples. Import, reopen, disk size, scan, commit,
the algorithm metrics, and projection construction are measured with exactly
`repeat` operations, so at the default `repeat: 5` their reported p95 is a
five-sample maximum, and its run-to-run spread exceeds any useful cap. Two
levers change that without touching the fixed runner:

- raise `repeat` in the design vector to at least 12 (the schema allows up to
  20), which makes p95 an actual percentile for those metrics;
- if they remain `noise-limited`, report the measured spread and treat the
  experiment as inconclusive; increase repetitions or improve environmental
  control before asking phase 13 to accept it.

The multi-sample metrics (point reads, adjacency) are unaffected: they already
carry hundreds to thousands of samples.

The fixed runner reports language-independent storage, transaction, and algorithm
measurements plus language-specific end-to-end point and one-hop queries.
Those public queries live in `benchmark/performance/query_templates.json`
rather than in Rust control flow. `build.py` validates each rendered template
with the matching generated grammar before compiling a measurement; GQL uses
`ELEMENT_ID`, Cypher uses `id`, and Gremlin uses `g.V`. This
separates a storage improvement from parser/planner/executor overhead while still
measuring the behavior a user sees.

Algorithm names are not fixed by this benchmark. A synthesis run completes
`.synthesis/algorithm_plan.json` with executable parameters and a selected-language
query for every algorithm requested by `d`. The runner reports
`algorithm.<name>.kernel` and `algorithm.<name>.e2e`; the design's optional
per-algorithm weights divide the generic `algorithm_kernel` and `algorithm_e2e`
workload weights. Semantic correctness is established first by the distinct
agent-authored tests executed through `build.py algorithm-check`.

Adjacency has two measurements. `storage.out_adjacency` uses evenly sampled
vertices; `storage.hotspot_out_adjacency` deterministically selects up to 64
highest-out-degree vertices from the input file. The latter prevents a power-law
graph such as `yt` from looking like a uniform graph merely because a sample
missed its hubs. The runner applies the framework's `.graph` mapping from external
zero-based ID `v` to storage `VertexId(v + 1)`.

Generate a single dataset directly:

```bash
python3 benchmark/performance/generate_uniform.py \
  --output data/raw/uniform_smoke.graph \
  --vertices 1000 --degree 8 --labels 4 --communities 32 --seed 20260918
```

The build driver generates missing synthetic datasets automatically. It never
downloads or commits real datasets.

Run a cheap smoke tier, a controlled standard tier, or all datasets selected by
the design vector:

```bash
python3 scripts/build.py micro-benchmark metagdb_example \
  --design metagdb_src/implementation/generated/metagdb_example/.synthesis/design_vector.json \
  --profile smoke --output-dir target/perf-baseline

python3 scripts/build.py micro-benchmark metagdb_example \
  --design metagdb_src/implementation/generated/metagdb_example/.synthesis/design_vector.json \
  --datasets yt --output-dir target/perf-yt-candidate
```

For a final comparison, baseline and candidate directories must contain every
dataset named by the design vector:

```bash
python3 scripts/build.py compare-performance metagdb_example \
  --design metagdb_src/implementation/generated/metagdb_example/.synthesis/design_vector.json \
  --baseline target/perf-baseline-1 --baseline target/perf-baseline-2 \
  --candidate target/perf-candidate-1 --candidate target/perf-candidate-2
```

An exit code of zero means the primary improvement threshold, absolute targets,
and all regression guardrails passed with no noise-limited required metric.
Exit code 3 requests confirmation of tentative failures. Exit code 4 records a
measurement-limited result: it may close phase 13 with a downgraded conclusion,
but cannot support a performance-improvement claim. For either accepted or
solely noise-limited results, `build.py` writes
`.synthesis/performance_evidence.json`; phase 13 requires two independent
measurements per side and rejects stale evidence after any candidate change.
The detailed JSON report remains under `target/` by default.

The micro-benchmark runner writes all requested TSV reports before returning a
nonzero status when any metric is `ERROR`. This gives the agent concrete query or
component failures to repair without allowing an incomplete run to satisfy the
phase gate.
Imported benchmark databases use a sanitized temporary path under
`target/performance-data/` and are deleted after each report, so baseline and
candidate runs retain only compact evidence files rather than duplicate datasets.
