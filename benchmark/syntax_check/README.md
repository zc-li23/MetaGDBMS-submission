# Grammar-backed profile check

This standalone Rust tool classifies each Core Profile case with the language's generated ANTLR
lexer/parser. It uses the complete entry rules used by generated candidates:
Cypher `oC_Cypher()`, GQL `gqlRequest()`, and Gremlin `queryList()`.
Consequently, valid prefixes followed by unconsumed garbage are rejected.

Check a full profile and optionally write the machine-readable applicability
report used by `scripts/build.py benchmark`:

```bash
cargo run --manifest-path benchmark/syntax_check/Cargo.toml -- \
    --language gql benchmark/gql/core.jsonl \
    --output target/benchmark-run/grammar.tsv
```

The TSV records whether each case is scored, negative, accepted by the grammar,
and applicable to a grammar-conditional score. A positive case is applicable
when the grammar accepts it; a negative parse case is applicable when the
grammar rejects it. Exit status is nonzero when the profile and grammar disagree,
because that disagreement still needs to remain visible even though the report
is useful.

Validate one fixed performance query before running a measurement:

```bash
cargo run --manifest-path benchmark/syntax_check/Cargo.toml -- \
    --language gql --query "MATCH (n) WHERE ELEMENT_ID(n) = 1 RETURN n"
```

The crate declares its own `[workspace]`, so it does not change the dynamic
research workspace. Its `target/` and `Cargo.lock` are local ignored outputs.
