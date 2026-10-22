# Requirement-driven design synthesis

This module provides the requirement-intake operations within the unified
agent workflow. `scripts/synthesis.py start` calls these operations, validates
the intermediate design vector, and initializes the same candidate run:

    vague business needs, guided answers, a structured requirement document, or a paper
        -> evidence-bearing Requirement IR
        -> component capability graph and compatibility constraints
        -> feasible component compositions
        -> multi-objective ranking and active clarification
        -> finalized design_vector.json
        -> existing staged code-synthesis backend

The default constraint solver is deterministic. The English text adapter is also
fully local. An optional OpenAI-compatible adapter can fill extraction gaps, but
it may only propose evidence-cited Requirement Facts and paper-specific component
descriptions. It cannot emit the final design vector or bypass constraint
filtering, capability closure, ranking, or the backend validator. The separate
`--mode llm-direct` path lets the model own the full design vector and repair
it from backend validation feedback; it does not run the component solver.

## Algorithm

The Constraint-Guided GDBMS Design Synthesis algorithm performs six steps.

1. Normalize input into facts. Each fact records a field, value, hard or soft
   strength, confidence, evidence, and source.
2. Expand one option from each exclusive design dimension and add user-defined
   components to the same capability catalog.
3. Reject combinations that violate hard selections, component conflicts,
   compatibility relations, or required capabilities. Repeatedly add the
   lowest-risk compatible providers until capability closure is reached.
4. Score feasible designs using workload affinity, implementation complexity,
   maturity risk, and explicit preferences. Hard requirements are filtered
   before scoring and cannot be traded for performance.
5. Compute the Pareto set and rank candidates with stable component-id
   tie-breaking. When leading candidates differ, rank business-level questions
   by candidate reduction, design impact, and user burden.
6. Render the chosen composition into the backend design-vector schema and run
   `scripts/synthesis.py check`. Alternatives, evidence, assumptions, input and
   catalog hashes, and optional model provenance are retained.

The extractor is an input adapter. The research method is the domain capability
model, compatibility closure, multi-objective component composition, and
impact-directed clarification.

## Storage is an open composition

Storage is not a closed profile enum. The built-in catalog composes a record
layout, update path, recovery mechanism, indexes, version support, and analytical
projection. A requirement may register a component with:

- id, dimension, and description;
- emitted backend description;
- provided and required capabilities;
- compatible and conflicting components;
- objective affinities, complexity, risk, and maturity.

`storage.required_components` can freeze several paper-defined layers as one
design. The solver still verifies dependencies and compatibility. A strict schema
must supply concrete `vertex_labels` and `edge_types`; the system does not
invent them.

## Interfaces

### Reproducible structured input

```bash
python3 scripts/design.py compile \
  design_synthesis/examples/financial_strict.json \
  --run-id financial-01 --output design_vector.json
```

### Natural business requirements

```bash
python3 scripts/design.py from-text requirements.txt \
  --crate metagdb_social_01 \
  --run-id social-01 --output design_vector.json
```

Add `--llm-fallback` when the bounded local rules leave important facts
unresolved:

```bash
python3 scripts/design.py from-text requirements.txt \
  --crate metagdb_social_01 --llm-fallback \
  --run-id social-llm-01 --output design_vector.json
```

### Paper-derived design

PDF input requires the `pdftotext` executable. A deterministic evidence window
keeps the abstract, introduction, and architecture-bearing sections while
excluding references. For faithful paper-specific storage or execution layers,
use the evidence-constrained adapter:

```bash
python3 scripts/design.py from-paper data/paper/AsterDB.pdf \
  --crate metagdb_aster_reproduction --language Gremlin \
  --llm-fallback --run-id aster-paper-01 --output design_vector.json
```

A paper may mention several systems and languages. `--language` identifies the
target being reproduced; omitting it is safe only when exactly one language is
identified or the optional extractor resolves it. Every model-proposed fact and
custom component must quote text present in the retained evidence window.
PDF line-break hyphenation, Unicode compatibility characters, and punctuation
differences are normalized before matching. A proposal item whose quote still
cannot be located is removed from the Requirement IR and constraint solver rather
than aborting the entire run. Every removal is retained under `rejected` in
`llm_proposal.json` and under `Rejected or unresolved input` in the decision
report. A successful command therefore means that a safe design was produced; a
faithful paper-reproduction claim additionally requires reviewing and resolving
every material rejected or unknown item.
If a paper-required component is rejected, the command fails and retains only a
`diagnostic_vector.json` in the run directory; it does not publish a finalized vector.

### Full LLM design with validation and repair

Use this mode when a paper's architecture cannot be represented reliably by the
component solver. It requires the configured OpenAI-compatible API, emits the
complete vector through the model, checks the exact backend schema and fixed
gates, checks source quotes, then asks the model for an independent full-design
review. Invalid results receive validator feedback for bounded repair. No final
vector is published unless review and validation pass.

```bash
python3 scripts/design.py from-paper data/paper/KUZU.pdf \
  --crate metagdb_kuzu_direct_01 --language Cypher \
  --mode llm-direct --max-repair-rounds 3 \
  --run-id kuzu-direct-01 --output design_vector.kuzu_direct_01.json
python3 scripts/synthesis.py check --design design_vector.kuzu_direct_01.json
```

`from-text` accepts the same flags. `--llm-fallback` belongs to constraint mode
and is unnecessary in direct mode. The model must record quotes or assumptions
for storage, recovery, isolation, execution, and capabilities. Query join
operators are design capabilities, not standalone graph-analysis algorithms.
Backend validation proves schema and gate compatibility; source quote matching
alone cannot prove paper fidelity. Review `decision_report.md` and compare the
vector with the paper before beginning the expensive E2E synthesis.
For Kùzu, the fixed performance suite measures one-hop queries and adjacency
access as proxies for join-heavy execution; it has no dedicated multiway join
latency metric. The vector therefore requires direct and public-language
correctness tests for ASP-Join, S-Join, factorized intermediates, and multiway
joins. Do not present the proxy performance score as a paper-level multiway
join speed comparison.
If the API drops a request or the bounded repair loop ends, continue the same
failed run with `--resume --run-id kuzu-direct-01`; source hash and model must
match. The previous attempts remain in the evidence bundle.

### Guided interaction

```bash
python3 scripts/design.py guided \
  --crate metagdb_guided_01 --run-id guided-01 \
  --output design_vector.json
```

The interface asks for language, graph topology, workload focus, then repeatedly
asks the highest-value design question. Strict schema selections also require
concrete labels and edge types. The final prompt accepts optional algorithm
names.

All finalized outputs can enter the backend with:

```bash
python3 scripts/synthesis.py init --design design_vector.json
```

## Optional LLM configuration

`.env` is ignored by Git. `.env.example` is the tracked template:

```dotenv
LLM_API_BASE=https://api.deepseek.com
LLM_API_KEY=
LLM_MODEL=deepseek-chat
LLM_TEMPERATURE=0
LLM_TIMEOUT_SECONDS=120
```

The endpoint must implement the OpenAI-compatible
`POST /chat/completions` protocol. The API is contacted when
`--llm-fallback` or `--mode llm-direct` is present. The input text or retained
paper evidence window is sent to that endpoint. No key is written to evidence artifacts.

## Run evidence

Each ignored `design_runs/<run-id>/` directory contains:

- `raw_input.txt`, the business text or retained paper evidence window
  (present for `from-text` and `from-paper`);
- `requirements.json`, the normalized Requirement IR;
- `candidates.json`, ranked feasible alternatives;
- `clarification.jsonl`, unresolved questions ordered by value;
- `decision_trace.json`, constraint and scoring trace;
- `design_vector.json`, the backend-compatible result;
- `decision_report.md`, the human-readable explanation;
- `provenance.json`, hashes, unresolved-item count, and optional model/proposal identity;
- `llm_proposal.json` only when the optional adapter was used; its `rejected`
  array records every model item excluded by evidence or structure validation.

Direct-mode runs instead contain `attempt_NN_response.json`,
`attempt_NN_vector.json`, and `attempt_NN_feedback.json` for every attempt,
plus `raw_input.txt`, `decision_report.md`, `provenance.json`, and a final
`design_vector.json` only when accepted. They have no solver candidates or
normalized Requirement IR artifact. Malformed model JSON is retained as
`attempt_NN_response.txt` with validation feedback.

## Verification and research boundary

Unit, integration, scenario, paper-window, and mocked OpenAI-compatible tests live
under `design_synthesis/tests`. They cover hard-constraint precedence,
incompatibility rejection, custom storage extension, deterministic ranking,
evidence validation, backend compatibility, common business scenarios, and model
provenance without requiring a network or paid token.

Cypher, GQL, and Gremlin are the languages currently connected to generated
parsers and fixed profiles. Algorithms remain open user requirements. Paper
extraction is assistance rather than an oracle: the report must be reviewed
against cited paper evidence before claiming a faithful reproduction. The
backend validation and downstream benchmarks remain mandatory in both modes.
