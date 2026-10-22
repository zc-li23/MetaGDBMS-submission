//! Check a Core Profile against the language's own grammar.
//!
//! A Core Profile states what a language *means*; this tool records whether
//! each statement is accepted by the grammar the project vendors under
//! data/language/. It uses the same complete top-level entry rule required of
//! generated candidates, so a valid prefix followed by garbage is rejected.
//!
//! Usage:
//!
//! ```bash
//! cargo run --manifest-path benchmark/syntax_check/Cargo.toml -- \
//!     --language gql benchmark/gql/core.jsonl \
//!     --rules-output target/gql-core-rule-coverage.tsv
//! ```
//!
//! Positive cases must parse; cases tagged `error` must not. Both directions
//! are reported, so the tool also catches a negative case that the grammar
//! happens to accept.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::ExitCode;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::BailErrorStrategy;
use antlr4rust::InputStream;
use antlr4rust::errors::ANTLRError;
use antlr4rust::tree::ParseTreeListener;

use metagdb_cypher_antlr::cypherlexer::CypherLexer;
use metagdb_cypher_antlr::cypherparser::CypherParser;
use metagdb_gql_antlr::gqllexer::GQLLexer;
use metagdb_gql_antlr::gqllistener::GQLListener;
use metagdb_gql_antlr::gqlparser::{GQLParser, GQLParserContextType, GQLTreeWalker, ruleNames};
use metagdb_gremlin_antlr::gremlinlexer::GremlinLexer;
use metagdb_gremlin_antlr::gremlinparser::GremlinParser;

fn parses(language: &str, query: &str) -> bool {
    match language {
        "cypher" => {
            let lexer = CypherLexer::new(InputStream::new(query));
            let mut parser = CypherParser::with_strategy(
                CommonTokenStream::new(lexer),
                Box::new(BailErrorStrategy::new()),
            );
            parser.oC_Cypher().is_ok()
        }
        "gql" => {
            let lexer = GQLLexer::new(InputStream::new(query));
            let mut parser = GQLParser::with_strategy(
                CommonTokenStream::new(lexer),
                Box::new(BailErrorStrategy::new()),
            );
            parser.gqlRequest().is_ok()
        }
        "gremlin" => {
            let lexer = GremlinLexer::new(InputStream::new(query));
            let mut parser = GremlinParser::with_strategy(
                CommonTokenStream::new(lexer),
                Box::new(BailErrorStrategy::new()),
            );
            parser.queryList().is_ok()
        }
        other => panic!("unsupported language {other:?}"),
    }
}

/// Minimal JSON string-field reader; the profile lines come from `json.dumps`,
/// so the layout is stable and no serde dependency is needed.
fn field(line: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\"");
    let found = line.find(&marker)? + marker.len();
    let rest = line[found..].trim_start();
    let rest = rest.strip_prefix(':').unwrap_or(rest).trim_start();
    let rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(out),
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    Some(out)
}

fn clean(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

#[derive(Default)]
struct RuleCollector {
    seen: BTreeSet<usize>,
}

impl<'input> ParseTreeListener<'input, GQLParserContextType> for RuleCollector {
    fn enter_every_rule(
        &mut self,
        ctx: &<GQLParserContextType as antlr4rust::parser::ParserNodeType<'input>>::Type,
    ) -> Result<(), ANTLRError> {
        self.seen.insert(ctx.get_rule_index());
        Ok(())
    }
}

impl<'input> GQLListener<'input> for RuleCollector {}

fn gql_rule_indices(query: &str) -> Result<BTreeSet<usize>, String> {
    let lexer = GQLLexer::new(InputStream::new(query));
    let mut parser = GQLParser::with_strategy(
        CommonTokenStream::new(lexer),
        Box::new(BailErrorStrategy::new()),
    );
    let tree = parser.gqlRequest().map_err(|error| format!("{error:?}"))?;
    let listener = GQLTreeWalker::walk(Box::new(RuleCollector::default()), &*tree)
        .map_err(|error| format!("{error:?}"))?;
    Ok(listener.seen)
}

#[cfg(test)]
mod tests {
    use super::parses;

    #[test]
    fn gql_request_rejects_trailing_garbage() {
        assert!(parses("gql", "MATCH (n) RETURN n"));
        assert!(!parses("gql", "MATCH (n) RETURN n BOGUSGARBAGE"));
    }

    #[test]
    fn each_performance_shape_is_grammar_valid() {
        assert!(parses("cypher", "MATCH (n) WHERE id(n) = 1 RETURN n"));
        assert!(parses("gql", "MATCH (n) WHERE ELEMENT_ID(n) = 1 RETURN n"));
        assert!(parses("gremlin", "g.V(1).out()"));
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut language = String::new();
    let mut path: Option<PathBuf> = None;
    let mut query: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut rules_output: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--language" => language = args.next().unwrap_or_default(),
            "--query" => query = args.next(),
            "--output" => output = args.next().map(PathBuf::from),
            "--rules-output" => rules_output = args.next().map(PathBuf::from),
            other if !other.starts_with("--") => path = Some(PathBuf::from(other)),
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
    }
    if !matches!(language.as_str(), "cypher" | "gql" | "gremlin") {
        eprintln!("usage: syntax-check --language <cypher|gql|gremlin> [--query <query> | <core.jsonl>] [--output <report.tsv>] [--rules-output <rules.tsv>]");
        return ExitCode::from(2);
    }
    if let Some(query) = query {
        if path.is_some() || output.is_some() || rules_output.is_some() {
            eprintln!("--query cannot be combined with a profile path or output paths");
            return ExitCode::from(2);
        }
        if parses(&language, &query) {
            println!("accepted");
            return ExitCode::SUCCESS;
        }
        println!("rejected");
        return ExitCode::from(1);
    }
    let Some(path) = path else {
        eprintln!("usage: syntax-check --language <cypher|gql|gremlin> <core.jsonl> [--output <report.tsv>] [--rules-output <rules.tsv>]");
        return ExitCode::from(2);
    };

    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("cannot read {}: {error}", path.display());
            return ExitCode::from(2);
        }
    };

    let mut positives = 0usize;
    let mut negatives = 0usize;
    let mut rejected = Vec::new();
    let mut wrongly_accepted = Vec::new();
    let mut per_category: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut rule_cases: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut report =
        String::from("id\tscored\tnegative\taccepted\tgrammar_applicable\tcategory\tquery\n");

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let id = field(line, "id").unwrap_or_else(|| "?".into());
        let category = field(line, "category").unwrap_or_else(|| "?".into());
        let query = field(line, "query").unwrap_or_default();
        let negative = line.contains("\"expect_error\"");
        let scored = !line.contains("\"scored\": false");
        let accepted = parses(&language, &query);
        if rules_output.is_some() && language == "gql" && accepted && !negative {
            match gql_rule_indices(&query) {
                Ok(indices) => {
                    for index in indices {
                        rule_cases.entry(index).or_default().push(id.clone());
                    }
                }
                Err(error) => {
                    eprintln!("cannot collect GQL rules for {id}: {error}");
                    return ExitCode::from(2);
                }
            }
        }
        let grammar_applicable = if negative { !accepted } else { accepted };
        report.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            clean(&id),
            scored,
            negative,
            accepted,
            grammar_applicable,
            clean(&category),
            clean(&query)
        ));
        let entry = per_category.entry(category).or_insert((0, 0));
        entry.1 += 1;
        if negative {
            negatives += 1;
            if accepted {
                wrongly_accepted.push(id);
            } else {
                entry.0 += 1;
            }
        } else {
            positives += 1;
            if accepted {
                entry.0 += 1;
            } else {
                rejected.push((id, query));
            }
        }
    }

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!("cannot create {}: {error}", parent.display());
                return ExitCode::from(2);
            }
        }
        if let Err(error) = fs::write(&output, report) {
            eprintln!("cannot write {}: {error}", output.display());
            return ExitCode::from(2);
        }
    }
    if let Some(output) = rules_output {
        if language != "gql" {
            eprintln!("--rules-output currently requires --language gql");
            return ExitCode::from(2);
        }
        if let Some(parent) = output.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!("cannot create {}: {error}", parent.display());
                return ExitCode::from(2);
            }
        }
        let mut rows = String::from("rule_index\trule_name\tcase_count\tcase_ids\n");
        for (index, name) in ruleNames.iter().enumerate() {
            let ids = rule_cases.get(&index);
            let count = ids.map_or(0, Vec::len);
            let joined = ids.map_or(String::new(), |items| items.join(","));
            rows.push_str(&format!("{index}\t{name}\t{count}\t{joined}\n"));
        }
        if let Err(error) = fs::write(&output, rows) {
            eprintln!("cannot write {}: {error}", output.display());
            return ExitCode::from(2);
        }
        println!("GQL parser rules observed: {}/{}", rule_cases.len(), ruleNames.len());
    }
    println!("{language}: {positives} positive cases, {negatives} negative cases");
    for (category, (ok, total)) in &per_category {
        println!("  {category:<30} {ok:>4}/{total:<4}");
    }
    if !rejected.is_empty() {
        println!("\nrejected by the grammar ({}):", rejected.len());
        for (id, query) in &rejected {
            println!("  {id}: {}", query.replace('\n', " "));
        }
    }
    if !wrongly_accepted.is_empty() {
        println!(
            "\nnegative cases the grammar accepts ({}):",
            wrongly_accepted.len()
        );
        for id in &wrongly_accepted {
            println!("  {id}");
        }
    }
    if rejected.is_empty() && wrongly_accepted.is_empty() {
        println!("\nevery case matches the grammar's accept/reject expectation");
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
