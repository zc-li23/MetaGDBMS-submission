//! Query compute layer: language AST, CST lowering, parser, planner, optimizer,
//! push-based executor, CSR projection, and the required OLAP algorithms.

pub mod algorithms;
pub mod ast;
pub mod executor;
pub mod functions;
pub mod ir;
pub mod lowering;
pub mod optimizer;
pub mod parser;
pub mod physical;
pub mod plan;
pub mod planner;
pub mod projection;
