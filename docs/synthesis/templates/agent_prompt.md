# Minimal coding-agent launch prompt

The repository owns the full autonomous execution contract. The user should not
repeat it or manually enumerate phases in every experiment. Replace
the input path and send only this prompt to a fresh coding-agent session
opened at the repository root:

> Read `AGENTS.md` and `docs/synthesis/README.md`. Use the requirement file or
> paper at `<input-path>` to start a new MetaGDB synthesis run; choose the target
> crate, language, and stable run ID from my request. Follow the full workflow,
> including upstream returns when the current stage exposes an earlier component
> mismatch, reviewed graph transitions, and final report, without routine user
> intervention.

When a finalized vector already exists, say “use the design vector at
`<design-vector-path>`” instead; the agent uses `synthesis.py init` rather than
`start`. Everything else, including ANTLR use, edit boundaries,
phase progression, autonomous repair, correctness gates, performance optimization,
snapshot rollback, stopping rules, and final evidence, comes from the two files
the prompt tells the agent to read.
