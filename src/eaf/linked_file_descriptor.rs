use serde::{Deserialize, Serialize};

/// EAF lnked file descriptor. Part of EAF header.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "LINKED_FILE_DESCRIPTOR")]
pub struct LinkedFileDescriptor {
    #[serde(rename="@LINK_URL")]
    link_url: String,
    #[serde(rename="@RELATIVE_LINK_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_link_url: Option<String>,
    #[serde(rename="@MIME_TYPE")]
    mime_type: String,
    #[serde(rename="@TIME_ORIGIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    time_origin: Option<String>,
    #[serde(rename="@ASSOCIATED_WITH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_with: Option<String>,
}