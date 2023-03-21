//! Data returned by the API endpoints of the `/search` path

mod todo {
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "search-results")]
    pub search_results: SearchResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
    pub search_time: i64,
    pub query: String,
    pub result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: String,
    pub org: String,
    pub mediapackage: Mediapackage,
    pub acl: Acl,
    pub dc_extent: i64,
    pub dc_title: String,
    pub dc_creator: Option<String>,
    pub dc_publisher: Option<String>,
    pub dc_created: String,
    pub dc_spatial: Option<String>,
    pub dc_is_part_of: String,
    pub oc_mediapackage: String,
    pub media_type: String,
    pub keywords: Value,
    pub modified: String,
    pub score: f64,
    pub segments: Option<Segments>,
    pub dc_description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mediapackage {
    pub duration: i64,
    pub id: String,
    pub start: String,
    pub title: String,
    pub series: String,
    pub seriestitle: String,
    pub creators: Option<Creators>,
    pub media: Media,
    pub metadata: Metadata,
    pub attachments: Attachments,
    pub publications: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creators {
    pub creator: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub track: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub mimetype: String,
    pub tags: Tags,
    pub url: String,
    pub size: i64,
    pub checksum: Option<Checksum>,
    pub duration: i64,
    pub video: Option<Video>,
    pub live: bool,
    pub master: bool,
    pub audio: Option<Audio>,
    pub transport: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub tag: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub device: String,
    pub encoder: Encoder,
    pub framecount: i64,
    pub bitrate: f64,
    pub framerate: i64,
    pub resolution: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoder {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    pub device: String,
    pub encoder: Encoder2,
    pub framecount: i64,
    pub channels: i64,
    pub samplingrate: i64,
    pub bitrate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoder2 {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub catalog: Vec<Catalog>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mimetype: String,
    pub tags: Tags2,
    pub url: String,
    pub checksum: Option<Checksum2>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags2 {
    pub tag: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachments {
    pub attachment: Vec<Attachment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mimetype: String,
    pub tags: Tags3,
    pub url: String,
    pub checksum: Option<Checksum3>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    pub size: Option<i64>,
    pub additional_properties: Option<AdditionalProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags3 {
    pub tag: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperties {
    pub property: Vec<Property>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub key: String,
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acl {
    pub ace: Vec<Ace>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ace {
    pub action: String,
    pub allow: bool,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segments {
    pub segment: Vec<Segment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub index: i64,
    pub time: i64,
    pub duration: i64,
    pub relevance: i64,
    pub hit: bool,
    pub text: String,
    pub previews: Previews,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Previews {
    pub preview: Preview,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    #[serde(rename = "ref")]
    pub ref_field: String,
    #[serde(rename = "$")]
    pub field: String,
}
}