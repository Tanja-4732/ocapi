//! Data returned by the API endpoints of the `/api` path
//!
//! **Remember**: Other modules in [`crate::core`] also contain data types for the API responses.  
//! See the documentation of [`crate::core`] for more information.
//!
//! The following endpoints are not available:
//!
//! - `/api/info/organization`
//! - `/api/info/organization/properties`
//! - `/api/info/organization/properties/engageuiurl`

pub mod events;

use serde::{Deserialize, Serialize};

/// Data returned by endpoint `/api`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Api {
    pub version: String,
    pub url: String,
}

/// Data returned by endpoint `/api/info/me`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyInfo {
    pub provider: String,
    pub name: String,
    #[serde(rename = "userrole")]
    pub user_role: String,
    pub email: String,
    pub username: String,
}

/// Data returned by endpoint `/api/info/me/roles`
pub type MyRoles = Vec<String>;

/// Data returned by endpoint `/api/version`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Versions {
    pub default: String,
    pub versions: Vec<String>,
}

/// Data returned by endpoint `/api/version/default`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct DefaultVersion {
    pub default: String,
}
