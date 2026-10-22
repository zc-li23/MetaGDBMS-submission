# System benchmark

The system benchmark checks storage, transactions, recovery, and algorithms
through `metagdb_src/metagdb/src/system_benchmark.rs`. It uses the schema mode
and isolation level in the selected design vector.

Run it with:

```bash
python3 scripts/build.py system-benchmark <crate> --design <design.json>
```

The command writes correctness results to `target/system-benchmark/report.tsv`
and timing measurements to `target/system-benchmark/performance.tsv`.
