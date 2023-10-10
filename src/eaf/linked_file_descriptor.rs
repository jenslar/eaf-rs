//! Linked file descriptor.
//! 
//! Specifies a media file to annotate or an external file, such as a time series CSV-file.
//! Part of the header.

use serde::{Deserialize, Serialize};

/// Linked file descriptor.
/// Specifies a media file to annotate or an external file, such as a time series CSV-file.
/// Part of the header.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "LINKED_FILE_DESCRIPTOR")]
pub struct LinkedFileDescriptor {
    #[serde(rename="@LINK_URL")]
    pub link_url: String,
    #[serde(rename="@RELATIVE_LINK_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_link_url: Option<String>,
    #[serde(rename="@MIME_TYPE")]
    pub mime_type: String,
    #[serde(rename="@TIME_ORIGIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_origin: Option<String>,
    #[serde(rename="@ASSOCIATED_WITH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_with: Option<String>,
}