#!/usr/bin/env python3
"""Generate deterministic directed regular graphs in metaGDB .graph format.

The affine vertex permutation makes IDs look shuffled while preserving an exact
in/out degree.  Generation is streaming, has O(1) auxiliary memory, and creates
no self-loops or duplicate edges for 0 < degree < vertices.
"""

from __future__ import annotations

import argparse
import math
from pathlib import Path


def coprime_multiplier(vertices: int, seed: int) -> int:
    candidate = (seed * 2 + 1) % vertices
    candidate = candidate or 1
    while math.gcd(candidate, vertices) != 1:
        candidate = (candidate + 2) % vertices or 1
    return candidate


def permute(value: int, vertices: int, multiplier: int, offset: int) -> int:
    return (multiplier * value + offset) % vertices


def generate(
    output: Path,
    vertices: int,
    degree: int,
    labels: int,
    communities: int,
    seed: int,
) -> None:
    if vertices < 2:
        raise ValueError("vertices must be at least 2")
    if not 0 < degree < vertices:
        raise ValueError("degree must be between 1 and vertices - 1")
    if labels < 1 or communities < 1:
        raise ValueError("labels and communities must be positive")

    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(output.suffix + ".tmp")
    multiplier = coprime_multiplier(vertices, seed)
    offset = seed % vertices

    with temporary.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write(f"t {vertices} {vertices * degree}\n")
        for logical in range(vertices):
            vid = permute(logical, vertices, multiplier, offset)
            label = (logical * 2654435761 + seed) % labels
            community = (logical * 11400714819323198485 + seed) % communities
            handle.write(f"v {vid} {label} {community}\n")
        for logical in range(vertices):
            src = permute(logical, vertices, multiplier, offset)
            for step in range(1, degree + 1):
                dst = permute((logical + step) % vertices, vertices, multiplier, offset)
                handle.write(f"e {src} {dst}\n")
    temporary.replace(output)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--vertices", type=int, required=True)
    parser.add_argument("--degree", type=int, required=True)
    parser.add_argument("--labels", type=int, default=4)
    parser.add_argument("--communities", type=int, default=32)
    parser.add_argument("--seed", type=int, default=20260918)
    args = parser.parse_args()
    generate(
        args.output,
        args.vertices,
        args.degree,
        args.labels,
        args.communities,
        args.seed,
    )
    print(
        f"generated {args.output}: vertices={args.vertices}, "
        f"edges={args.vertices * args.degree}, degree={args.degree}, seed={args.seed}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
