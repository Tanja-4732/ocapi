//! Data returned by the API endpoints of the `/api/events` path
//!
//! The following endpoints are not available (POST, PUT, DELETE):
//!
//! - `/api/events/`
//! - `/api/events/{eventId}`
//! - `/api/events/{eventId}/acl/{action}/{role}`
//! - `/api/events/{eventId}/metadata`
//! - `/api/events/{eventId}/acl`
//! - `/api/events/{eventId}/metadata`
//! - `/api/events/{eventId}`
//! - `/api/events/{eventId}/scheduling`

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Data returned by endpoint `/api/events`; this is one of thee most important types in the library
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub identifier: String,
    pub creator: String,
    pub presenter: Vec<String>,
    pub created: String,
    pub is_part_of: String,
    pub subjects: Vec<String>,
    pub start: String,
    pub description: String,
    pub language: String,
    pub source: String,
    pub title: String,
    pub processing_state: String,
    pub duration: Option<u64>,
    pub license: String,
    pub archive_version: u64,
    pub contributor: Vec<String>,
    pub series: String,
    pub has_previews: bool,
    pub location: String,
    pub rightsholder: String,
    pub publication_status: Vec<String>,
    pub status: String,
    pub publications: Option<Vec<Publication>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Publication {
    pub metadata: Vec<Value>,
    pub attachments: Vec<Value>,
    pub channel: String,
    pub id: String,
    pub media: Vec<Value>,
    #[serde(rename = "mediatype")]
    pub media_type: String,
    pub url: String,
}

/// Uninteresting data, as all the URLs yield 403 Forbidden
mod todo_media {
    use super::*;

    /// Data returned by endpoint `/api/events/{eventId}/media`
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root2 {
        pub identifier: String,
        #[serde(rename = "has_audio")]
        pub has_audio: bool,
        pub streams: Streams,
        pub description: String,
        #[serde(rename = "is_master_playlist")]
        pub is_master_playlist: bool,
        pub uri: String,
        #[serde(rename = "has_video")]
        pub has_video: bool,
        pub tags: Vec<String>,
        pub duration: i64,
        pub flavor: String,
        pub size: i64,
        pub checksum: String,
        #[serde(rename = "is_live")]
        pub is_live: bool,
        pub mimetype: String,
        #[serde(rename = "element-description")]
        pub element_description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Streams {
        #[serde(rename = "video-1")]
        pub video_1: Video1,
        #[serde(rename = "audio-1")]
        pub audio_1: Audio1,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Video1 {
        pub identifier: String,
        pub framecount: i64,
        pub framewidth: i64,
        pub framerate: i64,
        pub format: String,
        pub bitrate: i64,
        pub frameheight: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Audio1 {
        pub identifier: String,
        pub channels: i64,
        pub framecount: i64,
        pub format: String,
        pub bitrate: i64,
        pub samplingrate: i64,
    }
}

/// Dublin Core metadata
mod todo_metadata {
    use super::*;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root2 {
        pub flavor: String,
        pub title: String,
        pub fields: Vec<Field>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Field {
        pub read_only: bool,
        pub id: String,
        pub label: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub value: Value,
        pub required: bool,
        pub translatable: Option<bool>,
    }
}
