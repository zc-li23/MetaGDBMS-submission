# Evaluation results

These tables transcribe the numerical inputs of the figures in the current paper. Values retain the precision of the paper figure inputs. All latency values are in milliseconds. `n/a` means that no value was reported; `limit` means that the experiment reached the stated time limit without a result. The two statuses are not zero-valued measurements.

The measurements used Ubuntu 20.04.6, an Intel Xeon E5-2630 v4 (20 logical CPUs), 314 GiB RAM, and a single-threaded client. External systems were Neo4j Community 5.26.31, Kuzu 0.11.3, Memgraph 2.13.0, and TuGraph Community 4.5.2.

## YT basic operations

| System | Point read p95 | Outgoing neighbors p95 | Edge insert p95 | Edge update p95 | Edge delete p95 |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.136 | 0.166 | 2832.936 | 2.725 | 1.594 |
| MG-Update | 0.143 | 0.249 | 0.685 | 0.650 | 0.801 |
| Kùzu | 0.300 | 2.588 | 3.581 | 4.684 | 4.023 |
| Neo4j | 13.953 | 17.546 | 73.529 | 45.141 | 45.072 |
| Memgraph | 3.065 | 3.289 | 6.216 | 3.517 | 3.402 |
| TuGraph | 4.301 | 5.605 | 7.229 | 6.571 | 7.094 |

## YT graph queries

| System | 5-cycle p95 | Path p95 | Tree branch p95 |
| --- | --- | --- | --- |
| MG-Read | 1.81 | 6.15 | 23.86 |
| MG-Update | 8.99 | 2182.34 | 77.35 |
| Kùzu | 37.61 | 58.89 | 36.57 |
| Neo4j | 47.54 | 177.53 | 38.39 |
| Memgraph | 7.45 | 4868.27 | 12.25 |
| TuGraph | 40.96 | 1304.50 | 900000.00 (limit) |

## YT analysis queries

| System | BFS total | WCC total | PageRank total |
| --- | --- | --- | --- |
| MG-Alg | 3162.11 | 3181.84 | 3443.79 |
| Kùzu | 4042.84 | 4749.65 | 4495.41 |
| Neo4j | 34408.95 | 7465.96 | 8871.92 |
| Memgraph | 34304.86 | 34272.75 | 34941.42 |
| TuGraph | 29828.35 | 29803.91 | 30220.62 |

## Five-graph comparison

Each graph table reports p95 latency. The 5-cycle panel in the paper omits Orkut because no system completed that query within the time limit.

### Uniform

| System | Point read | Outgoing neighbors | 5-cycle | Path | Tree branch |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.152 | 0.235 | 451.47 | 0.13 | 8022.69 |
| MG-Update | 0.148 | 0.330 | 3823.96 | 211.22 | 29818.07 |
| Kùzu | 0.295 | 1.175 | 47.19 | 3.16 | 1217.51 |
| Neo4j | 11.844 | 17.311 | 798.09 | 109.82 | 5929.34 |
| Memgraph | 1.920 | 3.749 | 211.50 | 34.55 | 4049.83 |
| TuGraph | 5.222 | 5.889 | 16453.27 | 129.84 | 900000.00 (limit) |

### RN (Pennsylvania road network)

| System | Point read | Outgoing neighbors | 5-cycle | Path | Tree branch |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.147 | 0.175 | 0.49 | 0.04 | 4.06 |
| MG-Update | 0.166 | 0.228 | 3.90 | 0.75 | 15.20 |
| Kùzu | 0.304 | 2.490 | 34.56 | 1.46 | 27.69 |
| Neo4j | 13.814 | 16.429 | 62.64 | 31.39 | 30.36 |
| Memgraph | 0.947 | 0.752 | 15.84 | 6.08 | 14.79 |
| TuGraph | 3.491 | 6.141 | 15.85 | 6.19 | 2505.85 |

### YT

| System | Point read | Outgoing neighbors | 5-cycle | Path | Tree branch |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.136 | 0.166 | 1.81 | 6.15 | 23.86 |
| MG-Update | 0.143 | 0.249 | 8.99 | 2182.34 | 77.35 |
| Kùzu | 0.300 | 2.588 | 37.61 | 58.89 | 36.57 |
| Neo4j | 13.953 | 17.546 | 47.54 | 177.53 | 38.39 |
| Memgraph | 3.065 | 3.289 | 7.45 | 4868.27 | 12.25 |
| TuGraph | 4.301 | 5.605 | 40.96 | 1304.50 | 900000.00 (limit) |

### LJ

| System | Point read | Outgoing neighbors | 5-cycle | Path | Tree branch |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.176 | 0.371 | 2858.67 | 73.24 | 105.25 |
| MG-Update | 0.154 | 0.963 | 10085.16 | 16682.69 | 307.66 |
| Kùzu | 0.487 | 10.609 | 654.53 | 335.20 | 38.96 |
| Neo4j | 16.739 | 21.182 | 149.29 | 996.73 | 70.97 |
| Memgraph | 4.222 | 4.440 | 34.10 | 2226.28 | 17.66 |
| TuGraph | 5.204 | 6.679 | 47184.40 | 10978.41 | 900000.00 (limit) |

### Orkut

| System | Point read | Outgoing neighbors | 5-cycle | Path | Tree branch |
| --- | --- | --- | --- | --- | --- |
| MG-Read | 0.177 | 0.949 | n/a | 13352.55 | 309.30 |
| MG-Update | 0.195 | 3.507 | n/a | 900000.00 (limit) | 792.33 |
| Kùzu | 0.487 | 34.120 | n/a | 14994.60 | 64.55 |
| Neo4j | 16.731 | 21.706 | n/a | 6696.11 | 69.51 |
| Memgraph | 2.684 | 7.717 | n/a | 23078.82 | 23.53 |
| TuGraph | 4.933 | 8.843 | n/a | 85910.24 | 900000.00 (limit) |

## Generation ablation

The functional suite contains 200 evaluated cases.

| Group | Passed | Total |
| --- | --- | --- |
| B0 | 29 | 200 |
| B1 | 160 | 200 |
| B2 | 137 | 200 |
| B3 | 178 | 200 |
| B4 | 200 | 200 |

The B4 two-hop value marks a 180-second limit, not a completed query.

| Group | YT point read p95 | YT edge insert p95 | YT two-hop p95 |
| --- | --- | --- | --- |
| B4 | 702.5 | 1.062 | 180000 (limit) |
| B5 | 2.93 | 1.097 | 5.72 |

The generation cost values below match the manuscript table. B5 starts from the validated B4 snapshot, so its values represent the additional optimization stage.

| Group | Wall time (s) | Input tokens | Cache reads | Output tokens | Total tokens |
| --- | --- | --- | --- | --- | --- |
| B0 | 358.3 | 73,359 | 2,562,944 | 81,734 | 2,718,037 |
| B1 | 1,232.3 | 171,958 | 25,844,224 | 199,281 | 26,215,463 |
| B2 | 1,638.3 | 212,905 | 37,524,224 | 195,575 | 37,932,704 |
| B3 | 1,343.4 | 179,882 | 33,007,232 | 210,097 | 33,397,211 |
| B4 | 3,727.4 | 223,815 | 104,861,696 | 450,647 | 105,536,158 |
| B5 | 710.5 | 104,117 | 7,214,080 | 35,369 | 7,353,566 |

The repository contains the fixed framework benchmarks and the 200-case suite. The reported measurements can vary with hardware and system configuration. The host-specific multi-system timing orchestration is not included.
