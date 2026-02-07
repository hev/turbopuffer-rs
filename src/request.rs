use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

/// Typed request for upserting vectors into a namespace.
#[derive(Debug, Clone, Serialize)]
pub struct UpsertRequest {
    pub ids: Vec<Value>,
    pub vectors: Vec<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<Value>>>,
}

/// Typed request for querying vectors in a namespace.
#[derive(Debug, Clone, Serialize)]
pub struct QueryRequest {
    pub vector: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_metric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_vectors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
}

/// Typed request for deleting specific vectors by ID.
///
/// Turbopuffer handles per-vector deletes via the write endpoint
/// by sending a request body with a `deletes` array of IDs.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteVectorsRequest {
    pub deletes: Vec<Value>,
}
