#!/usr/bin/env python3
"""
metaGDB language automation tool.

Responsibilities:
  - list      : List available query languages under data/language/.
  - generate  : Run antlr4rust from .g4 grammar files to generate Rust code,
                and automatically create lib.rs and Cargo.toml.
  - configure : Create or update scripts/config/<crate>.toml,
                binding an implementation crate to a specified query language.

Usage workflow:
  # 1) View available languages
  ./scripts/language.py list

  # 2) Generate ANTLR Rust code from .g4
  ./scripts/language.py generate Cypher

  # 3) Bind an implementation crate to a language
  ./scripts/language.py configure metagdb_default Cypher

  # 4) Switch implementation and compile as usual
  ./scripts/build.py use metagdb_default
  cargo build

Note: The implementation crate's own parser.rs must correctly reference the
      corresponding language's ANTLR crate (done by LLM or developer).
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path


# ---------------------------------------------------------------------------
# Language name → QueryLanguage variant mapping
# ---------------------------------------------------------------------------
#
# Must be strictly consistent with the QueryLanguage enum definition in
# metagdb_core/src/l5_interaction.rs. Language names not in the mapping
# table will be rejected outright.

_LANGUAGE_VARIANT_MAP = {
    "Cypher":   "Cypher",
    "GQL":      "Gql",
    "Gremlin":  "Gremlin",
}


def language_to_variant(lang_name: str) -> str:
    """Map a language name to its QueryLanguage enum variant name.

    @param lang_name: Language name (e.g., "Cypher", "GQL", "Gremlin")
    @return: QueryLanguage enum variant name (e.g., "Cypher", "Gql", "Gremlin")
    @raise SystemExit: Exit with error if the language name is not in the mapping table
    """
    variant = _LANGUAGE_VARIANT_MAP.get(lang_name)
    if variant is None:
        raise SystemExit(
            f"Error: unsupported language '{lang_name}'.\n"
            f"  Supported languages: {', '.join(sorted(_LANGUAGE_VARIANT_MAP.keys()))}\n"
            f"  To add a new language, first add the corresponding variant to the\n"
            f"  QueryLanguage enum in metagdb_core/src/l5_interaction.rs,\n"
            f"  then add a mapping entry to _LANGUAGE_VARIANT_MAP in this file."
        )
    return variant


# ---------------------------------------------------------------------------
# Path constants
# ---------------------------------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
DATA_LANG_DIR = ROOT / "data" / "language"
LANG_IMPL_DIR = ROOT / "metagdb_src" / "implementation" / "language"
CONFIG_DIR = ROOT / "scripts" / "config"
IMPL_DIR = ROOT / "metagdb_src" / "implementation"
GEN_DIR = IMPL_DIR / "generated"


# ---------------------------------------------------------------------------
# Helper functions
# ---------------------------------------------------------------------------

def _find_g4_files(language: str) -> list[Path]:
    """Find all .g4 grammar files for the specified language under data/language/.

    Supports two naming patterns:
      - Single-file grammar: <Name>.g4 (e.g., Cypher.g4, Gremlin.g4, GQL.g4)
      - Split grammar: <Name>Lexer.g4 + <Name>Parser.g4

    @param language: Language name
    @return: List of matching .g4 file paths (sorted)
    @raise SystemExit: Exit with error if no .g4 files are found
    """
    patterns = [
        DATA_LANG_DIR / f"{language}.g4",
        DATA_LANG_DIR / f"{language}Lexer.g4",
        DATA_LANG_DIR / f"{language}Parser.g4",
    ]
    g4_files = sorted({p for p in patterns if p.is_file()})
    if not g4_files:
        available = _list_available_languages()
        raise SystemExit(
            f"Error: no .g4 grammar files found for language '{language}' under data/language/.\n"
            f"  Expected file: {DATA_LANG_DIR / language}.g4\n"
            f"  Or split grammar: {DATA_LANG_DIR / language}Lexer.g4 + "
            f"{DATA_LANG_DIR / language}Parser.g4\n"
            f"  Available languages: {', '.join(available) if available else '(none)'}"
        )
    return g4_files


def _list_available_languages() -> list[str]:
    """Scan all .g4 files under data/language/ and extract unique language names.

    @return: Sorted list of language names
    """
    languages: set[str] = set()
    for g4_file in DATA_LANG_DIR.glob("*.g4"):
        stem = g4_file.stem
        # Strip Lexer / Parser suffix to obtain the base language name
        for suffix in ("Parser", "Lexer"):
            if stem.endswith(suffix):
                stem = stem[:-len(suffix)]
                break
        languages.add(stem)
    return sorted(languages)


def _crate_name_from_language(language: str) -> str:
    """Derive the ANTLR crate's Rust package name by convention.

    Convention format: metagdb_<lowercase_language_name>_antlr
    Example: "Cypher" → "metagdb_cypher_antlr"

    @param language: Language name
    @return: Derived crate name
    """
    return f"metagdb_{language.lower()}_antlr"


def _parse_simple_toml(text: str) -> dict[str, str]:
    """Parse simple TOML text in `key = "value"` format.

    Ignores comments (lines starting with #) and blank lines; silently skips
    other forms. Consistent with the implementation in scripts/build.py.

    @param text: TOML text
    @return: Key-value dictionary
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


def _get_antlr4rust_cmd() -> list[str]:
    """Get the antlr4rust command.

    First looks for the antlr4rust executable in PATH (cargo install antlr4rust);
    if not found, tries to parse alias antlr4rust='...' from ~/.bashrc to obtain
    the actual java -jar command.

    @return: Command prefix list, e.g., ["antlr4rust"] or
             ["java", "-jar", "/path/to/antlr4-rust.jar", "-Dlanguage=Rust"]
    @raise SystemExit: Exit if neither method finds the command
    """
    # Method 1: executable in PATH (cargo install antlr4rust)
    if shutil.which("antlr4rust") is not None:
        return ["antlr4rust"]

    # Method 2: parse alias from ~/.bashrc (java -jar approach)
    bashrc = Path.home() / ".bashrc"
    if bashrc.is_file():
        for line in bashrc.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            # Match: alias antlr4rust='java -jar /path/to/antlr4-rust.jar ...'
            if line.startswith("alias antlr4rust="):
                # Extract the alias value after the '='
                _, _, alias_value = line.partition("=")
                alias_value = alias_value.strip()
                # Strip surrounding quotes (single or double)
                if (
                    alias_value.startswith("'") and alias_value.endswith("'")
                ) or (
                    alias_value.startswith('"') and alias_value.endswith('"')
                ):
                    alias_value = alias_value[1:-1]
                # Split into command list by whitespace
                cmd_parts = alias_value.split()
                if cmd_parts:
                    return cmd_parts

    raise SystemExit(
        "Error: antlr4rust command not found.\n"
        "  Please install via one of the following methods:\n"
        "    1. cargo install antlr4rust\n"
        "    2. Add an alias in ~/.bashrc:\n"
        "       alias antlr4rust='java -jar /path/to/antlr4-rust.jar -Dlanguage=Rust'"
    )


# ---------------------------------------------------------------------------
# Subcommand: list
# ---------------------------------------------------------------------------

def cmd_list(_args: argparse.Namespace) -> int:
    """List available query languages under data/language/.

    Languages that already have a generated ANTLR crate directory are marked with *.
    """
    languages = _list_available_languages()
    if not languages:
        print("(No .g4 grammar files found under data/language/)")
        return 0

    print(f"Available languages ({len(languages)} total):")
    for lang in languages:
        lang_dir = LANG_IMPL_DIR / lang
        marker = "*" if (lang_dir / "lib.rs").is_file() else " "
        print(f"  {marker} {lang}")
    return 0


# ---------------------------------------------------------------------------
# Subcommand: generate
# ---------------------------------------------------------------------------

def cmd_generate(args: argparse.Namespace) -> int:
    """Run antlr4rust from .g4 grammar files to generate Rust code,
    and automatically create lib.rs and Cargo.toml.

    Steps:
      1. Locate the .g4 source files
      2. Resolve the antlr4rust command (PATH executable or ~/.bashrc alias)
      3. Create output directory and run antlr4rust
      4. Scan generated .rs modules
      5. Generate lib.rs and Cargo.toml
    """
    language = args.language

    # 1. Locate .g4 source files
    g4_files = _find_g4_files(language)

    # 2. Get antlr4rust command (supports PATH executable or ~/.bashrc alias)
    antlr_cmd = _get_antlr4rust_cmd()

    # 3. Create output directory
    out_dir = LANG_IMPL_DIR / language
    out_dir.mkdir(parents=True, exist_ok=True)

    # 4. Run antlr4rust (for each .g4 file)
    #    Command format: antlr4rust -o <out_dir> <grammar.g4>
    for g4_file in g4_files:
        print(f"  Running antlr4rust: {g4_file.name} -> {out_dir.relative_to(ROOT)}")
        result = subprocess.run(
            antlr_cmd + ["-o", str(out_dir), str(g4_file)],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print(f"Error: antlr4rust failed ({g4_file.name})", file=sys.stderr)
            if result.stderr:
                print(result.stderr, file=sys.stderr)
            return result.returncode

    # 5. Scan generated modules (excluding lib.rs and mod.rs)
    modules = sorted(
        p.stem for p in out_dir.glob("*.rs")
        if p.name not in ("lib.rs", "mod.rs")
    )
    if not modules:
        print(
            f"Warning: no .rs generated files found under {out_dir.relative_to(ROOT)}.\n"
            f"  Please check whether the antlr4rust command produced output files.",
            file=sys.stderr,
        )
        return 1

    # 6. Generate Cargo.toml
    crate_name = _crate_name_from_language(language)
    cargo_toml = (
        "# Auto-generated by scripts/language.py; do not modify manually.\n"
        "# Regenerate with: ./scripts/language.py generate " + language + "\n"
        "\n"
        "[package]\n"
        f'name = "{crate_name}"\n'
        'version = "0.1.0"\n'
        'edition = "2021"\n'
        "\n"
        "[lib]\n"
        'path = "lib.rs"\n'
        "\n"
        "[dependencies]\n"
        'antlr4rust = "0.5.2"\n'
        "\n"
        "[lints.rust]\n"
        'unused_parens = "allow"\n'
    )
    (out_dir / "Cargo.toml").write_text(cargo_toml, encoding="utf-8")

    # 7. Generate lib.rs
    mod_lines = "\n".join(f"pub mod {m};" for m in modules)
    lib_rs = (
        "/**\n"
        f" * @brief: {crate_name}: ANTLR4-generated {language} frontend wrapper.\n"
        " *\n"
        " * This file is auto-generated by scripts/language.py; do not modify manually.\n"
        f" * Regenerate with: ./scripts/language.py generate {language}\n"
        " *\n"
        " * Unified exports for lexer / parser / listener / base_listener modules,\n"
        " * to be referenced by backend implementation crates via workspace dependency.\n"
        " */\n"
        "\n"
        + mod_lines +
        "\n"
    )
    (out_dir / "lib.rs").write_text(lib_rs, encoding="utf-8")

    print(f"  Generated {len(modules)} modules: {', '.join(modules)}")
    print(f"  ANTLR crate: {out_dir.relative_to(ROOT)}")
    print(f"  Cargo.toml  : {out_dir.relative_to(ROOT)}/Cargo.toml")
    print(f"  lib.rs      : {out_dir.relative_to(ROOT)}/lib.rs")
    return 0


# ---------------------------------------------------------------------------
# Subcommand: configure
# ---------------------------------------------------------------------------

def cmd_configure(args: argparse.Namespace) -> int:
    """Create or update the language configuration file for a given implementation crate.

    The configuration file is written to scripts/config/<crate>.toml and records:
      - Language name
      - QueryLanguage enum variant name
      - ANTLR crate name and directory path
    """
    crate = args.crate
    language = args.language

    # 1. Verify that the implementation crate exists
    impl_dir = IMPL_DIR / crate
    gen_dir = GEN_DIR / crate
    if not (impl_dir / "Cargo.toml").is_file() and not (gen_dir / "Cargo.toml").is_file():
        # Scan available crates
        candidates: list[str] = []
        for entry in IMPL_DIR.iterdir():
            if entry.is_dir() and entry.name != "generated" and (entry / "Cargo.toml").is_file():
                candidates.append(entry.name)
        if GEN_DIR.is_dir():
            for entry in GEN_DIR.iterdir():
                if entry.is_dir() and (entry / "Cargo.toml").is_file():
                    candidates.append(entry.name)
        raise SystemExit(
            f"Error: implementation crate '{crate}' does not exist.\n"
            f"  Available implementations: {', '.join(sorted(candidates)) if candidates else '(none)'}"
        )

    # 2. Verify that the language name is in the mapping table
    variant = language_to_variant(language)

    # 3. Verify that the ANTLR crate has been generated
    lang_dir = LANG_IMPL_DIR / language
    if not (lang_dir / "lib.rs").is_file():
        raise SystemExit(
            f"Error: the ANTLR crate for language '{language}' has not been generated yet.\n"
            f"  Expected directory: {lang_dir.relative_to(ROOT)}\n"
            f"  Please run first: ./scripts/language.py generate {language}"
        )

    # 4. Read the real crate name from Cargo.toml (fallback to convention-derived name)
    cargo_toml_path = lang_dir / "Cargo.toml"
    if cargo_toml_path.is_file():
        pkg = _parse_simple_toml(cargo_toml_path.read_text(encoding="utf-8"))
        crate_name = pkg.get("name", _crate_name_from_language(language))
    else:
        crate_name = _crate_name_from_language(language)

    # 5. Assemble config and write TOML
    crate_dir_rel = lang_dir.relative_to(ROOT).as_posix()
    config_text = (
        "# metaGDB language config —— generated by scripts/language.py configure; do not modify manually.\n"
        f"language = \"{language}\"\n"
        f"query_language_variant = \"{variant}\"\n"
        f"antlr_crate_name = \"{crate_name}\"\n"
        f"antlr_crate_dir = \"{crate_dir_rel}\"\n"
    )

    CONFIG_DIR.mkdir(parents=True, exist_ok=True)
    config_path = CONFIG_DIR / f"{crate}.toml"
    config_path.write_text(config_text, encoding="utf-8")

    print(f"Configured implementation crate '{crate}' to use language '{language}'.")
    print(f"  Config file        : {config_path.relative_to(ROOT)}")
    print(f"  Language variant   : QueryLanguage::{variant}")
    print(f"  ANTLR crate        : {crate_name}")
    print(f"  ANTLR crate dir    : {crate_dir_rel}")
    return 0


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="language.py",
        description="metaGDB language automation tool: ANTLR code generation + implementation crate language configuration",
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("list", help="List available query languages under data/language/")

    p_gen = sub.add_parser("generate", help="Generate ANTLR Rust code from .g4 grammar files")
    p_gen.add_argument("language", help="Language name (e.g., Cypher, GQL, Gremlin)")

    p_cfg = sub.add_parser("configure", help="Bind an implementation crate to a specified query language")
    p_cfg.add_argument("crate", help="Implementation crate name (e.g., metagdb_default)")
    p_cfg.add_argument("language", help="Language name (e.g., Cypher, GQL, Gremlin)")

    args = parser.parse_args(argv)
    dispatch = {
        "list": cmd_list,
        "generate": cmd_generate,
        "configure": cmd_configure,
    }
    return dispatch[args.cmd](args)


if __name__ == "__main__":
    sys.exit(main())
