# Isolated candidate snapshot protocol

Candidate snapshots are part of the synthesis method. They preserve a recoverable
implementation and workflow state after each passed phase, and they let the agent
try repairs or performance changes without using the research repository's Git.

## Two independent repositories

```text
research repository                         ignored candidate snapshot repository
<workspace>/.git                    metagdb_src/implementation/generated/
  tracks framework, workflow, benchmark       .snapshots/<crate>.git
  never receives candidate experiments        work tree: ../<crate>/
                                                tracks code + .synthesis state
```

Every snapshot command explicitly supplies both `--git-dir` and `--work-tree`.
It never searches parent directories for `.git`. The tool also verifies that the
work tree is exactly one direct child of `generated/`, and restoration cleans only
that verified candidate directory.

The research repository commits this protocol, `scripts/snapshot.py`, and its
integration with `scripts/synthesis.py`. It does not commit generated candidates,
their snapshot object databases, activation configs, benchmark reports, or local
datasets. Those paths remain ignored experiment state.

Deleting only `generated/<crate>/` leaves the independent snapshot history
behind. This makes an accidental source-directory deletion recoverable.

## Automatic lifecycle

1. `scripts/synthesis.py init` creates the candidate and initializes its isolated
   history. The first commit receives `checkpoints/initialized`; `best` initially
   points to the same commit.
2. After writing a self-review, the agent runs `scripts/synthesis.py advance`.
   New-run transitions execute fixed checks and bind the phase review and gate
   report to the current candidate revision before advancing. A failed check
   leaves the phase active. `revisit` records a generation-time upstream return
   transition and
   invalidates only the repaired node's downstream consumers.
   `advance` updates `.synthesis/state.json` and `state.md`, commits all candidate
   code and state, and writes `checkpoints/<phase>`. If snapshot creation fails,
   the phase state is not advanced.
3. After phase 12 has passed all correctness gates, phase 13 marks that correct
   baseline as `best`.
4. Each performance hypothesis or risky repair may receive a manual named
   checkpoint. An accepted version moves `best`; a rejected version restores it.
5. The phase-13 advance records the completed report and final workflow state.

No manual checkpoint is needed at a normal phase boundary: a successful
`synthesis.py advance` performs it. Do not advance before the phase gate passes.

## Commands available to the agent

```bash
# Inspect the candidate's independent history and uncommitted files.
python3 scripts/snapshot.py status <crate>
python3 scripts/snapshot.py list <crate>
python3 scripts/snapshot.py log <crate> --count 20

# Record an intra-phase repair or experiment.
python3 scripts/snapshot.py checkpoint <crate> \
  --name before_storage_rewrite --label "before storage layout rewrite"

# Restore code and .synthesis state together.
python3 scripts/snapshot.py restore <crate> 02_storage
python3 scripts/snapshot.py restore <crate> initial
python3 scripts/snapshot.py restore-best <crate>

# Promote the clean current snapshot after all relevant gates pass.
python3 scripts/snapshot.py mark-best <crate>
```

`restore` accepts `best`, `initial`, a named checkpoint, or a snapshot commit ID.
It deliberately discards uncommitted and untracked files inside that candidate,
then restores both its source and `.synthesis` state. Inspect or checkpoint useful
work first. Named phase refs remain available even when `main` moves backward, so
the agent can compare or return to later checkpoints.

`micro-benchmark`, `restore`, and `restore-best` acquire the same atomic
candidate-code lock. Measurement holds it before activation and compilation until
all requested reports are complete; restore holds it across the Git reset and
clean. Thus neither operation can enter the other's check-to-action window. The operating system releases the lock automatically when either process exits,
including after a crash; the file itself remains only as owner diagnostics.

## Agent rules

- Use `synthesis.py advance` after every passed phase; it is the canonical phase
  transition and snapshot operation.
- Use manual checkpoints inside a phase when a risky change would otherwise make
  rollback expensive. Use concise names tied to the hypothesis or repair.
- Run the relevant gate before moving `best`. A snapshot records evidence; it
  does not prove correctness by itself.
- Never run experiment `git commit`, `git reset`, `git clean`, `git stash`, or
  branch commands against the outer research repository.
- Never read a sibling candidate or its snapshot as synthesis input. This keeps
  independent experimental runs from leaking implementation knowledge.
