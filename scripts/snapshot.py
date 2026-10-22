#!/usr/bin/env python3
"""Manage isolated lifecycle snapshots for one generated GDBMS candidate.

The Git directory lives beside generated candidates under ignored `.snapshots/`.
Every command explicitly supplies both --git-dir and --work-tree, so it never
discovers or mutates the outer research repository.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

from candidate_lock import CandidateCodeLock, CandidateLockError


ROOT = Path(__file__).resolve().parent.parent
GENERATED = ROOT / "metagdb_src" / "implementation" / "generated"
SNAPSHOTS = GENERATED / ".snapshots"


class SnapshotError(ValueError):
    pass


CHECKPOINT_RE = re.compile(r"[A-Za-z0-9][A-Za-z0-9_.-]{0,79}")


def paths(crate: str) -> tuple[Path, Path]:
    if not re.fullmatch(r"metagdb_[a-z0-9_]+", crate):
        raise SnapshotError("crate must be a lowercase Rust name beginning with metagdb_")
    candidate = (GENERATED / crate).resolve()
    generated = GENERATED.resolve()
    if candidate.parent != generated or not (candidate / "Cargo.toml").is_file():
        raise SnapshotError(f"candidate does not exist directly under generated/: {crate}")
    if (candidate / ".git").exists():
        raise SnapshotError("candidate contains its own .git; remove it before using isolated snapshots")
    git_dir = (SNAPSHOTS / f"{crate}.git").resolve()
    if git_dir.parent != SNAPSHOTS.resolve():
        raise SnapshotError("snapshot path escaped the snapshot directory")
    return candidate, git_dir


def run_git(
    candidate: Path,
    git_dir: Path,
    *args: str,
    check: bool = True,
    capture_output: bool = False,
) -> subprocess.CompletedProcess[str]:
    command = [
        "git", f"--git-dir={git_dir}", f"--work-tree={candidate}", *args,
    ]
    return subprocess.run(
        command,
        text=True,
        check=check,
        capture_output=capture_output,
    )


def require_snapshot(git_dir: Path) -> None:
    if not (git_dir / "HEAD").is_file():
        raise SnapshotError("snapshot repository is not initialized; run init first")


def checkpoint_ref(name: str) -> str:
    if not CHECKPOINT_RE.fullmatch(name):
        raise SnapshotError(
            "checkpoint name must be 1-80 ASCII letters, digits, '_', '-' or '.', "
            "and begin with a letter or digit"
        )
    ref = f"refs/heads/checkpoints/{name}"
    result = subprocess.run(
        ["git", "check-ref-format", ref],
        text=True,
        capture_output=True,
    )
    if result.returncode != 0:
        raise SnapshotError(f"invalid checkpoint name for Git: {name}")
    return ref


def current_commit(candidate: Path, git_dir: Path) -> str:
    result = run_git(candidate, git_dir, "rev-parse", "HEAD", capture_output=True)
    return result.stdout.strip()


def resolve_commit(candidate: Path, git_dir: Path, revision: str) -> str:
    if revision == "best":
        candidates = ["refs/heads/best"]
    elif revision == "initial":
        candidates = [checkpoint_ref("initialized")]
    elif re.fullmatch(r"[0-9a-fA-F]{7,40}", revision):
        candidates = [checkpoint_ref(revision), revision]
    elif CHECKPOINT_RE.fullmatch(revision):
        candidates = [checkpoint_ref(revision)]
    else:
        raise SnapshotError("revision must be best, initial, a checkpoint name, or a commit ID")
    for value in candidates:
        result = run_git(
            candidate,
            git_dir,
            "rev-parse",
            "--verify",
            f"{value}^{{commit}}",
            check=False,
            capture_output=True,
        )
        if result.returncode == 0:
            return result.stdout.strip()
    raise SnapshotError(f"snapshot revision does not exist: {revision}")


def cmd_init(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    if git_dir.exists():
        raise SnapshotError(f"snapshot repository already exists: {git_dir}")
    git_dir.parent.mkdir(parents=True, exist_ok=True)
    try:
        subprocess.run(["git", "init", "--bare", str(git_dir)], check=True)
        subprocess.run(
            ["git", f"--git-dir={git_dir}", "symbolic-ref", "HEAD", "refs/heads/main"],
            check=True,
        )
        run_git(candidate, git_dir, "config", "core.bare", "false")
        run_git(candidate, git_dir, "config", "core.worktree", str(candidate))
        run_git(candidate, git_dir, "config", "user.name", "metaGDB synthesis snapshot")
        run_git(candidate, git_dir, "config", "user.email", "snapshot@metagdb.local")
        run_git(candidate, git_dir, "config", "core.autocrlf", "false")
        run_git(candidate, git_dir, "add", "-A")
        run_git(candidate, git_dir, "commit", "-m", "[snapshot] initial candidate")
        run_git(candidate, git_dir, "branch", "-f", "checkpoints/initialized", "HEAD")
        run_git(candidate, git_dir, "branch", "-f", "best", "HEAD")
    except Exception:
        import shutil
        shutil.rmtree(git_dir, ignore_errors=True)
        raise
    print(f"Initialized isolated snapshot Git: {git_dir}")
    print("Marked initial candidate as best.")
    return 0


def cmd_status(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    run_git(candidate, git_dir, "status", "--short", "--branch")
    return 0


def cmd_checkpoint(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    label = args.label.strip()
    if not label or "\n" in label or "\r" in label:
        raise SnapshotError("checkpoint label must be one non-empty line")
    named_ref = checkpoint_ref(args.name) if args.name else None
    previous = current_commit(candidate, git_dir)
    old_named = None
    if named_ref:
        existing = run_git(
            candidate,
            git_dir,
            "rev-parse",
            "--verify",
            named_ref,
            check=False,
            capture_output=True,
        )
        if existing.returncode == 0:
            old_named = existing.stdout.strip()
    try:
        run_git(candidate, git_dir, "add", "-A")
        changed = run_git(candidate, git_dir, "diff", "--cached", "--quiet", check=False)
        if changed.returncode == 0:
            print("No candidate changes to checkpoint.")
        elif changed.returncode == 1:
            run_git(candidate, git_dir, "commit", "-m", f"[snapshot] {label}")
        else:
            raise subprocess.CalledProcessError(changed.returncode, changed.args)
        if named_ref:
            run_git(candidate, git_dir, "update-ref", named_ref, "HEAD")
    except Exception:
        # Preserve candidate files for repair, but restore this repository's
        # index and HEAD if the checkpoint operation fails partway through.
        run_git(candidate, git_dir, "reset", "--mixed", previous, check=False)
        if named_ref:
            if old_named:
                run_git(candidate, git_dir, "update-ref", named_ref, old_named, check=False)
            else:
                run_git(candidate, git_dir, "update-ref", "-d", named_ref, check=False)
        raise
    commit = current_commit(candidate, git_dir)
    print(f"Snapshot: {commit[:12]}")
    if args.name:
        print(f"Checkpoint: {args.name}")
    return 0


def cmd_mark_best(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    clean = subprocess.run(
        ["git", f"--git-dir={git_dir}", f"--work-tree={candidate}", "status", "--porcelain"],
        text=True, capture_output=True, check=True,
    )
    if clean.stdout.strip():
        raise SnapshotError("candidate has uncommitted changes; checkpoint it before mark-best")
    run_git(candidate, git_dir, "branch", "-f", "best", "HEAD")
    print("Updated best to current snapshot.")
    return 0


def restore(candidate: Path, git_dir: Path, revision: str) -> None:
    commit = resolve_commit(candidate, git_dir, revision)
    run_git(candidate, git_dir, "reset", "--hard", commit)
    run_git(candidate, git_dir, "clean", "-fdx")
    print(f"Restored candidate to {revision} ({commit[:12]}).")


def cmd_restore(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    with CandidateCodeLock(ROOT, args.crate, "snapshot restore"):
        restore(candidate, git_dir, args.revision)
    return 0


def cmd_restore_best(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    with CandidateCodeLock(ROOT, args.crate, "snapshot restore"):
        restore(candidate, git_dir, "best")
    return 0


def cmd_log(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    run_git(candidate, git_dir, "log", "--oneline", "--decorate", "--max-count", str(args.count))
    return 0


def cmd_list(args: argparse.Namespace) -> int:
    candidate, git_dir = paths(args.crate)
    require_snapshot(git_dir)
    print("Named checkpoints:", flush=True)
    run_git(
        candidate,
        git_dir,
        "for-each-ref",
        "--sort=refname",
        "--format=%(refname:short) %(objectname:short) %(subject)",
        "refs/heads/checkpoints",
        "refs/heads/best",
    )
    print("Recent snapshots:", flush=True)
    run_git(candidate, git_dir, "log", "--oneline", "--decorate", "--max-count", str(args.count))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    for name in ("init", "status", "mark-best", "restore-best"):
        item = sub.add_parser(name)
        item.add_argument("crate")
    checkpoint = sub.add_parser("checkpoint")
    checkpoint.add_argument("crate")
    checkpoint.add_argument("--label", required=True)
    checkpoint.add_argument("--name", help="stable checkpoint name, such as a completed phase")
    restore_parser = sub.add_parser("restore")
    restore_parser.add_argument("crate")
    restore_parser.add_argument(
        "revision",
        help="best, initial, a named checkpoint, or a snapshot commit ID",
    )
    log = sub.add_parser("log")
    log.add_argument("crate")
    log.add_argument("--count", type=int, default=12)
    list_parser = sub.add_parser("list")
    list_parser.add_argument("crate")
    list_parser.add_argument("--count", type=int, default=12)
    args = parser.parse_args()
    handlers = {
        "init": cmd_init,
        "status": cmd_status,
        "checkpoint": cmd_checkpoint,
        "mark-best": cmd_mark_best,
        "restore": cmd_restore,
        "restore-best": cmd_restore_best,
        "log": cmd_log,
        "list": cmd_list,
    }
    try:
        return handlers[args.command](args)
    except (SnapshotError, CandidateLockError) as error:
        print(f"Error: {error}", file=sys.stderr)
        return 2
    except subprocess.CalledProcessError as error:
        print(f"Error: snapshot Git command failed with exit code {error.returncode}", file=sys.stderr)
        return error.returncode


if __name__ == "__main__":
    raise SystemExit(main())
