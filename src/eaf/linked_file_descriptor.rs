//! Linked file descriptor.
//!
//! Specifies a media file to annotate or an external file, such as a time series CSV-file.
//! Part of the header.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{EafError, MimeType, support::url_from_path};

/// Linked file descriptor.
/// Specifies a media file to annotate or an external file, such as a time series CSV-file.
/// Part of the header.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LINKED_FILE_DESCRIPTOR")]
pub struct LinkedFileDescriptor {
    #[serde(rename="@LINK_URL")]
    pub(crate) link_url: String,
    #[serde(rename="@RELATIVE_LINK_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relative_link_url: Option<String>,
    #[serde(rename="@MIME_TYPE")]
    pub mime_type: String,
    #[serde(rename="@TIME_ORIGIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_origin: Option<String>,
    #[serde(rename="@ASSOCIATED_WITH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_with: Option<String>,
}

impl LinkedFileDescriptor {
    pub fn new(path: &Path) -> Result<Self, EafError> {
        let mut descriptor = Self::default();
        let url = url_from_path(path)?;
        descriptor.link_url = url.to_string();
        descriptor.mime_type = MimeType::from_path(path).to_string();
        if let Some(filename) = path.file_name() {
            descriptor.relative_link_url = Some(format!("./{}",
                filename.to_string_lossy().to_string()
            ));
        }

        Ok(descriptor)
    }

    /// Returns file URL verbatim (absolute file path).
    pub fn file_url(&self) -> &str {
        &self.link_url.as_str()
    }

    /// Returns relative file URL verbatim (relative file path).
    pub fn relative_file_url(&self) -> Option<&str> {
        self.relative_link_url.as_deref()
    }

    /// Returns absolute media path.
    pub fn abs_path(&self) -> Option<PathBuf> {
        let url = Url::parse(&self.link_url).ok()?;
        Some(PathBuf::from(url.path()))
    }

    /// Returns `true` if the absolute media path is valid.
    pub fn abs_exists(&self) -> bool {
        self.abs_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// Returns relative media path.
    pub fn rel_path(&self) -> Option<PathBuf> {
        self.relative_link_url
            .as_ref()
            .map(|s| PathBuf::from(s))
    }

    /// Returns `true` if the relative media path is valid.
    pub fn rel_exists(&self) -> bool {
        self.rel_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }
}
