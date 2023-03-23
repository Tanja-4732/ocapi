//! Data returned by the API endpoints of the `/series` path
//!
//! The following endpoints are not available (POST, PUT, DELETE):
//!
//! - `/series/allInRangeAdministrative.json`
//! - `/series/{seriesID:.+}`
//! - `/series/{seriesId}/elements/{elementType}`
//! - `/series/{seriesId}/property/{propertyName}`
//! - `/series/{seriesID:.+}/accesscontrol`
//! - `/series` (the POST call; GET is available)
//! - `/series/{seriesId}/elements/{elementType}`
//! - `/series/{seriesId}/property`
//!
//! Not yet implemented (low priority or XML (low priority)):
//!
//! - `/series/{seriesID:.+}/acl.xml`
//! - `/series/{seriesID:.+}.xml`
//! - `/series/{seriesID:.+}.json`
//! - `/series/{seriesId}/elements.json`
//! - `/series/{seriesId}/elements/{elementType}`
//! - `/series/{seriesId}/property/{propertyName}.json` (related to the one above)

use serde::{Deserialize, Serialize};

/// Data returned by endpoint `/series`, usually comes in a [`Vec`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub identifier: String,
    pub license: String,
    pub creator: String,
    pub created: String,
    pub subjects: Vec<String>,
    pub organizers: Vec<String>,
    pub description: String,
    pub publishers: Vec<String>,
    pub language: String,
    pub contributors: Vec<String>,
    pub title: String,
    #[serde(rename = "rightsholder")]
    pub rights_holder: String,
}

/// Data returned by endpoint `/series/count`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Count(u64);

/// Data returned by endpoint `/series/{seriesID:.+}/acl.json`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessControlList {
    pub acl: Acl,
}

/// Used in [`AccessControlList`]
// TODO Merge with `AccessControlList` above
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    pub ace: Vec<Ace>,
}

/// Used in [`Acl`], which is used in [`AccessControlList`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ace {
    pub action: String,
    pub allow: bool,
    pub role: String,
}

/// Data returned by endpoint `/series/{seriesID:.+}.json`
// TODO implement SeriesJson; see https://docs.opencast.org/develop/developer/modules/api-docs/src/main/resources/static/paths/series.json
struct DublinCore;

/// Data returned by endpoint `/series/{id}/properties.json`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Likely contains useful data for live streams
    // TODO check if this is correct, and implement
    #[serde(rename = "slave")]
    pub follow: String,

    /// Either "1" or missing entirely
    pub chat: Option<String>,

    /// Likely contains useful data for live streams
    // TODO check if this is correct, and implement
    #[serde(rename = "master")]
    pub lead: String,
}
