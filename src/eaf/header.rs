//! Header.
//!
//! Specifies linked media files, and other external files, such as time series CSV-files.

use std::path::{PathBuf, Path};

use serde::{Serialize, Deserialize};

use crate::EafError;

use super::{
    Property,
    MediaDescriptor, LinkedFileDescriptor
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// Header.
pub struct Header {
    // Attributes

    /// Name or path of a media file, optional.
    /// Deprecated and ignored by ELAN.
    #[serde(rename = "@MEDIA_FILE", default)]
    pub media_file: String,
    /// Milliseconds or NTSC-frames or PAL-frames, optional,
    /// default is milliseconds. ELAN only supports (and assumes) milliseconds.
    #[serde(rename = "@TIME_UNITS")]
    pub time_units: String,

    // Child elements

    /// Linked media files. Optional,
    /// since ELAN opens EAF-file with no linked media.
    #[serde(default)]
    pub media_descriptor: Vec<MediaDescriptor>,
    /// Linked files. Optional.
    #[serde(default)]
    pub linked_file_descriptor: Vec<LinkedFileDescriptor>,
    /// Property. Optional.
    /// Can be used as a custom key, value store.
    #[serde(rename = "PROPERTY", default)]
    pub properties: Vec<Property>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            media_file: "".to_owned(),
            time_units: "milliseconds".to_owned(),
            media_descriptor: Vec::default(),
            linked_file_descriptor: Vec::default(),
            properties: Vec::default(),
        }
    }
}

impl Header {
    pub fn new(paths: &[PathBuf]) -> Result<Self, EafError> {
        let mut hdr = Self::default();

        for path in paths.iter() {
            hdr.media_descriptor.push(MediaDescriptor::new(path, None)?);
        }

        Ok(hdr)
    }

    /// Adds a new media descriptor to the header.
    pub fn add_media(&mut self, path: &Path, extracted_from: Option<&Path>) -> Result<(), EafError> {
        let mdsc = MediaDescriptor::new(path, extracted_from)?;
        self.media_descriptor.push(mdsc);
        Ok(())
    }

    /// Removes specified media file if set.
    /// TODO Does not work as intended: should remove media descriptior entirely if filename matches,
    /// TODO or keep media descriptor with filename only for both media urls.
    // pub fn remove_media(&mut self, path: &Path, keep_filename: bool) {
    pub fn remove_media(&mut self, path: &Path) {
        self.media_descriptor.retain(|md| !md.contains(path))
    }

    /// Removes all media paths, with the option to keep the file name only.
    /// These sometimes contain user names and information which may be unwanted
    /// when e.g. sharing data.
    /// You may have to link media in ELAN again.
    pub fn scrub_media(&mut self, keep_filename: bool) -> Result<(), EafError> {
        match keep_filename {
            false => self.media_descriptor = Vec::new(),
            true => {
                for md in self.media_descriptor.iter_mut() {
                    if let Some(filename) = md.file_name().map(|s| s.to_string_lossy().to_string()) {
                        // md.media_url = format!("file://{}", filename);
                        // md.media_url = "file://".to_owned();
                        md.set_media_rel(&PathBuf::from(filename), keep_filename);
                        // md.media_url = "file://".to_owned();
                        // md.media_url = url_from_path(Path::new(""))?; // fails?
                        md.media_url = String::default(); // fails?
                    }
                }
            }
        }
        Ok(())
    }

    /// Adds a new property to the header.
    pub fn add_property(&mut self, property: &Property) {
        self.properties.push(property.to_owned())
    }

    /// Returns all media paths as tuples,
    /// `(media_url, relative_media_url)`
    pub fn media_paths(&self) -> Vec<(PathBuf, Option<PathBuf>)> {
        self.media_descriptor.iter()
            .filter_map(|m| Some((m.abs_path()?, m.rel_path())))
            .collect()
    }

    /// Returns all absolute media paths.
    // pub fn media_abs_paths(&self) -> Vec<String> {
    pub fn media_abs_paths(&self) -> Vec<PathBuf> {
        self.media_descriptor.iter()
            .filter_map(|m| m.abs_path())
            .collect()
    }

    /// Returns all relative media paths (optional value).
    // pub fn media_rel_paths(&self) -> Vec<String> {
    pub fn media_rel_paths(&self) -> Vec<PathBuf> {
        self.media_descriptor.iter()
            .filter_map(|m| m.rel_path())
            .collect()
    }

    // // pub fn timespan(&mut self, start: i64, end: i64, media_dir: Option<&Path>, ffmpeg_path: Option<&Path>) -> Result<(), EafError> {
    // pub fn timespan(&mut self, start: i64, end: i64, suffix_filename: bool, ffmpeg_path: Option<&Path>) -> Result<(), EafError> {
    //     for mdsc in self.media_descriptor.iter_mut() {
    //         // mdsc.timespan(start, end, media_dir, ffmpeg_path)?;
    //         mdsc.timespan(start, end, suffix_filename, ffmpeg_path)?;
    //     }
    //     Ok(())
    // }
}
