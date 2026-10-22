"""Paper text extraction and deterministic evidence-window selection."""

from __future__ import annotations

from pathlib import Path
import re
import shutil
import subprocess

from .requirements import RequirementError


SECTION_HINT = re.compile(
    r"^(?:\d+(?:\.\d+)*\s+)?(?:abstract|introduction|overview|system overview|"
    r"architecture|system architecture|design|storage|storage and indices|"
    r"query language and data model|transaction|contributions?)\b",
    re.IGNORECASE,
)
STOP_HINT = re.compile(r"^(?:\d+(?:\.\d+)*\s+)?references\b", re.IGNORECASE)


def extract_document_text(path: Path) -> str:
    if not path.exists():
        raise RequirementError(f"paper input does not exist: {path}")
    if path.suffix.lower() != ".pdf":
        text = path.read_text(encoding="utf-8")
    else:
        executable = shutil.which("pdftotext")
        if executable is None:
            raise RequirementError("PDF input requires the pdftotext executable")
        completed = subprocess.run(
            [executable, str(path), "-"],
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        if completed.returncode:
            raise RequirementError(
                f"pdftotext failed for {path}: {completed.stderr.strip()[:500]}"
            )
        text = completed.stdout
    if not text.strip():
        raise RequirementError(f"paper input contains no extractable text: {path}")
    return text


def evidence_window(text: str, *, max_chars: int = 60000) -> str:
    """Keep design-bearing paper regions and exclude references.

    The window is deterministic. It is not a semantic extractor: it reduces
    related-work and bibliography noise before deterministic or LLM extraction.
    """
    before_references = []
    for line in text.splitlines():
        if STOP_HINT.match(line.strip()):
            break
        before_references.append(line.rstrip())
    cleaned = "\n".join(before_references).strip()
    if len(cleaned) <= max_chars:
        return cleaned

    lines = cleaned.splitlines()
    selected: list[str] = []
    # The title, abstract, and introduction normally establish workload and language.
    prefix_budget = min(max_chars // 2, 24000)
    selected.append(cleaned[:prefix_budget])

    # Add bounded windows around architecture-bearing section headings.
    remaining = max_chars - sum(len(value) for value in selected)
    for index, line in enumerate(lines):
        if remaining <= 0:
            break
        if not SECTION_HINT.match(line.strip()):
            continue
        block = "\n".join(lines[index : index + 120]).strip()
        if not block or block in selected:
            continue
        block = block[: min(remaining, 12000)]
        selected.append(block)
        remaining -= len(block)
    return "\n\n".join(selected)[:max_chars]
