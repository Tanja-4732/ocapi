//! Data returned by the API endpoints of the `/info` path

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Data returned by endpoint `/info/health`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub release_id: String,
    pub description: String,
    pub service_id: String,
    pub version: String,
    pub status: String,
}

/// Data returned by endpoint `/info/components.json`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Components {
    pub rest: Vec<RestComponent>,
    pub ui: Vec<Value>, // TODO consider using a concrete type
    pub engage: String,
    pub admin: String,
}

/// Used in [`Components`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestComponent {
    pub path: String,
    pub docs: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_id: String,
    pub version: String,
}
