# MetaGDB design synthesis package

The implementation and tests for requirement-to-design-vector synthesis live
here. The deterministic solver supports structured requirements, bounded natural
text extraction, impact-directed guided questions, and paper-specific catalog
extensions. In the default constraint mode, an optional OpenAI-compatible
adapter proposes evidence-cited facts. The separate `llm-direct` mode asks
the model to generate the complete design vector and repair validation errors.

Read `docs/design_synthesis/README.md` for commands, configuration, evidence
artifacts, and research boundaries.
