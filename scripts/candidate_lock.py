#!/usr/bin/env python3
"""Kernel-backed ownership lock for candidate code used by measure and restore."""

from __future__ import annotations

import fcntl
import os
import time
from pathlib import Path
from typing import TextIO


class CandidateLockError(RuntimeError):
    pass


def _process_start(pid: int) -> str | None:
    """Linux process start tick, recorded only for diagnostics."""
    try:
        text = Path("/proc", str(pid), "stat").read_text(encoding="utf-8")
        tail = text[text.rfind(")") + 2 :].split()
        return tail[19]
    except (OSError, ValueError, IndexError):
        return None


def _read_owner(handle: TextIO) -> dict[str, str]:
    try:
        handle.seek(0)
        return {
            key: value
            for line in handle.read().splitlines()
            if "=" in line
            for key, value in [line.split("=", 1)]
        }
    except OSError:
        return {}


class CandidateCodeLock:
    """Exclusive OS lock for operations that read or rewrite one candidate tree."""

    def __init__(self, root: Path, crate: str, operation: str):
        self.directory = root / "target" / "candidate-locks"
        self.path = self.directory / f"{crate}.lock"
        self.crate = crate
        self.operation = operation
        self.handle: TextIO | None = None

    def __enter__(self) -> "CandidateCodeLock":
        self.directory.mkdir(parents=True, exist_ok=True)
        handle = self.path.open("a+", encoding="utf-8")
        try:
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            owner = _read_owner(handle)
            handle.close()
            raise CandidateLockError(
                f"candidate {self.crate} is busy with "
                f"{owner.get('operation', 'another operation')} "
                f"(pid {owner.get('pid', '?')}); wait for it to finish"
            ) from error
        start = _process_start(os.getpid())
        if start is None:
            handle.close()
            raise CandidateLockError(
                "cannot read this process identity from /proc; refusing an unsafe lock"
            )
        handle.seek(0)
        handle.truncate()
        handle.write(
            f"pid={os.getpid()}\n"
            f"process_start={start}\n"
            f"operation={self.operation}\n"
            f"started_unix={int(time.time())}\n"
        )
        handle.flush()
        os.fsync(handle.fileno())
        self.handle = handle
        return self

    def __exit__(self, _type, _value, _traceback) -> None:
        if self.handle is None:
            return
        fcntl.flock(self.handle.fileno(), fcntl.LOCK_UN)
        self.handle.close()
        self.handle = None
