# Symptom-indexed playbooks

Open only the file matching current evidence:

| Symptom | Playbook |
|---|---|
| generated parser import, missing context/accessor, parse-tree mismatch | `antlr_and_lowering.md` |
| Rust trait signature, object safety, ownership, or public export error | `rust_interfaces.md` |
| query parses but result/value/path/aggregation is wrong | `query_semantics.md` |
| restart, WAL, visibility, commit, rollback, or isolation failure | `storage_transactions.md` |
| fixes regress tests/score or repeated failures do not converge | `regression_control.md` |

These files contain debugging tactics, not extra requirements. Evidence and the
frozen design vector remain authoritative.
