#!/usr/bin/env python3
"""Repository entry point for deterministic MetaGDB design synthesis."""

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from design_synthesis.cli import main

if __name__ == "__main__":
    raise SystemExit(main())
