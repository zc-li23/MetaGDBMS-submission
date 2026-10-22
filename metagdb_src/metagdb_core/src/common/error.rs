/**
 * @brief: Framework unified error type definition, categorizing errors according to
 *         the five-layer model abstraction.
 */

use std::fmt;

/**
 * @brief: Framework unified error enumeration.
 */
#[derive(Debug, Clone)]
pub enum Error {
    InvalidQuery(String),
    Parse(String),
    Plan(String),
    Optimize(String),
    Execute(String),
    Storage(String),
    Transaction(String),
    Schema(String),
    NotFound(String),
    Unsupported(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidQuery(s) => write!(f, "invalid query: {s}"),
            Error::Parse(s) => write!(f, "parse error: {s}"),
            Error::Plan(s) => write!(f, "plan error: {s}"),
            Error::Optimize(s) => write!(f, "optimize error: {s}"),
            Error::Execute(s) => write!(f, "execute error: {s}"),
            Error::Storage(s) => write!(f, "storage error: {s}"),
            Error::Transaction(s) => write!(f, "transaction error: {s}"),
            Error::Schema(s) => write!(f, "schema error: {s}"),
            Error::NotFound(s) => write!(f, "not found: {s}"),
            Error::Unsupported(s) => write!(f, "unsupported: {s}"),
            Error::Other(s) => write!(f, "error: {s}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
