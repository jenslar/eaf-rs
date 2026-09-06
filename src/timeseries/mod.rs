//! Time series configuration file.

pub mod config;
pub mod source;
pub mod track;

pub use config::TimeSeriesConfig;
pub use source::TrackSource;
pub use track::{Track, SamplePosition, Position, Units, Description, TrackProperty, Range, Color};
