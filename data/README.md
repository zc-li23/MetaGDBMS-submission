# Evaluation graphs

The three bundled files are the exact graph inputs used in the paper. They are
compressed with gzip to keep the Git download small. Restore them at the paths
expected by the benchmark manifest:

```bash
gzip -dk data/raw/uniform_standard.graph.gz data/raw/rn.graph.gz data/raw/yt.graph.gz
sha256sum data/raw/uniform_standard.graph data/raw/rn.graph data/raw/yt.graph
```

| Paper name | Restored file | Vertices in evaluated file | Directed edge records | SHA-256 of restored file |
|---|---|---:|---:|---|
| Uniform | `data/raw/uniform_standard.graph` | 10,000 | 160,000 | `2d0136c40148ba324f1a0b6971611f74d35feeca7a51d3ea94a88334d82b0e93` |
| RN | `data/raw/rn.graph` | 1,090,920 | 3,083,796 | `1e3961ca648822515741adb1ed22f1ce1b5e5709e1b2f202c471c5699094e43c` |
| YT | `data/raw/yt.graph` | 1,157,828 | 2,987,624 | `a36272a38d522fd5a1a63d7aae15dd60951b9fe42c6804b31d693d452f802bd3` |

Uniform was generated with `benchmark/performance/generate_uniform.py` using
the parameters and seed in `benchmark/performance/datasets.json`. RN derives
from SNAP's [Pennsylvania road network](https://snap.stanford.edu/data/roadNet-PA.html),
and YT derives from SNAP's [YouTube network](https://snap.stanford.edu/data/com-Youtube.html).
The local files are the definitive evaluated inputs: their vertex counts include
the ID range used in the `.graph` representation and therefore differ from the
number of distinct nodes reported by SNAP. An undirected edge may have one or
two directed records in the local graph; use the bundled files and checksums
when comparing measurements.

The format is text: `t <vertex-count> <edge-count>` is followed by
`v <id> <label> <property>` and `e <source-id> <target-id>` records.

## Large inputs

LJ and Orkut can be downloaded from SNAP's
[LiveJournal](https://snap.stanford.edu/data/com-LiveJournal.html) and
[Orkut](https://snap.stanford.edu/data/com-Orkut.html) pages. The evaluated
files are normalized `.graph` files rather than SNAP's original edge lists.
They are not distributed in this repository.

| Paper name | Evaluated file | Vertices | Directed edge records | SHA-256 of evaluated file |
|---|---|---:|---:|---|
| LJ | `data/raw/lj.graph` | 4,036,538 | 69,362,378 | `0d6b457e3bcecfd4f8c864e24e3595b05e145c724e4c03069245da64f4b451ea` |
| Orkut | `data/raw/orkut.graph` | 3,072,441 | 234,370,204 | `d94941d0416a52d4a1c18f2e3b16625a4cb596f6ae8be0da2c6e4f002ee6b566` |

The upstream downloads alone do not reproduce these exact normalized files.
The evaluated LJ vertex count includes unused IDs in its ID range. The
normalization procedure and exact Orkut edge-count difference from twice the
upstream undirected count are not yet documented, so exact large-graph reruns
require the evaluated files matching the hashes above.
