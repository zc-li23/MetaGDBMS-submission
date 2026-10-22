# ANTLR and lowering failures

The `.g4` source is already compiled. Use its generated crate as the syntax
frontend; do not recreate tokenization or parsing in candidate code.

## Required pipeline

```text
InputStream → generated Lexer → CommonTokenStream → generated Parser
            → top-level rule → CST lowering → private AST
```

Use `BailErrorStrategy`. Generated accessors commonly return
`Option<Rc<Context>>`; pass `value.as_ref()` to lowering helpers. Import Context
attribute traits from the generated parser module. Inspect the generated summary,
then source, before naming a context or accessor.

| Language | Lexer | Parser | Entry rule |
|---|---|---|---|
| Cypher | `CypherLexer` | `CypherParser` | `oC_Cypher()` |
| GQL | `GQLLexer` | `GQLParser` | `gqlRequest()` |
| Gremlin | `GremlinLexer` | `GremlinParser` | `queryList()` |

For GQL, traverse every branch from `gqlRequest` through program/transaction
activity and procedure body. Its grammar may route a pure query through the
linear-data-modifying alternative; inspect contained statements instead of
assuming that branch always contains DML. `primitiveResultStatement` owns RETURN
and ORDER/OFFSET/LIMIT as sibling contexts.

For Gremlin, labeled alternatives produce `ContextAll` enums. Match their actual
variants, including the error variant. `chainedTraversal` is left recursive:
lower the preceding `chainedTraversal()` first, then append the current
`traversalMethod()` or the step order is reversed.

## Historical failure: benchmark-only parser

A generated GQL candidate once removed `metagdb_gql_antlr` and `antlr4rust`, then
implemented a handwritten lexer/parser for the visible 200-query profile. It
scored 200/200 but rejected grammar-valid parameters, shortest paths, quantified
paths, label expressions, temporal literals, set operations, and procedure calls.
This proves benchmark-subset behavior, not `.g4 + d → code` synthesis.

When this symptom appears, retain the backend and replace only the frontend with
the generated ANTLR pipeline and CST lowering. Add grammar-valid probes outside
the visible benchmark. A parsed alternative without implemented semantics must
produce a precise unsupported error in lowering/planning and remain recorded in
the capability matrix.
