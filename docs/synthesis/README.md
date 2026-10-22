# File-guided staged GDBMS synthesis

This is the sole agent workflow for turning requirements, a paper, or a finalized
design vector into one runnable metaGDB implementation. A persistent coding agent
performs the work. `synthesis.py start` runs the existing requirement-intake
adapter and freezes its validated design vector before code generation; the
design vector is an intermediate artifact of the same workflow.

The user's launch prompt only needs to identify this entry point and the design
vector. All implementation, repair, validation, performance, rollback, and
reporting requirements are owned by this repository so experiment behavior does
not depend on how much detail a user repeats in each session.

## Autonomous execution contract

Reading this entry point authorizes and requires the coding agent to complete the
entire workflow without routine user intervention:

- Treat the supplied design vector as immutable experiment input. Validate it,
  initialize or resume its named candidate, and never lower its thresholds or
  remove requirements to obtain a pass.
- Keep hand-written implementation edits inside that candidate directory. Fixed
  tools may create their documented activation, configuration, state, snapshot,
  and `target/` evidence artifacts; do not edit those generated files manually.
  Do not read or copy external research notes, other generated candidates,
  their snapshots, or reports from other runs. They are not synthesis inputs
  and would contaminate a clean run.
- Use the repository-generated ANTLR lexer/parser as the only syntax frontend,
  invoke its real top-level rule, and lower its CST to a private AST. A handwritten
  replacement parser or benchmark-text dispatch is invalid even if tests pass.
- Follow the generation dependencies in `scripts/workflow_graph.py`. For every
  new-run phase, write a structured self-review naming observations, evidence
  limits, remaining risks and the next action. `advance --review` executes a
  fixed gate and records its log and revision before selecting the next ready
  node. A failed gate keeps the phase active; inspect its gate log, repair the
  current component, and rerun the check. If local repair cannot resolve a
  diagnosed upstream interface or design mismatch, write a review identifying
  its owner and use `revisit --to` from the active generation stage. Revalidate
  the owner, affected consumers, and the current node.
  The same return route applies in integration/performance. Make routine
  decisions independently.
- Treat compiler errors, failed tests, low scores, and unsupported required
  semantics as work items. Read complete reports, group failures by root cause,
  repair the candidate, and rerun focused plus full regressions without asking the
  user to choose fixes.
- Phase 12 must finish with passing `verify`, the design's language threshold,
  100% of applicable fixed system/ACID cases, and the requested warning policy.
  Phase 13 then establishes a performance baseline, uses isolated candidate
  snapshots, and accepts only changes that satisfy weighted goals and every
  correctness and regression guardrail.
- Never gain a pass or speedup by editing fixed inputs, weakening tests, hiding
  errors, returning fabricated/empty results, or removing durability, recovery,
  validation, isolation, or conflict detection.
- Stop unsuccessfully only for a concrete reproducible environmental blocker or
  an exhausted explicit nonzero budget. Before stopping, record attempted remedies
  and exact evidence. Otherwise continue through the final regression and complete
  `.synthesis/final_report.md` with exact results and unsupported optional scope.

## Start or resume

1. Read `AGENTS.md`, this file, `global_contract.md`, and `workflow.md`.
   When algorithms are requested, also read `algorithm_evidence.md`; algorithm
   names and descriptions come from the user and are never limited to a catalog.
   Read `snapshots.md` before the first phase so candidate-local rollback behavior
   is understood without using the outer research Git.
2. For a new run from a frozen vector, execute:

   ```bash
   python3 scripts/synthesis.py check --design <design-vector.json>
   python3 scripts/synthesis.py init --design <design-vector.json>
   ```

   Initialization also creates the candidate's independent snapshot repository
   and the `initialized`/`best` refs. Do not initialize a second Git repository
   manually.

   For an end-to-end run from natural language or a paper, use one entry instead:

   ```bash
   python3 scripts/synthesis.py start --text <requirements.txt> --crate <crate> --language Cypher --run-id <run-id>
   # Or use --paper <paper.pdf>; structured JSON uses --requirements <requirements.json>.
   ```

   Existing `from-text`/`from-paper` design commands remain usable for design
   review. The design artifact and its source hash are frozen inside the new run.

3. Run `python3 scripts/synthesis.py status <crate>` and read only:
   - the frozen `.synthesis/design_vector.json`;
   - `.synthesis/state.md`, `capability_matrix.tsv`, and `decisions.md`;
   - the current file under `phases/`;
   - the dependencies and framework sources named by that phase.
4. Satisfy the phase contract, then create
   `.synthesis/reviews/<phase>.json` from `templates/phase_review.example.json`.
   Explain what passed, why it is enough, what remains uncertain, and the next
   action. The phase-00 review must also assess the frozen design vector against
   its source, validator/decision evidence, remaining assumptions, and
   implementation feasibility. The tool rejects a changed design-vector hash.
   Run `scripts/synthesis.py advance <crate> --phase <phase> --review
   <candidate>/.synthesis/reviews/<phase>.json`. The tool reruns its fixed gate,
   records the check output and code revision, and creates the phase snapshot.
   New runs require this review; initialized v1 candidates keep `--evidence`.
   If a failure or cross-layer mismatch in any active phase belongs to an earlier
   component, write a review from `templates/phase_feedback.example.json` with `decision: revisit`,
   a concrete `diagnosis`, and the agent-selected earlier `target` (the tool
   checks that the target precedes the current phase), then
   run `scripts/synthesis.py revisit <crate> --phase <phase> --to <earlier-phase>
   --review <candidate>/.synthesis/reviews/<phase>.json`.
   Do not ask the user to inspect output, select fixes, or approve routine
   decisions.
5. In phase 12, run the autonomous correctness loop until every semantic and ACID
   criterion passes. In phase 13, use fixed micro-benchmarks and isolated
   candidate snapshots to optimize the graph scenarios and operations weighted
   by the design vector. Read a `playbooks/` file only when its indexed symptom
   applies.

The example input is `templates/design_vector.example.json`. Copy it outside the
generated candidate, change the crate and design choices, validate it, then begin.
`templates/agent_prompt.md` is the concise initial prompt for a fresh coding-agent
session.

## Context discipline

Do not load every phase or playbook at once. The design vector, framework contract,
current state, current phase, and named dependency outputs are the working set.
After phase 11, synthesis recipes are historical context: use real compiler,
test, demo, and benchmark evidence to drive repairs.

## Definition of complete

A run is complete only when all of these are true:

- every required capability has executable semantic evidence;
- `python3 scripts/build.py verify <crate>` succeeds;
- the full external benchmark meets `quality.benchmark_min_pass_rate`;
- the fixed system benchmark passes every applicable atomicity, consistency,
  isolation, durability, storage, and required-algorithm case;
- every required algorithm has passing agent-authored direct and public-language
  correctness tests recorded by `algorithm-check`;
- the final candidate passes its per-dataset regression guardrails and configured
  absolute targets; an improvement claim requires an `ACCEPTED` comparison.
  A solely noise-limited comparison may finish as `completed_with_limitations`
  under the phase-13 evidence rules and must decline an improvement claim;
- the full suite still passes after the final repair;
- warnings satisfy `quality.zero_compiler_warnings`;
- `.synthesis/final_report.md` gives exact results and honest unsupported cases.

Parser acceptance alone is never semantic support. Continue autonomously until
these gates pass. Stop unsuccessfully only for a reproducible environmental
blocker or an explicitly exhausted nonzero budget, and document the blocker and
all attempted remedies in the final report.
