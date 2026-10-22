/**
 * @brief: metaGDB query pipeline control flow module, defining the complete stage orchestration
 *         from query text to result output.
 */

pub mod executor;
pub mod frontend;
pub mod optimizer;
pub mod parser;
pub mod planner;

use std::collections::HashMap;
use std::sync::Arc;

use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::StorageEngine;
use metagdb_core::l3_txn::TxnManager;
use metagdb_core::l4_compute::algebra::PhysicalPlan;
use metagdb_core::l4_compute::engine::{Executor, Optimizer, Parser, Planner, ResultStream};
use metagdb_core::l4_compute::olap::{AlgorithmRegistry, AnalyticalAlgorithm};
use metagdb_core::l5_interaction::{Frontend, QueryLanguage};
use metagdb_core::{Error, Result};

/**
 * @brief: metaGDB runtime container, holding compute engine components and storage/transaction
 *         abstractions.
 *
 * Supports multiple languages: each language can register an independent Parser (via
 * with_parser(lang, p)). After the Frontend identifies the language, the Runtime automatically
 * routes to the corresponding Parser. Planner / Optimizer / Executor are language-agnostic
 * components shared by all languages.
 */
pub struct Runtime {
    /// Parser registry indexed by language
    parsers: HashMap<QueryLanguage, Box<dyn Parser>>,
    planner: Box<dyn Planner>,
    optimizer: Box<dyn Optimizer>,
    executor: Box<dyn Executor>,

    schema: Arc<dyn GraphSchema>,
    storage: Arc<dyn StorageEngine>,
    txn_manager: Arc<dyn TxnManager>,
    algorithms: Arc<AlgorithmRegistry>,

    frontends: HashMap<QueryLanguage, Arc<dyn Frontend>>,
    default_language: QueryLanguage,
}

impl Runtime {
    /**
     * @brief: Run a query text end-to-end using the default query language.
     * @param src: Query source text.
     * @return: Returns a ResultStream on success, or Error otherwise.
     */
    pub fn execute_query(&self, src: &str) -> Result<ResultStream> {
        self.execute_query_with(self.default_language.clone(), src)
    }

    /**
     * @brief: Run a query end-to-end with an explicitly specified query language.
     *
     * Control flow: Frontend → language identification → Parser (language-specific) → Planner
     * (language-agnostic) → Optimizer → Executor
     *
     * @param language: Specified query language type.
     * @param src: Query source text.
     * @return: Returns a ResultStream on success, or Error otherwise.
     */
    pub fn execute_query_with(
        &self,
        language: QueryLanguage,
        src: &str,
    ) -> Result<ResultStream> {
        let fe = self.frontend_for(&language)?;
        let req = frontend::run(fe, src);
        debug_assert_eq!(req.language, language, "Frontend must honestly declare the language");

        // Route to the Parser corresponding to the language
        let parser = self.parser_for(&req.language)?;
        let ast = parser::run(parser, &req)?;

        // Planner / Optimizer / Executor are language-agnostic components
        let lp = planner::run(&*self.planner, ast, &*self.schema)?;
        let pp = optimizer::run(
            &*self.optimizer,
            lp,
            self.schema.clone(),
            self.algorithms.clone(),
        )?;
        executor::run(
            &*self.executor,
            &pp,
            &*self.txn_manager,
            self.storage.clone(),
            self.schema.clone(),
            self.algorithms.clone(),
        )
    }

    /**
     * @brief: Get the physical execution plan for a query (EXPLAIN mode), without actually
     *         executing.
     * @param src: Query source text.
     * @return: Returns a PhysicalPlan on success, or Error otherwise.
     */
    pub fn explain(&self, src: &str) -> Result<PhysicalPlan> {
        self.explain_with(self.default_language.clone(), src)
    }

    /**
     * @brief: Get the physical execution plan for a query with an explicitly specified language.
     * @param language: Specified query language type.
     * @param src: Query source text.
     * @return: Returns a PhysicalPlan on success, or Error otherwise.
     */
    pub fn explain_with(
        &self,
        language: QueryLanguage,
        src: &str,
    ) -> Result<PhysicalPlan> {
        let fe = self.frontend_for(&language)?;
        let req = frontend::run(fe, src);
        let parser = self.parser_for(&req.language)?;
        let ast = parser::run(parser, &req)?;
        let lp = planner::run(&*self.planner, ast, &*self.schema)?;
        optimizer::run(
            &*self.optimizer,
            lp,
            self.schema.clone(),
            self.algorithms.clone(),
        )
    }

    /**
     * @brief: Get the graph metadata Schema of the current runtime.
     * @return: Returns a GraphSchema reference.
     */
    pub fn schema(&self) -> &dyn GraphSchema {
        &*self.schema
    }

    /**
     * @brief: Get the default query language of the current runtime.
     * @return: Returns a default language reference.
     */
    pub fn default_language(&self) -> &QueryLanguage {
        &self.default_language
    }

    /**
     * @brief: Get an iterator over all query languages supported by the current runtime.
     * @return: Returns a QueryLanguage iterator.
     */
    pub fn supported_languages(&self) -> impl Iterator<Item = &QueryLanguage> {
        self.frontends.keys()
    }

    /**
     * @brief: Get the corresponding frontend handler for a language.
     * @param language: The query language.
     * @return: Returns a Frontend reference on success, or Error otherwise.
     */
    fn frontend_for(&self, language: &QueryLanguage) -> Result<&dyn Frontend> {
        self.frontends.get(language).map(|fe| &**fe).ok_or_else(|| {
            Error::Other(format!(
                "Runtime: query language {:?} not registered (registered: {:?})",
                language,
                self.frontends.keys().collect::<Vec<_>>()
            ))
        })
    }

    /**
     * @brief: Get the corresponding Parser for a language.
     */
    fn parser_for(&self, language: &QueryLanguage) -> Result<&dyn Parser> {
        self.parsers.get(language).map(|p| &**p).ok_or_else(|| {
            Error::Other(format!(
                "Runtime: Parser for query language {:?} not registered (registered: {:?})",
                language,
                self.parsers.keys().collect::<Vec<_>>()
            ))
        })
    }
}

/**
 * @brief: Runtime builder, responsible for collecting component implementations from all layers
 *         and finally assembling a Runtime instance.
 */
pub struct RuntimeBuilder {
    parsers: HashMap<QueryLanguage, Box<dyn Parser>>,
    planner: Option<Box<dyn Planner>>,
    optimizer: Option<Box<dyn Optimizer>>,
    executor: Option<Box<dyn Executor>>,

    schema: Option<Arc<dyn GraphSchema>>,
    storage: Option<Arc<dyn StorageEngine>>,
    txn_manager: Option<Arc<dyn TxnManager>>,
    algorithms: AlgorithmRegistry,

    frontends: HashMap<QueryLanguage, Arc<dyn Frontend>>,
    frontend_order: Vec<QueryLanguage>,
    default_language: Option<QueryLanguage>,
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBuilder {
    /**
     * @brief: Create a new RuntimeBuilder instance.
     */
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
            planner: None,
            optimizer: None,
            executor: None,
            schema: None,
            storage: None,
            txn_manager: None,
            algorithms: AlgorithmRegistry::default(),
            frontends: HashMap::new(),
            frontend_order: Vec::new(),
            default_language: None,
        }
    }

    /**
     * @brief: Set the graph metadata Schema implementation.
     */
    pub fn with_schema(mut self, s: Arc<dyn GraphSchema>) -> Self {
        self.schema = Some(s);
        self
    }

    /**
     * @brief: Set the graph storage engine implementation.
     */
    pub fn with_storage(mut self, s: Arc<dyn StorageEngine>) -> Self {
        self.storage = Some(s);
        self
    }

    /**
     * @brief: Set the transaction manager implementation.
     */
    pub fn with_txn_manager(mut self, m: Arc<dyn TxnManager>) -> Self {
        self.txn_manager = Some(m);
        self
    }

    /**
     * @brief: Register a Parser implementation for the specified language.
     *
     * @param language: The query language this Parser handles.
     * @param p: Parser implementation.
     */
    pub fn with_parser(mut self, language: QueryLanguage, p: Box<dyn Parser>) -> Self {
        self.parsers.insert(language, p);
        self
    }

    /**
     * @brief: Set the query planner implementation (language-agnostic).
     */
    pub fn with_planner(mut self, p: Box<dyn Planner>) -> Self {
        self.planner = Some(p);
        self
    }

    /**
     * @brief: Set the query optimizer implementation (language-agnostic).
     */
    pub fn with_optimizer(mut self, o: Box<dyn Optimizer>) -> Self {
        self.optimizer = Some(o);
        self
    }

    /**
     * @brief: Set the query executor implementation (language-agnostic).
     */
    pub fn with_executor(mut self, e: Box<dyn Executor>) -> Self {
        self.executor = Some(e);
        self
    }

    /**
     * @brief: Register an OLAP algorithm.
     */
    pub fn register_algorithm(mut self, algo: Arc<dyn AnalyticalAlgorithm>) -> Self {
        self.algorithms
            .algorithms
            .insert(algo.name().to_string(), algo);
        self
    }

    /**
     * @brief: Register a query language frontend implementation.
     * @param fe: Frontend instance.
     */
    pub fn with_frontend(mut self, fe: Arc<dyn Frontend>) -> Self {
        let lang = fe.language();
        if !self.frontends.contains_key(&lang) {
            self.frontend_order.push(lang.clone());
        }
        self.frontends.insert(lang, fe);
        self
    }

    /**
     * @brief: Explicitly specify the default query language for the runtime.
     */
    pub fn with_default_language(mut self, language: QueryLanguage) -> Self {
        self.default_language = Some(language);
        self
    }

    /**
     * @brief: Validate the configuration and complete the final assembly of the Runtime.
     * @return: Returns a Runtime instance on success, or Error if required components are missing.
     */
    pub fn build(self) -> Result<Runtime> {
        let schema = self
            .schema
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing schema".into()))?;
        let storage = self
            .storage
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing storage".into()))?;
        let txn_manager = self
            .txn_manager
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing txn_manager".into()))?;
        if self.parsers.is_empty() {
            return Err(Error::Other(
                "RuntimeBuilder: at least one Parser must be registered (with_parser)".into(),
            ));
        }
        let planner = self
            .planner
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing planner".into()))?;
        let optimizer = self
            .optimizer
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing optimizer".into()))?;
        let executor = self
            .executor
            .ok_or_else(|| Error::Other("RuntimeBuilder: missing executor".into()))?;

        if self.frontends.is_empty() {
            return Err(Error::Other(
                "RuntimeBuilder: at least one Frontend must be registered (with_frontend)".into(),
            ));
        }

        let default_language = match self.default_language {
            Some(ref lang) => {
                if !self.frontends.contains_key(lang) {
                    return Err(Error::Other(format!(
                        "RuntimeBuilder: default language {:?} not registered via with_frontend",
                        lang
                    )));
                }
                if !self.parsers.contains_key(lang) {
                    return Err(Error::Other(format!(
                        "RuntimeBuilder: default language {:?} not registered via with_parser",
                        lang
                    )));
                }
                lang.clone()
            }
            None => self
                .frontend_order
                .first()
                .cloned()
                .expect("frontend_order must be non-empty when frontends is non-empty"),
        };

        Ok(Runtime {
            parsers: self.parsers,
            planner,
            optimizer,
            executor,
            schema,
            storage,
            txn_manager,
            algorithms: Arc::new(self.algorithms),
            frontends: self.frontends,
            default_language,
        })
    }
}
