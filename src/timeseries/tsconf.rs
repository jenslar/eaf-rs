//! Time series configuration file created by ELAN when linking CSV timeseries files
//! (Menu: "Edit" `->` "Linked files..." `->` tab "Linked secondary files").
//! 
//! File naming convention: For `myelanfile.eaf`, the time series config file should be named `myelanfile_tsconf.xml`.
//! The ELAN file links both the timeseries file and the time series configuration file.
//! 
//! Example file:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <timeseries date="2023-09-18T15:54:44+01:00" version="1.0">
//!     <tracksource sample-type="Discontinuous Rate"
//!         source-url="file:///Users/jens/Desktop/GL010042/GL010042_LO_GPS.csv" time-column="2">
//!         <property key="provider" value="mpi.eudico.client.annotator.timeseries.csv.CSVServiceProvider"/>
//!         <track derivative="0" name="altitude">
//!             <property key="detect-range" value="false"/>
//!             <sample-position>
//!                 <pos col="5" row="0"/>
//!             </sample-position>
//!             <description/>
//!             <units>m</units>
//!             <range max="450.0" min="300.0"/>
//!             <color>0,255,0</color>
//!         </track>
//!     </tracksource>
//! </timeseries>
//! ```

use std::{path::Path, fs::File, io::Write};

use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

use crate::EafError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "timeseries")]
pub struct TimeSeries {
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "tracksource")]
    tracksources: Vec<TrackSource>,
}

impl TimeSeries {
    /// Read time series configuration file.
    pub fn read(path: &Path) -> Result<TimeSeries, EafError> {
        Self::de(path)
    }
    
    /// Write time series configuration file, optionally with indentation
    /// specified in single spaces.
    /// If `indent` is `None`, the XML will be written on a single line.
    pub fn write(&self, path: &Path, indent: Option<usize>) -> Result<(), EafError> {
        let content = self.to_string(indent)?;
        let mut outfile = File::create(&path)?;
        
        outfile.write_all(content.as_bytes()).map_err(|e| e.into())
    }
    
    /// Serialize `TimeSeries`to XML string.
    pub fn to_string(&self, indent: Option<usize>) -> Result<String, EafError> {
        self.se(indent)
    }

    /// Deserialize time series configuration file.
    fn de(path: &Path) -> Result<TimeSeries, EafError> {
        // Let Quick XML use serde to deserialize
        quick_xml::de::from_str::<TimeSeries>(&std::fs::read_to_string(path)?)
            .map_err(|e| EafError::QuickXMLDeError(e))
    }

    /// Serialize time series configuration file.
    fn se(&self, indent: Option<usize>) -> Result<String, EafError> {
        let mut ts_str = String::new();
        // Create serializer that indents XML with 4 spaces
        let mut ser = Serializer::new(&mut ts_str);
        if let Some(i) = indent {
            ser.indent(' ', i);
        }

        self.serialize(ser).map_err(|e| EafError::QuickXMLDeError(e))?;

        Ok([
            // Add XML declaration, since not added by quick-xml
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            ts_str.as_str()
        ].join("\n"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "tracksource")]
pub struct TrackSource {
    #[serde(rename = "@sample-type")]
    sample_type: String,
    #[serde(rename = "@source-url")]
    source_url: String,
    #[serde(rename = "@time-column")]
    time_column: String,
    property: Property,
    #[serde(rename = "track")]
    tracks: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Track {
    #[serde(rename = "@derivative")]
    derivative: String,
    #[serde(rename = "@name")]
    name: String,
    property: Property,
    #[serde(rename = "sample-position")]
    sample_position: SamplePosition,
    description: Description,
    units: Units,
    range: Range,
    color: Color,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "sample-position")]
pub struct SamplePosition {
    #[serde(rename = "pos")]
    position: Position
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Position {
    #[serde(rename = "@col")]
    col: String,
    #[serde(rename = "@row")]
    row: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Description {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Units {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Range {
    #[serde(rename = "@max")]
    max: String,
    #[serde(rename = "@min")]
    min: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Color {
    #[serde(rename = "$value")]
    value: String, // e.g. 0,255,0 (rgb? de/serialize value?)
}