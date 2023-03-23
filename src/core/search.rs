//! Data returned by the API endpoints of the `/search` path
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodesData {
    #[serde(rename = "search-results")]
    pub search_results: SearchResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
    pub search_time: u64,
    pub query: String,
    // #[serde(deserialize_with = "one_or_more_result")]
    pub result: Option<Vec<Result>>,
}

fn one_or_more_result<'de, D>(deserializer: D) -> std::result::Result<Vec<Result>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    // TODO care about the following warning: large size difference between variants
    enum OneOrMore {
        One(Result),
        More(Vec<Result>),
    }

    // Attempts de-serialization
    // If we got one object instead of a vector, wrap it in a vector
    let data = match OneOrMore::deserialize(deserializer)? {
        OneOrMore::One(the_one) => vec![the_one],
        OneOrMore::More(the_more) => the_more,
    };

    Ok(data)
}

fn one_or_more_string<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMore {
        One(String),
        More(Vec<String>),
    }

    Ok(match OneOrMore::deserialize(deserializer)? {
        OneOrMore::One(the_one) => vec![the_one],
        OneOrMore::More(the_more) => the_more,
    })
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: String,
    pub org: String,
    pub mediapackage: Mediapackage,
    pub dc_extent: u64,
    pub dc_title: String,
    pub dc_creator: Option<String>,
    pub dc_publisher: Option<String>,
    // pub dc_created: Option<String>, // FIXME
    pub dc_created: Option<Value>,
    pub dc_spatial: Option<String>,
    pub dc_is_part_of: Option<String>,
    pub oc_mediapackage: String,
    pub media_type: String,
    pub keywords: KeywordsOrString,
    pub modified: String,
    pub score: f64,
    // pub segments: Option<Segments>, // FIXME
    pub segments: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mediapackage {
    pub duration: u64,
    pub id: String,
    pub start: String,
    pub title: String,
    pub series: Option<String>,
    #[serde(rename = "seriestitle")]
    pub series_title: Option<String>,
    pub media: Media,
    pub metadata: Metadata,
    pub attachments: Attachments,
    pub publications: String,
    // I got a response with [0, "text"] once instead of just "text"
    // TODO handle an array with both strings and numbers for some reason
    pub creators: Option<CreatorsOrString>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub track: Vec<Track>,
}

/// A candidate for downloading
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    #[serde(rename = "type")]
    // TODO figure out if/how the enum should be used
    // pub type_field: TrackType,
    pub type_field: String,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    pub mimetype: String,
    /// Contains info on the videos quality
    pub tags: TagsOrString,
    pub url: String,
    pub checksum: Option<Checksum>,
    pub duration: u64,
    // TODO uncomment the lines below
    pub audio: Option<Audio>,
    pub video: Option<Video>,
    pub live: bool,
    pub transport: Option<String>,
    pub size: Option<u64>,
    pub master: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum TrackType {
    #[serde(rename = "presenter/delivery")]
    Presenter,
    #[serde(rename = "presenter_video/delivery")]
    PresenterNoAudio,
    #[serde(rename = "presentation/delivery")]
    Presentation,
    #[serde(rename = "raw/delivery")]
    Raw,
}

impl Default for TrackType {
    fn default() -> Self {
        TrackType::Presentation
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(deserialize_with = "one_or_more_string")]
    pub tag: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    pub device: String,
    pub encoder: Encoder,
    pub framecount: Option<u64>,
    pub channels: u64,
    pub samplingrate: Option<u64>,
    pub bitrate: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoder {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub device: String,
    pub encoder: Encoder,
    pub framecount: Option<u64>,
    pub bitrate: f64,
    pub framerate: f64,
    pub resolution: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(deserialize_with = "one_or_more_catalog")]
    pub catalog: Vec<Catalog>,
}

fn one_or_more_catalog<'de, D>(deserializer: D) -> std::result::Result<Vec<Catalog>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    // TODO care about the following warning: large size difference between variants
    enum OneOrMore {
        One(Catalog),
        More(Vec<Catalog>),
    }

    // Attempts de-serialization
    // If we got one object instead of a vector, wrap it in a vector
    let data = match OneOrMore::deserialize(deserializer)? {
        OneOrMore::One(the_one) => vec![the_one],
        OneOrMore::More(the_more) => the_more,
    };

    Ok(data)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mimetype: String,
    pub tags: TagsOrString,
    pub url: String,
    pub checksum: Option<Checksum>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
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
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    pub mimetype: String,
    pub tags: TagsOrString,
    pub url: String,
    pub size: Option<u64>,
    pub additional_properties: Option<AdditionalProperties>,
    pub checksum: Option<Checksum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperties {
    pub property: Vec<Property>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creators {
    #[serde(deserialize_with = "one_or_more_string")]
    pub creator: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keywords {
    #[serde(deserialize_with = "one_or_more_string")]
    pub keywords: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segments {
    pub segment: Vec<Segment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub index: u64,
    pub time: u64,
    pub duration: u64,
    pub relevance: i64,
    pub hit: bool,
    #[serde(deserialize_with = "string_or_int")]
    pub text: String,
    pub previews: PreviewsOrString,
}

fn string_or_int<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Int(u64),
        String(String),
    }

    let data = match StringOrInt::deserialize(deserializer)? {
        StringOrInt::Int(the_int) => the_int.to_string(),
        StringOrInt::String(the_string) => the_string,
    };

    Ok(data)
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub key: String,
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "$")]
    pub field: String,
}

macro_rules! something_or_string {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum $name {
            One(String),
            More($type),
        }

        impl Default for $name {
            fn default() -> Self {
                $name::One(String::default())
            }
        }
    };
}

something_or_string!(TagsOrString, Tags);
something_or_string!(CreatorsOrString, Creators);
something_or_string!(KeywordsOrString, Keywords);
something_or_string!(PreviewsOrString, Previews);
