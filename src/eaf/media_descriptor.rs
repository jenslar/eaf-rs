//! Media descriptor.
//! 
//! Specifies a linked media file, including path, and mime type,
//! but also an optional time offset (`time_origin`),
//! used by ELAN as a starting point to be able to synchronise media files.
//! 
//! Part of the header.

use std::{path::{Path, PathBuf}, ffi::OsStr};
use serde::{Serialize, Deserialize};

use crate::ffmpeg;

use super::{eaf::path_to_string, EafError};

/// Media descriptor.
/// 
/// Specifies a linked media file, including path, and mime type,
/// but also an optional time offset (`time_origin`),
/// used by ELAN as a starting point to be able to synchronise media files.
/// 
/// Part of the header.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MEDIA_DESCRIPTOR")]
pub struct MediaDescriptor {
    /// Absolute media file path.
    #[serde(rename="@MEDIA_URL")]
    pub media_url: String, // is abs path really required? or...
    /// Relative media file path.
    #[serde(rename="@RELATIVE_MEDIA_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_media_url: Option<String>, // ...is it rel path that is required?
    /// Media mime type.
    #[serde(rename="@MIME_TYPE")]
    pub mime_type: String,
    /// Time offset in milliseconds used when synchronising multiple media files.
    #[serde(rename="@TIME_ORIGIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_origin: Option<u64>,
    /// Path to e.g. the video which a wav-file was extracted from.
    #[serde(rename="@EXTRACTED_FROM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_from: Option<String>,
}

impl Default for MediaDescriptor {
    fn default() -> Self {
        Self {
            media_url: "".to_owned(),
            relative_media_url: None,
            mime_type: "".to_owned(),
            time_origin: None,
            extracted_from: None,
        }
    }
}

impl MediaDescriptor {
    /// Createa a new media descriptor. Relative media path is set to
    /// filename only, e.g. `./VIDEO.MP4`.
    pub fn new(path: &Path, extracted_from: Option<&str>) -> Self {
        let mut mdsc = MediaDescriptor::default();
        // need prefix "file:///"
        mdsc.media_url = path_to_string(path, Some("file:///"), false);
        mdsc.relative_media_url = Some(path_to_string(path, Some("./"), true));
        mdsc.mime_type = MimeType::from_path(path).to_string();
        mdsc.extracted_from = extracted_from.map(String::from);
        mdsc
    }

    /// Returns filename as `&OsStr`. Prioritises absolute media url.
    pub fn file_name<'a>(&'a self) -> Option<&'a OsStr> {
        if &self.media_url.replace("file:///", "") != "" {
            Path::new(&self.media_url).file_name()
        } else if let Some(p) = &self.relative_media_url { 
            Path::new(p).file_name()
        } else {
            None
        }
    }

    /// Sets absolute media path, and optional relative path.
    pub fn set_path(&mut self, path: &Path, rel_path: Option<&Path>) {
        self.media_url = path_to_string(path, Some("file:///"), false);
        if let Some(rel) = rel_path {
            self.relative_media_url = Some(format!("./{}", rel.to_string_lossy()));
        }
        self.mime_type = MimeType::from_path(path).to_string();
    }

    /// Sets relative media path.
    pub fn set_rel_path(&mut self, rel_path: &Path, filename_only: bool) {
        self.relative_media_url = Some(path_to_string(rel_path, Some("./"), filename_only));
        self.mime_type = MimeType::from_path(rel_path).to_string();
    }

    /// Matches file names, not full path, to check if media descriptor contains path.
    pub fn contains(&self, path: &Path) -> bool {
        if let (Some(fn_self), Some(fn_in)) = (self.file_name(), path.file_name()) {
            return fn_self == fn_in
        }
        false
    }

    /// Extract specified time span for the media path and sets
    /// the path to the new media file.
    /// 
    /// Relative media url is prioritised if `media_dir` is specified, since the absolute
    /// media url may refer to invalid paths.
    pub fn timespan(&mut self, start: i64, end: i64, media_dir: Option<&Path>, ffmpeg_path: Option<&Path>) -> Result<(), EafError> {
        if start < 0 || end < 0 {
            return Err(EafError::ValueTooSmall(start))
        }

        // First try relative media path + media dir (e.g. eaf-dir containing eaf + media-file)...
        let media_path = if let (Some(dir), Some(rel_path)) = (media_dir, &self.relative_media_url) {
            let mut dir = dir.to_owned();
            dir.push(rel_path);
            dir
        } else {
        // ...or default to media url.
            PathBuf::from(&self.media_url)
        };

        self.media_url = ffmpeg::process::extract_timespan(&media_path, start as u64, end as u64, None, ffmpeg_path)?
            .display()
            .to_string()
            .trim_start_matches("file://") // media url/absolute path starts with "file://"
            .to_owned();

        Ok(())
    }
}

enum MimeType {
    Wav,
    Mp4,
    Mpeg,
    Other(String) // file extension
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
            _ => MimeType::Other(path.to_string_lossy().to_string()),
        }
    }

    /// Returns a mime type string
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Wav => "audio/x-wav".to_owned(),
            MimeType::Mp4 => "video/mp4".to_owned(),
            // presumably not mpeg1...?
            MimeType::Mpeg => "video/mpeg2".to_owned(),
            MimeType::Other(s) => format!("application/{}", s.to_owned())
        }
    }
}
