use serde::Deserialize;

/// Metadata for a NASA "Astronomy Picture Of the Day"
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct APODMetadata {
    pub title: String,
    pub explanation: String,
    pub copyright: Option<String>,
    pub url: String,
    #[serde(rename = "hdurl")]
    pub hd_url: Option<String>,
    pub media_type: String,
}
