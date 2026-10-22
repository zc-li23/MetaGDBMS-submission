# Fixed system benchmark

This suite tests the generated database below the query-language layer. Cypher,
GQL, and Gremlin candidates therefore run the same storage, ACID, recovery, and
algorithm checks. The executable checks are in
`metagdb_src/metagdb/src/system_benchmark.rs`.

The finalized design vector selects the schema mode and isolation level. ACID is
not optional: multi-operation atomicity, graph consistency, the selected
isolation semantics, and crash-safe durability must all pass. Serializable adds
the write-skew case. A candidate passes only when every applicable correctness
case passes; performance is reported separately and has no machine-dependent
threshold.

Run a diagnostic baseline while implementing:

```bash
python3 scripts/build.py system-benchmark <crate> --design <design.json> --diagnostic
```

Run the completion gate without `--diagnostic`:

```bash
python3 scripts/build.py system-benchmark <crate> --design <design.json>
```

Outputs:

- `target/system-benchmark/report.tsv`: one row per applicable correctness case,
  including the exact failure;
- `target/system-benchmark/performance.tsv`: deterministic smoke workload timings
  for loading, point lookup, scan, and adjacency traversal. Open user-defined
  algorithms are checked by `build.py algorithm-check` and measured by the
  performance micro-benchmark because this suite does not own their semantics.

Each correctness case uses a fresh directory. Crash cases start a child process,
write a two-record transaction, terminate without destructors, reopen the same
directory, and verify that recovery retains either all or none of the write set.

The suite is a strong executable conformance oracle for the framework contract,
not a mathematical proof of all database histories. Final paper experiments
should keep additional randomized seeds and run histories outside the input
visible to the generation agent.
