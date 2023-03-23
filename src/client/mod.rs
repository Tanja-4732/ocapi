//! Functions for invoking the API endpoints, returning the data types defined in the [`crate::core`] module.

pub mod api;
pub mod info;
pub mod series;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A struct for invoking the API endpoints, holds the base URL
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OcApi {
    // TODO consider including the reqwest::Client in OcApi for cookies
    base_url: String,
}

impl OcApi {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_ref()
    }
}
