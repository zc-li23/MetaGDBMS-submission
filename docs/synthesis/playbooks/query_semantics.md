# Query semantic failures

Group the report by semantic root cause, such as binding, cardinality, traversal
state, value representation, ordering, null behavior, aggregation, or path labels.
Trace one minimal public-runtime case from parse tree through AST, plan, operators,
and result encoding. Fix the shared semantic layer and add a regression test; do
not patch query strings. Check empty, singleton, duplicate, and mixed-value cases.
Then rerun the whole category and the full benchmark.
