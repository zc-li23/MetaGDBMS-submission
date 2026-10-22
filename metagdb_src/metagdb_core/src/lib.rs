/**
 * @brief: metaGDB core contract layer, defining the five-layer model standard
 *         interfaces (traits) and the data model (IR) for cross-layer communication.
 */

pub mod benchmark;
pub mod common;
pub mod l1_model;
pub mod l2_storage;
pub mod l3_txn;
pub mod l4_compute;
pub mod l5_interaction;

pub use common::error::{Error, Result};
pub use common::ids::{
    EdgeId, EdgeTypeId, LabelId, LocalEid, LocalVid, PropertyKeyId, VertexId,
};
pub use common::property::{PropertyElement, PropertyMap, PropertyValue, ResultPath, ResultValue};
