from __future__ import annotations

import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from types import SimpleNamespace
from unittest.mock import patch

from design_synthesis.paper import evidence_window, extract_document_text


class PaperTests(unittest.TestCase):
    def test_evidence_window_drops_references(self):
        text = """Title
Abstract
A graph database uses Gremlin and a hybrid LSM layout.
1 Introduction
The system supports intensive graph updates.
References
[1] A paper mentioning Cypher and unrelated systems.
"""
        window = evidence_window(text)
        self.assertIn("hybrid LSM", window)
        self.assertNotIn("unrelated systems", window)

    def test_window_is_bounded(self):
        text = "Abstract\n" + ("graph storage design " * 1000)
        self.assertLessEqual(len(evidence_window(text, max_chars=500)), 500)

    def test_pdf_extracts_reading_order_without_layout_interleaving(self):
        with TemporaryDirectory() as directory:
            path = Path(directory) / "two_columns.pdf"
            path.write_bytes(b"pdf fixture")
            with patch("design_synthesis.paper.shutil.which", return_value="pdftotext"), patch(
                "design_synthesis.paper.subprocess.run",
                return_value=SimpleNamespace(returncode=0, stdout="A complete sentence from one column.", stderr=""),
            ) as runner:
                self.assertIn("complete sentence", extract_document_text(path))
            self.assertEqual(runner.call_args.args[0], ["pdftotext", str(path), "-"])


if __name__ == "__main__":
    unittest.main()
