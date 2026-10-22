//! Storage layer: record encoding, the MVCC-backed `StorageEngine`, and the
//! graph-file import path.

pub mod codec;
pub mod engine;
pub mod import;

pub use codec::{FileSink, Reader, VecSink, Writer};
pub use engine::{MvccStorage, VERTEX_ID_INDEX};
pub use import::{import_graph_file, ImportSummary};
