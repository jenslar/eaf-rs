use std::{path::{Path, PathBuf}, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{EafError, Track, TrackProperty};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename = "tracksource")]
pub struct TrackSource {
    /// Sample rate type.
    /// One of:
    /// - "Discontinuous Rate"
    /// - "Continuous Rate"
    #[serde(rename = "@sample-type")]
    sample_type: String, // enum instead?
    /// Absolute path to CSV-file.
    /// Ensure `.csv` file extension
    /// or ELAN may not detect that the
    /// file is a timeseries.
    #[serde(rename = "@source-url")]
    source_url: String, // Url type instead?
    #[serde(rename = "@time-column")]
    time_column: usize,
    property: TrackProperty,
    #[serde(rename = "track")]
    tracks: Vec<Track>,
}

impl TrackSource {
    pub fn new() -> Self {
        Self {

            ..Self::default()
        }
    }

    //7 Create track with discontinuous time stamps. Time column inex is 0-based.
    pub fn discontinuous(csv_path: &Path, time_column_index: usize) -> Result<TrackSource, EafError> {
        let url = Url::from_file_path(csv_path)
            .map_err(|_| EafError::UrlFromPathError(csv_path.to_path_buf()))?;
        Ok(Self {
            sample_type: "Discontinuous Rate".to_string(),
            source_url: url.to_string(),
            time_column: time_column_index,
            property: TrackProperty::csv_source(),
            ..Self::default()
        })
    }

    //7 Create track with continuous time stamps.
    pub fn continuous(csv_path: &Path) -> Result<TrackSource, EafError> {
        let url = Url::from_file_path(csv_path)
            .map_err(|_| EafError::UrlFromPathError(csv_path.to_path_buf()))?;
        Ok(Self {
            sample_type: "Continuous Rate".to_string(),
            source_url: url.to_string(),
            // time_column, // need to verify what this should contain for continuous
            property: TrackProperty::csv_source(),
            ..Self::default()
        })
    }

    pub fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.tracks.iter()
    }

    pub fn add_track(&mut self, track: &Track) {
        self.tracks.push(track.to_owned());
    }

    pub fn add_tracks(&mut self, tracks: &[Track]) {
        self.tracks.extend_from_slice(tracks);
    }

    pub fn with_tracks(self, tracks: Vec<Track>) -> TrackSource {
        Self {
            tracks,
            ..self
        }
    }

    /// Returns the path for linked CSV file.
    pub fn source_path(&self) -> Result<PathBuf, EafError> {
        Url::from_str(&self.source_url)?
            .to_file_path()
            .map_err(|_| EafError::UrlToPathError(self.source_url.to_owned()))
    }

    pub fn tracknames(&self) -> impl Iterator<Item=&str> {
        self.tracks.iter()
            .map(|t| t.name.as_str())
    }
}
