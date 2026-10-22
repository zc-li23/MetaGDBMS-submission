#!/usr/bin/env python3
"""
metaGDB build system scaffold.

Responsible for:
  - list    : List all candidate implementation crates under implementation/.
  - current : Display the currently active implementation.
  - use     : Switch the active implementation and regenerate Cargo.toml /
              metagdb-Cargo.toml / metagdb/src/assembly/active.rs.
  - clean   : Remove all script-generated activation artifacts.
  - create  : Create an incomplete, isolated implementation shell for an agent.
  - verify  : Activate an implementation, test it, and run its demo.
  - benchmark: Activate an implementation and run the fixed language benchmark.
  - system-benchmark: Run fixed ACID/storage/recovery and algorithm-registration checks.
  - algorithm-check: Run agent-authored core and language tests for every requested algorithm.
  - micro-benchmark: Run fixed single-thread workloads on selected graph scenarios.
  - compare-performance: Apply design-vector weights and regression guardrails.

Active implementation source (by priority):
  1. CLI explicit argument: ./scripts/build.py use <crate>
  2. implementation/manifest.local.toml (gitignored, personal override)
  3. implementation/manifest.toml         (team default)

Note: metagdb/src/main.rs is the stable entry point of the framework itself,
      independent of any implementation, and is NOT generated or managed
      by this script.
"""

from __future__ import annotations

import argparse
import csv
import datetime
import hashlib
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

from candidate_lock import CandidateCodeLock, CandidateLockError


# ---------------------------------------------------------------------------
# Minimal TOML reader
# ---------------------------------------------------------------------------
#
# Our manifest only contains simple key-value pairs of the form `active = "name"`,
# so we avoid introducing any third-party dependencies and also avoid depending
# on Python 3.11+'s tomllib.

def _parse_simple_toml(text: str) -> dict[str, str]:
    """Parse simple TOML text in `key = "value"` form.

    Ignores comments (starting with #) and blank lines; silently skips
    other forms.
    """
    out: dict[str, str] = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            continue
        key, _, value = line.partition("=")
        key = key.strip()
        value = value.strip()
        # Remove trailing comment
        if "#" in value:
            value = value.split("#", 1)[0].rstrip()
        if (value.startswith('"') and value.endswith('"')) or (
            value.startswith("'") and value.endswith("'")
        ):
            value = value[1:-1]
        out[key] = value
    return out



# ---------------------------------------------------------------------------
# Path constants
# ---------------------------------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
IMPL_DIR = ROOT / "metagdb_src" / "implementation"
GEN_DIR = IMPL_DIR / "generated"
MANIFEST = IMPL_DIR / "manifest.toml"
MANIFEST_LOCAL = IMPL_DIR / "manifest.local.toml"
CONFIG_DIR = Path(__file__).resolve().parent / "config"
PERFORMANCE_DIR = ROOT / "benchmark" / "performance"
PERFORMANCE_MANIFEST = PERFORMANCE_DIR / "datasets.json"
PERFORMANCE_QUERY_TEMPLATES = PERFORMANCE_DIR / "query_templates.json"
SYNTAX_CHECK_MANIFEST = ROOT / "benchmark" / "syntax_check" / "Cargo.toml"

TEMPLATES = Path(__file__).resolve().parent / "templates"
TPL_WORKSPACE = TEMPLATES / "Cargo.toml.tmpl"
TPL_METAGDB_CARGO = TEMPLATES / "metagdb.Cargo.toml.tmpl"
TPL_ASSEMBLY = TEMPLATES / "assembly.rs.tmpl"
TPL_IMPL_CARGO = TEMPLATES / "implementation.Cargo.toml.tmpl"
TPL_IMPL_LIB = TEMPLATES / "implementation.lib.rs.tmpl"
TPL_IMPL_README = TEMPLATES / "implementation.README.md.tmpl"

OUT_WORKSPACE_CARGO = ROOT / "Cargo.toml"
OUT_METAGDB_CARGO = ROOT / "metagdb_src" / "metagdb" / "Cargo.toml"
OUT_ASSEMBLY = ROOT / "metagdb_src" / "metagdb" / "src" / "assembly" / "active.rs"

GENERATED_FILES = [OUT_WORKSPACE_CARGO, OUT_METAGDB_CARGO, OUT_ASSEMBLY]


# ---------------------------------------------------------------------------
# Implementation discovery
# ---------------------------------------------------------------------------

def discover_implementations() -> list[tuple[str, Path]]:
    """Scan implementation/ for candidate crates.

    Returns [(crate_name, crate_path_relative_to_root), ...], sorted by
    crate_name. Directories under the generated/ subdirectory are also
    considered candidates.
    """
    candidates: list[tuple[str, Path]] = []

    # Top-level (hand-written implementations, e.g. metagdb_default)
    for entry in IMPL_DIR.iterdir():
        if not entry.is_dir() or entry.name == "generated":
            continue
        if (entry / "Cargo.toml").is_file():
            candidates.append((entry.name, entry.relative_to(ROOT)))

    # LLM-generated implementations
    gen_dir = IMPL_DIR / "generated"
    if gen_dir.is_dir():
        for entry in gen_dir.iterdir():
            if entry.is_dir() and (entry / "Cargo.toml").is_file():
                candidates.append((entry.name, entry.relative_to(ROOT)))

    candidates.sort(key=lambda x: x[0])
    return candidates


def resolve_active(cli_choice: str | None) -> str:
    """Resolve the currently active implementation crate name by priority."""
    if cli_choice:
        return cli_choice
    if MANIFEST_LOCAL.is_file():
        data = _parse_simple_toml(MANIFEST_LOCAL.read_text(encoding="utf-8"))
        if "active" in data:
            return data["active"]
    if MANIFEST.is_file():
        data = _parse_simple_toml(MANIFEST.read_text(encoding="utf-8"))
        if "active" in data:
            return data["active"]
    raise SystemExit("Error: unable to resolve active implementation; no argument provided and manifest files are missing the 'active' field")


# ---------------------------------------------------------------------------
# Language config reading
# ---------------------------------------------------------------------------

def read_language_config(crate_name: str) -> dict[str, str]:
    """Read the language configuration TOML for a given implementation crate.

    Config file path: scripts/config/<crate_name>.toml.
    If the file does not exist or is malformed, prints a clear error message
    and exits.

    @param crate_name: implementation crate name (e.g. "metagdb_default")
    @return: dictionary containing language / query_language_variant /
             antlr_crate_name / antlr_crate_dir
    @raise SystemExit: exits when the config file is missing, malformed,
                       or lacks required fields
    """
    config_path = CONFIG_DIR / f"{crate_name}.toml"
    if not config_path.is_file():
        raise SystemExit(
            f"Error: language config file not found for implementation crate '{crate_name}'.\n"
            f"  Expected location: {config_path.relative_to(ROOT)}\n"
            f"  Please run the language configuration command first:\n"
            f"    ./scripts/language.py configure {crate_name} <language>\n"
            f"  Available languages:\n"
            f"    ./scripts/language.py list"
        )
    try:
        config = _parse_simple_toml(config_path.read_text(encoding="utf-8"))
    except Exception as e:
        raise SystemExit(
            f"Error: malformed language config file '{config_path.relative_to(ROOT)}': {e}\n"
            f"  Please re-run: ./scripts/language.py configure {crate_name} <language>"
        )

    # Validate required fields (defensive programming)
    required = [
        "language",
        "query_language_variant",
        "antlr_crate_name",
        "antlr_crate_dir",
    ]
    missing = [k for k in required if k not in config]
    if missing:
        raise SystemExit(
            f"Error: language config file '{config_path.relative_to(ROOT)}' "
            f"is missing required fields: {missing}\n"
            f"  Please re-run: ./scripts/language.py configure {crate_name} <language>"
        )
    return config


# ---------------------------------------------------------------------------
# Template rendering
# ---------------------------------------------------------------------------

def render(tpl_path: Path, vars_: dict[str, str]) -> str:
    """Minimal mustache: replace {{KEY}} with vars_[KEY].

    Substitutions are performed in descending order of variable name length
    to prevent shorter names from breaking longer ones
    (e.g. substituting {{LANG}} first would corrupt subsequent {{LANG_PATH}}).
    """
    text = tpl_path.read_text(encoding="utf-8")
    for k, v in sorted(vars_.items(), key=lambda x: -len(x[0])):
        text = text.replace("{{" + k + "}}", v)
    return text


def write_file(path: Path, content: str) -> None:
    """Write file, creating parent directories if necessary."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


# ---------------------------------------------------------------------------
# Subcommands
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# Agent candidate creation and verification
# ---------------------------------------------------------------------------

SUPPORTED_LANGUAGES = {
    "Cypher": "Cypher",
    "GQL": "Gql",
    "Gremlin": "Gremlin",
}


def cmd_create(args: argparse.Namespace) -> int:
    """Create a deliberately incomplete implementation crate for a coding agent."""
    crate = args.crate
    if not re.fullmatch(r"[a-z][a-z0-9_]*", crate) or not crate.startswith("metagdb_"):
        print(
            "Error: crate name must be lowercase Rust identifier beginning with 'metagdb_'.",
            file=sys.stderr,
        )
        return 2

    target = GEN_DIR / crate
    stable_target = IMPL_DIR / crate
    config_path = CONFIG_DIR / f"{crate}.toml"
    if target.exists() or stable_target.exists() or config_path.exists():
        print(f"Error: implementation '{crate}' already exists.", file=sys.stderr)
        return 2

    language = args.language
    variant = SUPPORTED_LANGUAGES[language]
    lang_dir = IMPL_DIR / "language" / language
    lang_cargo = lang_dir / "Cargo.toml"
    if not lang_cargo.is_file():
        print(
            f"Error: generated parser crate is missing for language '{language}': "
            f"{lang_cargo.relative_to(ROOT)}",
            file=sys.stderr,
        )
        return 2

    lang_package = _parse_simple_toml(lang_cargo.read_text(encoding="utf-8"))
    lang_crate = lang_package.get("name", f"metagdb_{language.lower()}_antlr")
    vars_ = {
        "IMPL_CRATE": crate,
        "LANGUAGE": language,
        "LANG_CRATE": lang_crate,
    }

    try:
        (target / "src").mkdir(parents=True)
        write_file(target / "Cargo.toml", render(TPL_IMPL_CARGO, vars_))
        write_file(target / "src" / "lib.rs", render(TPL_IMPL_LIB, vars_))
        write_file(target / "README.md", render(TPL_IMPL_README, vars_))
        config_text = (
            "# Generated by scripts/build.py create; keep with the local candidate.\n"
            f'language = "{language}"\n'
            f'query_language_variant = "{variant}"\n'
            f'antlr_crate_name = "{lang_crate}"\n'
            f'antlr_crate_dir = "{lang_dir.relative_to(ROOT).as_posix()}"\n'
        )
        write_file(config_path, config_text)
    except Exception:
        shutil.rmtree(target, ignore_errors=True)
        config_path.unlink(missing_ok=True)
        raise

    print(f"Created incomplete implementation shell: {target.relative_to(ROOT)}")
    print(f"Language configuration: {config_path.relative_to(ROOT)}")
    print("Next:")
    print(f"  1. Implement the contract in docs/implementation_contract.md")
    print(f"  2. Run: python3 scripts/build.py verify {crate}")
    return 0


def cmd_verify(args: argparse.Namespace) -> int:
    """Activate and verify a complete implementation with real build feedback."""
    selected = argparse.Namespace(crate=args.crate)
    status = cmd_use(selected)
    if status != 0:
        return status

    steps = [
        ("workspace check", ["cargo", "check", "--workspace"]),
        ("workspace tests", ["cargo", "test", "--workspace"]),
        ("end-to-end demo", ["cargo", "run", "--", "--demo"]),
    ]
    for label, command in steps:
        print(f"\n==> {label}: {' '.join(command)}")
        try:
            result = subprocess.run(command, cwd=ROOT, timeout=args.timeout)
        except subprocess.TimeoutExpired:
            print(
                f"Error: {label} exceeded {args.timeout} seconds.",
                file=sys.stderr,
            )
            return 124
        if result.returncode != 0:
            print(f"Error: {label} failed with exit code {result.returncode}.", file=sys.stderr)
            return result.returncode

    print(f"\nImplementation '{args.crate}' passed all verification steps.")
    return 0


def _grammar_conditional_report(language: str, dataset_dir: Path, output_dir: Path) -> None:
    """Report candidate coverage over both the full profile and its grammar-valid subset."""
    grammar_report = output_dir / "grammar.tsv"
    command = [
        "cargo", "run", "--quiet", "--manifest-path", str(SYNTAX_CHECK_MANIFEST), "--",
        "--language", language.lower(), str(dataset_dir / "core.jsonl"),
        "--output", str(grammar_report),
    ]
    print(f"\n==> grammar applicability: {' '.join(command)}")
    syntax = subprocess.run(command, cwd=ROOT)
    if syntax.returncode not in (0, 1):
        print("Warning: grammar applicability report could not be generated.", file=sys.stderr)
        return
    execution_report = output_dir / "report.tsv"
    if not grammar_report.is_file() or not execution_report.is_file():
        print("Warning: grammar-conditional score lacks an input report.", file=sys.stderr)
        return
    with grammar_report.open(encoding="utf-8", newline="") as handle:
        grammar_rows = {row["id"]: row for row in csv.DictReader(handle, delimiter="	")}
    with execution_report.open(encoding="utf-8", newline="") as handle:
        execution_rows = {row["id"]: row for row in csv.DictReader(handle, delimiter="	")}
    eligible = {
        case_id for case_id, row in grammar_rows.items()
        if row.get("scored") == "true" and row.get("grammar_applicable") == "true"
    }
    missing = sorted(eligible - execution_rows.keys())
    passed = sum(execution_rows[case_id].get("status") == "PASS" for case_id in eligible & execution_rows.keys())
    total = len(eligible)
    rate = passed / total if total else 0.0
    rejected_positive = sorted(
        case_id for case_id, row in grammar_rows.items()
        if row.get("scored") == "true"
        and row.get("negative") == "false"
        and row.get("accepted") == "false"
    )
    summary = {
        "schema_version": 1,
        "language": language.lower(),
        "full_profile_report": "summary.json",
        "grammar_report": grammar_report.name,
        "grammar_conditional_passed": passed,
        "grammar_conditional_scored": total,
        "grammar_conditional_pass_rate": rate,
        "grammar_rejected_positive_cases": rejected_positive,
        "missing_execution_cases": missing,
    }
    (output_dir / "grammar_summary.json").write_text(
        json.dumps(summary, indent=2) + "\n", encoding="utf-8"
    )
    print(
        f"Grammar-conditional score: {passed}/{total} = {rate:.1%}; "
        f"grammar-rejected positive profile cases: {len(rejected_positive)}"
    )


def cmd_benchmark(args: argparse.Namespace) -> int:
    """Activate an implementation and evaluate it against the fixed benchmark."""
    selected = argparse.Namespace(crate=args.crate)
    status = cmd_use(selected)
    if status != 0:
        return status

    language = read_language_config(args.crate)["language"]
    dataset_dir = ROOT / "benchmark" / language.lower()
    if not dataset_dir.is_dir():
        print(
            f"Error: benchmark dataset not found: {dataset_dir.relative_to(ROOT)}",
            file=sys.stderr,
        )
        return 2

    output_dir = Path(args.output_dir).resolve() if args.output_dir else (
        ROOT / "target" / "benchmark-run"
    )
    command = [
        "cargo",
        "run",
        "--",
        "--benchmark",
        str(dataset_dir.relative_to(ROOT)),
        "--min-pass-rate",
        str(args.min_pass_rate),
        "--output-dir",
        str(output_dir),
    ]
    print(f"\n==> external benchmark: {' '.join(command)}")
    try:
        result = subprocess.run(command, cwd=ROOT, timeout=args.timeout)
    except subprocess.TimeoutExpired:
        print(
            f"Error: benchmark exceeded {args.timeout} seconds.",
            file=sys.stderr,
        )
        return 124
    _grammar_conditional_report(language, dataset_dir, output_dir)
    return result.returncode


def cmd_system_benchmark(args: argparse.Namespace) -> int:
    """Run fixed ACID, storage, recovery, and algorithm-registration checks."""
    design_path = Path(args.design).resolve()
    from synthesis import DesignError, algorithm_specs, validate_design

    try:
        design = validate_design(design_path)
    except DesignError as error:
        print(f"Error: invalid design vector: {error}", file=sys.stderr)
        return 2

    crate = design.get("candidate", {}).get("crate_name")
    if crate != args.crate:
        print(
            f"Error: design vector candidate is {crate!r}, expected {args.crate!r}.",
            file=sys.stderr,
        )
        return 2
    try:
        transactions = design["transactions"]
        schema_mode = design["schema"]["mode"]
        vertex_labels = design["schema"]["vertex_labels"]
        edge_types = design["schema"]["edge_types"]
        property_keys = design["schema"]["property_keys"]
        algorithms = [spec["name"] for spec in algorithm_specs(design)]
        isolation = transactions["isolation_level"]
        concurrency_control = transactions["concurrency_control"]
        durability = transactions["durability"]
    except (KeyError, TypeError) as error:
        print(f"Error: design vector lacks system benchmark field: {error}", file=sys.stderr)
        return 2

    selected = argparse.Namespace(crate=args.crate)
    status = cmd_use(selected)
    if status != 0:
        return status

    command = [
        "cargo", "run", "--", "--system-benchmark",
        "--isolation", str(isolation),
        "--concurrency-control", str(concurrency_control),
        "--durability", str(durability),
        "--schema-mode", str(schema_mode),
        "--vertex-labels", ",".join(vertex_labels),
        "--edge-types", ",".join(edge_types),
        "--property-keys", ",".join(property_keys),
        "--required-algorithms", ",".join(algorithms),
    ]
    if args.diagnostic:
        command.append("--diagnostic")
    print(f"\n==> fixed system benchmark: {' '.join(command)}")
    try:
        result = subprocess.run(command, cwd=ROOT, timeout=args.timeout)
    except subprocess.TimeoutExpired:
        print(f"Error: system benchmark exceeded {args.timeout} seconds.", file=sys.stderr)
        return 124
    return result.returncode


def _algorithm_plan_path(crate: str) -> Path:
    return GEN_DIR / crate / ".synthesis" / "algorithm_plan.json"


def _load_complete_algorithm_plan(design: dict, crate: str) -> dict:
    from synthesis import DesignError, validate_algorithm_plan

    path = _algorithm_plan_path(crate)
    try:
        return validate_algorithm_plan(design, path, require_complete=True)
    except DesignError as error:
        raise SystemExit(f"Error: incomplete algorithm validation plan: {error}") from error


def cmd_algorithm_check(args: argparse.Namespace) -> int:
    """Run the independent tests named by the agent-authored algorithm plan."""
    design_path = Path(args.design).resolve()
    from synthesis import DesignError, algorithm_specs, validate_design

    try:
        design = validate_design(design_path)
    except DesignError as error:
        print(f"Error: invalid design vector: {error}", file=sys.stderr)
        return 2
    if design["candidate"]["crate_name"] != args.crate:
        print("Error: design vector candidate does not match the requested crate.", file=sys.stderr)
        return 2
    try:
        plan = _load_complete_algorithm_plan(design, args.crate)
    except SystemExit as error:
        print(error, file=sys.stderr)
        return 2
    if cmd_use(argparse.Namespace(crate=args.crate)) != 0:
        return 1

    try:
        listed = subprocess.run(
            ["cargo", "test", "-p", args.crate, "--", "--list"],
            cwd=ROOT, timeout=args.timeout, capture_output=True, text=True,
        )
    except subprocess.TimeoutExpired:
        print("Error: listing candidate tests timed out.", file=sys.stderr)
        return 124
    if listed.returncode != 0:
        sys.stdout.write(listed.stdout)
        sys.stderr.write(listed.stderr)
        return listed.returncode
    available = {
        line.rsplit(": test", 1)[0]
        for line in listed.stdout.splitlines()
        if line.endswith(": test")
    }
    output = ROOT / "target" / "algorithm-check" / "report.tsv"
    output.parent.mkdir(parents=True, exist_ok=True)
    rows: list[dict[str, str]] = []
    failed = False
    weights = {spec["name"].casefold(): spec["performance_weight"] for spec in algorithm_specs(design)}
    for row in plan["algorithms"]:
        for kind, field in (("core", "core_test"), ("language_e2e", "language_test")):
            test_name = row[field]
            status = "PASS"
            detail = row["correctness_summary"]
            if test_name not in available:
                status = "FAIL"
                detail = f"declared test does not exist: {test_name}"
                failed = True
            else:
                try:
                    result = subprocess.run(
                        ["cargo", "test", "-p", args.crate, test_name, "--", "--exact", "--nocapture"],
                        cwd=ROOT, timeout=args.timeout,
                    )
                    if result.returncode != 0:
                        status = "FAIL"
                        detail = f"test exited with status {result.returncode}"
                        failed = True
                except subprocess.TimeoutExpired:
                    status = "FAIL"
                    detail = f"test exceeded {args.timeout} seconds"
                    failed = True
            rows.append({
                "algorithm": row["name"],
                "performance_weight": str(weights[row["name"].casefold()]),
                "evidence": kind,
                "test": test_name,
                "status": status,
                "detail": detail.replace("\t", " ").replace("\n", " "),
            })
    with output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=["algorithm", "performance_weight", "evidence", "test", "status", "detail"],
            delimiter="\t", lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)
    print(f"\nAlgorithm evidence report: {output}")
    return 1 if failed else 0


def _write_runtime_algorithm_plan(plan: dict, output: Path) -> None:
    """Write a dependency-free wire format consumed by the Rust micro runner."""
    def encoded(value: str) -> str:
        return value.encode("utf-8").hex()

    lines = ["metagdb-algorithm-plan-v1"]
    for row in plan["algorithms"]:
        name = row["name"]
        lines.append(
            "A\t{}\t{}\t{}".format(
                encoded(name),
                "1" if row["allow_empty_result"] else "0",
                encoded(row["language_query_template"]),
            )
        )
        for key, value in row["arguments"].items():
            if value is None:
                kind, rendered = "null", ""
            elif isinstance(value, bool):
                kind, rendered = "bool", str(value).lower()
            elif isinstance(value, int):
                kind, rendered = "int", str(value)
            elif isinstance(value, float):
                kind, rendered = "float", repr(value)
            else:
                kind, rendered = "string", value
            lines.append(
                "P\t{}\t{}\t{}\t{}".format(
                    encoded(name), encoded(key), kind, encoded(rendered)
                )
            )
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _load_query_templates(language: str) -> tuple[str, str]:
    """Load and grammar-check the fixed public-query performance workload."""
    try:
        document = json.loads(PERFORMANCE_QUERY_TEMPLATES.read_text(encoding="utf-8"))
        if document.get("schema_version") != 1:
            raise ValueError("schema_version must be 1")
        row = document["languages"][language]
        point_read = row["point_read"]
        one_hop = row["one_hop"]
    except (OSError, json.JSONDecodeError, KeyError, TypeError, ValueError) as error:
        raise SystemExit(f"Error: invalid performance query templates: {error}")
    for name, template in (("point_read", point_read), ("one_hop", one_hop)):
        if not isinstance(template, str) or template.count("{source_id}") != 1:
            raise SystemExit(
                f"Error: performance template {language}.{name} must contain "
                "exactly one {source_id} placeholder"
            )
        query = template.replace("{source_id}", "1")
        command = [
            "cargo", "run", "--quiet", "--manifest-path", str(SYNTAX_CHECK_MANIFEST), "--",
            "--language", language.lower(), "--query", query,
        ]
        result = subprocess.run(command, cwd=ROOT)
        if result.returncode != 0:
            raise SystemExit(
                f"Error: performance template {language}.{name} is rejected by the "
                "configured grammar: {query}"
            )
    return point_read, one_hop


def _load_performance_inputs(design_path: Path, crate: str) -> tuple[dict, dict]:
    from synthesis import DesignError, validate_design

    try:
        design = validate_design(design_path)
    except DesignError as error:
        raise SystemExit(f"Error: invalid design vector: {error}")
    configured = design.get("candidate", {}).get("crate_name")
    if configured != crate:
        raise SystemExit(
            f"Error: design vector candidate is {configured!r}, expected {crate!r}."
        )
    try:
        manifest = json.loads(PERFORMANCE_MANIFEST.read_text(encoding="utf-8"))
        datasets = manifest["datasets"]
    except (OSError, json.JSONDecodeError, KeyError, TypeError) as error:
        raise SystemExit(f"Error: invalid performance dataset manifest: {error}")
    return design, datasets


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _fingerprint_files(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    if not path.is_dir():
        return []
    excluded = {"target", ".git", ".synthesis", "__pycache__"}
    return [
        item
        for item in sorted(path.rglob("*"))
        if item.is_file() and not any(part in excluded for part in item.parts)
    ]


def source_revision(crate: str) -> str:
    """Fingerprint every source input that can change the measured binary.

    Candidate code is only one input. The stable runner, metagdb_core, the
    selected generated ANTLR crate, generated Cargo manifests and the resolved
    dependency lock all affect the executable and therefore belong to its
    measurement identity.
    """
    crate_dir = ROOT / "metagdb_src" / "implementation" / "generated" / crate
    if not crate_dir.is_dir():
        raise SystemExit(f"Error: candidate crate not found: {crate_dir}")
    language = read_language_config(crate)
    roots = [
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "metagdb_src" / "metagdb",
        ROOT / "metagdb_src" / "metagdb_core",
        crate_dir,
        crate_dir / ".synthesis" / "algorithm_plan.json",
        ROOT / language["antlr_crate_dir"],
        PERFORMANCE_QUERY_TEMPLATES,
    ]
    files: list[Path] = []
    for root in roots:
        files.extend(_fingerprint_files(root))
    unique = sorted(set(files))
    if not unique:
        raise SystemExit("Error: no source files found for the performance revision")
    digest = hashlib.sha256()
    for item in unique:
        try:
            relative = item.relative_to(ROOT)
        except ValueError:
            relative = item
        digest.update(str(relative).encode("utf-8"))
        digest.update(b"\0")
        digest.update(item.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def _prepare_dataset(name: str, spec: dict) -> tuple[Path, str]:
    try:
        path = (ROOT / spec["path"]).resolve()
        path.relative_to(ROOT.resolve())
    except (KeyError, TypeError, ValueError) as error:
        raise SystemExit(f"Error: dataset {name!r} has an unsafe or invalid path: {error}")
    kind = spec.get("kind")
    if kind == "generated_regular":
        if not path.is_file():
            command = [
                sys.executable,
                str(PERFORMANCE_DIR / "generate_uniform.py"),
                "--output", str(path),
                "--vertices", str(spec["vertices"]),
                "--degree", str(spec["degree"]),
                "--labels", str(spec["labels"]),
                "--communities", str(spec["communities"]),
                "--seed", str(spec["seed"]),
            ]
            print(f"\n==> generate dataset {name}: {' '.join(command)}")
            result = subprocess.run(command, cwd=ROOT)
            if result.returncode != 0:
                raise SystemExit(result.returncode)
    elif kind == "local_real":
        if not path.is_file():
            raise SystemExit(
                f"Error: local dataset {name!r} is missing at {path}. "
                "Real datasets are intentionally not downloaded or committed; place the verified file there."
            )
        expected = spec.get("sha256")
        actual = _sha256(path)
        if not expected or actual != expected:
            raise SystemExit(
                f"Error: checksum mismatch for {name!r}: expected {expected}, got {actual}"
            )
    else:
        raise SystemExit(f"Error: dataset {name!r} has unsupported kind {kind!r}")
    return path, actual if kind == "local_real" else _sha256(path)


def cmd_micro_benchmark(args: argparse.Namespace) -> int:
    """Run a measurement while owning the candidate from activation through reports."""
    try:
        with CandidateCodeLock(ROOT, args.crate, "performance measurement"):
            return _cmd_micro_benchmark_locked(args)
    except CandidateLockError as error:
        print(f"Error: {error}", file=sys.stderr)
        return 2


def _cmd_micro_benchmark_locked(args: argparse.Namespace) -> int:
    """Run fixed performance workloads without placing datasets in repository history."""
    design_path = Path(args.design).resolve()
    design, manifest = _load_performance_inputs(design_path, args.crate)
    selected = list(design["performance"]["datasets"])
    if args.profile:
        selected = [name for name in selected if args.profile in manifest[name].get("profiles", [])]
    if args.datasets:
        requested = [name.strip() for name in args.datasets.split(",") if name.strip()]
        unknown = set(requested) - set(design["performance"]["datasets"])
        if unknown:
            print(
                "Error: --datasets must be a subset of performance.datasets: "
                + ", ".join(sorted(unknown)),
                file=sys.stderr,
            )
            return 2
        selected = requested
    if not selected:
        print("Error: dataset selection is empty.", file=sys.stderr)
        return 2

    status = cmd_use(argparse.Namespace(crate=args.crate))
    if status != 0:
        return status
    lockfile = subprocess.run(["cargo", "generate-lockfile"], cwd=ROOT)
    if lockfile.returncode != 0:
        return lockfile.returncode
    language = read_language_config(args.crate)["language"]
    point_query_template, one_hop_query_template = _load_query_templates(language)
    output_dir = Path(args.output_dir).resolve() if args.output_dir else (
        ROOT / "target" / "performance-run" / args.crate
    )
    output_dir.mkdir(parents=True, exist_ok=True)
    repeat = int(design["performance"]["repeat"])
    from synthesis import algorithm_specs
    runtime_plan: Path | None = None
    candidate_plan = _algorithm_plan_path(args.crate)
    object_algorithms = any(not spec["legacy_shorthand"] for spec in algorithm_specs(design))
    if candidate_plan.is_file():
        try:
            plan = _load_complete_algorithm_plan(design, args.crate)
        except SystemExit as error:
            print(error, file=sys.stderr)
            return 2
        runtime_plan = output_dir / "algorithm_plan.runtime"
        _write_runtime_algorithm_plan(plan, runtime_plan)
    elif object_algorithms:
        print(
            f"Error: required algorithm plan is missing: {candidate_plan}",
            file=sys.stderr,
        )
        return 2
    revision = source_revision(args.crate)
    design_sha = _sha256(design_path)
    error_metrics: list[str] = []
    dataset_hashes: dict[str, str] = {}
    for name in selected:
        graph, graph_sha = _prepare_dataset(name, manifest[name])
        dataset_hashes[name] = graph_sha
        output = output_dir / f"{name}.tsv"
        command = [
            "cargo", "run", "--release", "--", "--micro-benchmark",
            "--dataset", name,
            "--graph", str(graph),
            "--repeat", str(repeat),
            "--output", str(output),
            "--revision", revision,
            "--design-sha", design_sha,
            "--graph-sha", graph_sha,
            "--point-query-template", point_query_template,
            "--one-hop-query-template", one_hop_query_template,
        ]
        if args.reuse_data:
            command.append("--reuse-data")
        if "transaction_edge_insert_window" in design["performance"]["workload"]:
            command.append("--edge-insert-window")
        if runtime_plan is not None:
            command.extend(["--algorithm-plan", str(runtime_plan)])
        print(f"\n==> micro-benchmark {name}: {' '.join(command)}")
        try:
            result = subprocess.run(command, cwd=ROOT, timeout=args.timeout)
        except subprocess.TimeoutExpired:
            print(f"Error: micro-benchmark {name} exceeded {args.timeout} seconds.", file=sys.stderr)
            return 124
        if result.returncode != 0:
            return result.returncode
        try:
            with output.open(encoding="utf-8", newline="") as handle:
                for row in csv.DictReader(handle, delimiter="\t"):
                    if row.get("status") == "ERROR":
                        error_metrics.append(f"{name}/{row.get('metric')}: {row.get('detail')}")
        except OSError as error:
            print(f"Error: cannot read micro-benchmark report {output}: {error}", file=sys.stderr)
            return 2
    # The report records which source and design produced it, so a comparison can
    # tell replicate runs of one binary from runs of two different ones.
    (output_dir / "run.json").write_text(
        json.dumps(
            {
                "schema_version": 1,
                "crate": args.crate,
                "revision": revision,
                "design_sha256": design_sha,
                "datasets": selected,
                "dataset_sha256": dataset_hashes,
                "query_templates_sha256": _sha256(PERFORMANCE_QUERY_TEMPLATES),
                "repeat": repeat,
                "reuse_data": bool(args.reuse_data),
                "metrics_error": bool(error_metrics),
                "finished_at": datetime.datetime.now(datetime.timezone.utc)
                .replace(microsecond=0)
                .isoformat(),
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"\nPerformance reports: {output_dir} (revision {revision[:12]})")
    if error_metrics:
        print("Error: micro-benchmark contains failed metrics:", file=sys.stderr)
        for item in error_metrics:
            print(f"  - {item}", file=sys.stderr)
        return 1
    return 0


def cmd_compare_performance(args: argparse.Namespace) -> int:
    """Compare baseline and candidate reports using the canonical scorer."""
    design_path = Path(args.design).resolve()
    design, _ = _load_performance_inputs(design_path, args.crate)
    del design
    output = Path(args.output).resolve() if args.output else (
        ROOT / "target" / "performance-comparison.json"
    )
    command = [
        sys.executable,
        str(PERFORMANCE_DIR / "compare.py"),
        "--design", str(design_path),
        "--output", str(output),
    ]
    for directory in args.baseline:
        command += ["--baseline", str(Path(directory).resolve())]
    for directory in args.candidate:
        command += ["--candidate", str(Path(directory).resolve())]
    if args.confirm_with:
        command += ["--confirm-with", str(Path(args.confirm_with).resolve())]
    if args.allow_reused:
        command.append("--allow-reused")
    if args.allow_unstamped:
        command.append("--allow-unstamped")
    print(f"\n==> compare performance: {' '.join(command)}")
    evidence_path = GEN_DIR / args.crate / ".synthesis" / "performance_evidence.json"
    result = subprocess.run(command, cwd=ROOT)
    if result.returncode not in (0, 4):
        evidence_path.unlink(missing_ok=True)
        return result.returncode
    try:
        comparison = json.loads(output.read_text(encoding="utf-8"))
        revision = source_revision(args.crate)
        candidate_revisions = comparison.get("candidate_revisions", [])
        status = comparison.get("status")
        accepted = status == "accepted" and comparison.get("accepted") is True
        inconclusive = (
            status == "inconclusive"
            and comparison.get("accepted") is False
            and bool(comparison.get("noise_limited_details"))
            and not comparison.get("failure_details")
            and not comparison.get("tentative_failure_details")
        )
        if not (accepted or inconclusive):
            raise ValueError("comparison is neither accepted nor solely noise-limited")
        if accepted and comparison.get("noise_limited_details"):
            raise ValueError("accepted comparison contains noise-limited required metrics")
        if candidate_revisions != [revision]:
            raise ValueError(
                "performance comparison does not measure the candidate's current source revision"
            )
        evidence = {
            "schema_version": 1,
            "recorded_at": datetime.datetime.now(datetime.timezone.utc)
                .replace(microsecond=0).isoformat(),
            "conclusion_scope": (
                "performance_accepted" if accepted
                else "correctness_complete_performance_inconclusive"
            ),
            "comparison_path": str(output),
            "comparison_sha256": _sha256(output),
            "current_candidate_revision": revision,
            "comparison": comparison,
        }
        evidence_path.write_text(
            json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
        )
    except (OSError, json.JSONDecodeError, TypeError, ValueError) as error:
        evidence_path.unlink(missing_ok=True)
        print(f"Error: cannot record performance evidence: {error}", file=sys.stderr)
        return 2
    print(f"Structured performance evidence: {evidence_path}")
    return result.returncode


def cmd_list(_args: argparse.Namespace) -> int:
    impls = discover_implementations()
    if not impls:
        print("(No implementation crates found)")
        return 0
    active = None
    try:
        active = resolve_active(None)
    except SystemExit:
        pass
    print(f"Available implementations ({len(impls)} total):")
    for name, path in impls:
        marker = "*" if name == active else " "
        print(f"  {marker} {name:<24} {path}")
    if active:
        print(f"\nCurrently active: {active}")
    return 0


def cmd_current(_args: argparse.Namespace) -> int:
    active = resolve_active(None)
    print(active)
    return 0


def cmd_use(args: argparse.Namespace) -> int:
    impls = dict(discover_implementations())
    target = args.crate or resolve_active(None)
    if target not in impls:
        print(f"Error: implementation crate `{target}` does not exist in implementation/", file=sys.stderr)
        print("Available: ", ", ".join(impls.keys()) or "(none)", file=sys.stderr)
        return 1

    impl_path = impls[target].as_posix()

    # Read the language config bound to this implementation crate
    lang_config = read_language_config(target)

    vars_ = {
        "IMPL_CRATE": target,
        "IMPL_PATH": impl_path,
        "LANG_CRATE": lang_config["antlr_crate_name"],
        "LANG_PATH": lang_config["antlr_crate_dir"],
        "QUERY_LANGUAGE_VARIANT": lang_config["query_language_variant"],
    }

    write_file(OUT_WORKSPACE_CARGO, render(TPL_WORKSPACE, vars_))
    write_file(OUT_METAGDB_CARGO, render(TPL_METAGDB_CARGO, vars_))
    write_file(OUT_ASSEMBLY, render(TPL_ASSEMBLY, vars_))

    print(f"Switched to implementation: {target}")
    print(f"  workspace path : {impl_path}")
    print(f"  query language : {lang_config['language']}"
          f" (QueryLanguage::{lang_config['query_language_variant']})")
    print(f"  ANTLR crate    : {lang_config['antlr_crate_name']}")
    print("Generated artifacts:")
    for p in GENERATED_FILES:
        print(f"  - {p.relative_to(ROOT)}")
    print("\nNext step: cargo build  or  cargo run")
    return 0


def cmd_clean(_args: argparse.Namespace) -> int:
    removed = []
    for p in GENERATED_FILES:
        if p.exists():
            p.unlink()
            removed.append(p.relative_to(ROOT))
    if removed:
        print("Removed:")
        for p in removed:
            print(f"  - {p}")
    else:
        print("No generated artifacts to remove.")
    # Do not delete target/; leave that to cargo clean.
    return 0


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="build.py",
        description="metaGDB build system scaffold",
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("list", help="List all available implementation crates")
    sub.add_parser("current", help="Show the currently active implementation")

    p_use = sub.add_parser("use", help="Switch the active implementation and regenerate artifacts")
    p_use.add_argument(
        "crate",
        nargs="?",
        help="Implementation crate name to activate; uses the manifest value when omitted",
    )

    sub.add_parser("clean", help="Remove all script-generated activation artifacts")

    p_create = sub.add_parser(
        "create",
        help="Create an incomplete implementation shell for a coding agent",
    )
    p_create.add_argument("crate", help="New implementation crate name")
    p_create.add_argument(
        "--language",
        required=True,
        choices=sorted(SUPPORTED_LANGUAGES),
        help="Query language for the implementation",
    )

    p_verify = sub.add_parser(
        "verify",
        help="Activate an implementation, run tests, and execute its demo",
    )
    p_verify.add_argument("crate", help="Implementation crate name")
    p_verify.add_argument(
        "--timeout",
        type=int,
        default=120,
        help="Timeout in seconds for each verification step (default: 120)",
    )

    p_benchmark = sub.add_parser(
        "benchmark",
        help="Activate an implementation and run its fixed language benchmark",
    )
    p_benchmark.add_argument("crate", help="Implementation crate name")
    p_benchmark.add_argument(
        "--min-pass-rate",
        type=float,
        default=0.9,
        help="Required pass rate from 0 to 1 (default: 0.9)",
    )
    p_benchmark.add_argument(
        "--timeout",
        type=int,
        default=600,
        help="Benchmark timeout in seconds (default: 600)",
    )
    p_benchmark.add_argument(
        "--output-dir",
        help="Report directory (default: target/benchmark-run)",
    )

    p_system = sub.add_parser(
        "system-benchmark",
        help="Run fixed ACID, storage, recovery, and algorithm-registration benchmarks",
    )
    p_system.add_argument("crate", help="Implementation crate name")
    p_system.add_argument("--design", required=True, help="Finalized design vector JSON")
    p_system.add_argument(
        "--diagnostic",
        action="store_true",
        help="Write every failure but return success so an agent can inspect the first baseline",
    )
    p_system.add_argument(
        "--timeout",
        type=int,
        default=900,
        help="System benchmark timeout in seconds (default: 900)",
    )

    p_algorithm = sub.add_parser(
        "algorithm-check",
        help="Run agent-authored core and language tests for every requested algorithm",
    )
    p_algorithm.add_argument("crate", help="Implementation crate name")
    p_algorithm.add_argument("--design", required=True, help="Finalized design vector JSON")
    p_algorithm.add_argument(
        "--timeout", type=int, default=300,
        help="Timeout in seconds for each generated test (default: 300)",
    )

    p_micro = sub.add_parser(
        "micro-benchmark",
        help="Run fixed single-thread performance workloads selected by a design vector",
    )
    p_micro.add_argument("crate", help="Implementation crate name")
    p_micro.add_argument("--design", required=True, help="Finalized design vector JSON")
    selection = p_micro.add_mutually_exclusive_group()
    selection.add_argument(
        "--profile",
        choices=["smoke", "standard", "paper"],
        help="Run selected design datasets belonging to this cost tier",
    )
    selection.add_argument(
        "--datasets",
        help="Comma-separated focused subset of performance.datasets",
    )
    p_micro.add_argument("--output-dir", help="Report directory (default: target/performance-run/<crate>)")
    p_micro.add_argument(
        "--reuse-data",
        action="store_true",
        help="Reuse a scratch database whose graph, source fingerprint and design "
             "hash all match this run; import is then reported as reused instead of "
             "re-measured. Faster replicates, not acceptable as acceptance evidence",
    )
    p_micro.add_argument(
        "--timeout", type=int, default=3600,
        help="Timeout in seconds for each dataset (default: 3600)",
    )

    p_compare = sub.add_parser(
        "compare-performance",
        help="Compare baseline and candidate micro-benchmark report directories",
    )
    p_compare.add_argument("crate", help="Implementation crate name")
    p_compare.add_argument("--design", required=True, help="Finalized design vector JSON")
    p_compare.add_argument(
        "--baseline", action="append", required=True,
        help="Baseline report directory; repeat for replicate runs of the same "
             "code, which give each metric a measured noise floor",
    )
    p_compare.add_argument(
        "--candidate", action="append", required=True,
        help="Candidate report directory; repeat for replicate runs",
    )
    p_compare.add_argument(
        "--confirm-with",
        help="Previous comparison JSON from an independent measurement; a "
             "measured failure is fatal only if it is reproduced there",
    )
    p_compare.add_argument(
        "--allow-reused",
        action="store_true",
        help="Accept directories measured with --reuse-data; by default a "
             "comparison that contains reused metrics is refused",
    )
    p_compare.add_argument(
        "--allow-unstamped",
        action="store_true",
        help="Allow legacy directories without complete run.json metadata; "
             "exploratory use only",
    )
    p_compare.add_argument("--output", help="Comparison JSON output path")

    args = parser.parse_args(argv)
    dispatch = {
        "list": cmd_list,
        "current": cmd_current,
        "use": cmd_use,
        "clean": cmd_clean,
        "create": cmd_create,
        "verify": cmd_verify,
        "benchmark": cmd_benchmark,
        "system-benchmark": cmd_system_benchmark,
        "algorithm-check": cmd_algorithm_check,
        "micro-benchmark": cmd_micro_benchmark,
        "compare-performance": cmd_compare_performance,
    }
    return dispatch[args.cmd](args)


if __name__ == "__main__":
    sys.exit(main())
