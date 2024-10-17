//! Media descriptor.
//!
//! Specifies a linked media file, including path, and mime type,
//! but also an optional time offset (`time_origin`),
//! used by ELAN as a starting point to be able to synchronise media files.
//!
//! Part of the header.

use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};
use url::Url;

use crate::{
    support::url_from_path, EafError
};

/// Media descriptor.
///
/// Specifies a linked media file, including path, and mime type,
/// but also an optional time offset (`time_origin`),
/// used by ELAN as a starting point to be able to synchronise media files.
///
/// Part of the header.
///
/// Paths (media url, relative media url) are specified as `anyURI` in EAF XML schema,
/// and are stored as strings for easier conversion between URL schema (`file://`) and Path/PathBuf.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MEDIA_DESCRIPTOR")]
pub struct MediaDescriptor {
    /// Absolute media file path.
    #[serde(rename = "@MEDIA_URL")]
    pub(crate) media_url: String,
    /// Relative media file path.
    /// URI file scheme `file://...` can not be used as it only accepts absolute paths.
    #[serde(rename = "@RELATIVE_MEDIA_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relative_media_url: Option<String>,
    /// Media mime type.
    #[serde(rename = "@MIME_TYPE")]
    pub mime_type: String,
    /// Time offset in milliseconds used when synchronising multiple media files.
    #[serde(rename = "@TIME_ORIGIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_origin: Option<u64>,
    /// Path to e.g. the video which a wav-file was extracted from.
    #[serde(rename = "@EXTRACTED_FROM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_from: Option<String>,
}

impl Default for MediaDescriptor {
    fn default() -> Self {
        Self {
            media_url: String::default(),
            relative_media_url: None,
            mime_type: String::default(),
            time_origin: None,
            extracted_from: None,
        }
    }
}

impl MediaDescriptor {
    /// Createa a new media descriptor.
    pub fn new(path: &Path, extracted_from: Option<&Path>) -> Result<Self, EafError> {
        let mut mdsc = MediaDescriptor::default();
        let url = url_from_path(path)?;
        mdsc.media_url = url.to_string();
        mdsc.mime_type = MimeType::from_path(path).to_string();
        if let Some(p) = extracted_from {
            mdsc.extracted_from = Some(url_from_path(p)?.to_string());
        }
        Ok(mdsc)
    }

    /// Returns media URL verbatim (absolute media path).
    pub fn media_url(&self) -> &str {
        self.media_url.as_str()
    }

    /// Returns relative media URL verbatim (relative media path).
    pub fn relative_media_url(&self) -> Option<&str> {
        self.relative_media_url.as_deref().map(|m| m)
    }

    /// Returns absolute media path.
    pub fn abs_path(&self) -> Option<PathBuf> {
        let url = Url::parse(&self.media_url).ok()?;
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
        self.relative_media_url
            .as_ref()
            .map(|s| PathBuf::from(s))
    }

    /// Returns `true` if the relative media path is valid.
    pub fn rel_exists(&self) -> bool {
        self.rel_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// Returns media path if the corresponding file exists.
    pub fn path(&self) -> Option<PathBuf> {
        match (self.abs_exists(), self.rel_exists()) {
            (true, _) => self.abs_path(),
            (_, true) => self.rel_path(),
            _ => None
        }
        // PathBuf::from(self.media_url.path())
    }

    /// Returns filename as `&OsStr`. Prioritises absolute media url.
    pub fn file_name(&self) -> Option<OsString> {
        self.path()
            .as_deref()
            .and_then(|p| p.file_name())
            .map(|f| f.to_owned())
    }

    /// Sets absolute media path.
    pub fn set_media_abs(&mut self, path: &Path) -> Result<(), EafError> {
        self.mime_type = MimeType::from_path(path).to_string();
        self.media_url = url_from_path(path)?.to_string();
        Ok(())
    }

    /// Sets relative media path.
    pub fn set_media_rel(
        &mut self,
        rel_path: &Path,
        filename_only: bool,
    ) {
        self.mime_type = MimeType::from_path(rel_path).to_string();
        if filename_only {
            self.relative_media_url = {
                let name = rel_path.file_name()
                    .and_then(|n| n.to_str());
                name
                    .map(|n| format!("./{}", n)) // ok on windows?
            };
        } else {
            self.relative_media_url = Some(rel_path.display().to_string());
        }
    }

    /// Matches file names, not full path, to check if media descriptor contains path.
    pub fn contains(&self, path: &Path) -> bool {
        if let (Some(fn_self), Some(fn_in)) = (self.file_name(), path.file_name()) {
            return fn_self == fn_in;
        }
        false
    }

    // /// Extract specified time span for the media path and sets
    // /// the path to the new media file.
    // /// `suffix` = `true` adds start end end times
    // ///
    // /// Note that FFmpeg is required, meaning this functionality is perhaps better
    // /// to implment in an end user tool, as opposed to the unerlying crate.
    // pub fn timespan(
    //     &mut self,
    //     start: i64,
    //     end: i64,
    //     suffix_filename: bool,
    //     ffmpeg_path: Option<&Path>
    // ) -> Result<(), EafError> {
    //     if start < 0 || end < 0 {
    //         return Err(EafError::ValueTooSmall(start))
    //     }

    //     let mut media_path = self.path()
    //         .ok_or(EafError::PathInvalid(self.media_url.to_owned()))?;
    //     if suffix_filename {
    //         media_path = affix_file_name(&media_path, None, Some(&format!("{start}-{end}")))
    //     }

    //     // // First try relative media path + media dir (e.g. eaf-dir containing eaf + media-file)...
    //     // let media_path = if let (Some(dir), Some(rel_path)) = (media_dir, &self.relative_media_url) {
    //     //     let mut dir = dir.to_owned();
    //     //     dir.push(rel_path);
    //     //     dir
    //     // } else {
    //     // // ...or default to media url.
    //     //     PathBuf::from(&self.media_url)
    //     // };

    //     self.media_url = FFmpeg::extract_timespan(&media_path, start as u64, end as u64, None, ffmpeg_path)?
    //         .display()
    //         .to_string()
    //         .trim_start_matches("file://") // media url/absolute path starts with "file://"
    //         .to_owned();

    //     Ok(())
    // }
}

/// Extremly simplified mime type check from file extension
/// that may eventually include more media types,
/// but will always be focused on
/// media types that ELAN supports.
enum MimeType {
    Wav,
    Mp4,
    Mpeg,
    Other(String), // file extension
}

impl MimeType {
    /// Returns mime type for linked media files.
    ///
    /// This is only intended for determining mime type for
    /// ELAN-compatible multimedia files.
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .map(|o| o.to_string_lossy().to_string())
            .unwrap_or(String::from("none"))
            .to_lowercase();

        match ext.as_ref() {
            "mp4" => MimeType::Mp4,
            "wav" => MimeType::Wav,
            "mpg" | "mpeg" => MimeType::Mpeg,
            _ => MimeType::Other(ext),
        }
    }

    /// Returns a mime type string
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Wav => "audio/x-wav".to_owned(),
            MimeType::Mp4 => "video/mp4".to_owned(),
            // presumably not mpeg1...?
            MimeType::Mpeg => "video/mpeg2".to_owned(),
            MimeType::Other(s) => format!("application/{}", s.to_owned()),
        }
    }
}
