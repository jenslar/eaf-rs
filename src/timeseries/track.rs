use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Track {
    #[serde(rename = "@derivative")]
    // derivative: String,
    pub derivative: u8, // 0-4
    #[serde(rename = "@name")]
    pub name: String,
    pub property: TrackProperty,
    #[serde(rename = "sample-position")]
    pub sample_position: SamplePosition,
    pub description: Option<Description>,
    pub units: Option<Units>,
    pub range: Range,
    pub color: Color,
}

impl Track {

}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename = "property")]
pub struct TrackProperty {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@value")]
    pub value: String, // bool?
}

impl TrackProperty {
    pub fn csv_source() -> TrackProperty {
        Self {
            key: "provider".to_string(),
            value: "mpi.eudico.client.annotator.timeseries.csv.CSVServiceProvider".to_string()
        }
    }
}

// impl Default for TrackProperty {
//     fn default() -> Self {
//         Self {
//             key: "provider".to_string(),
//             value: "mpi.eudico.client.annotator.timeseries.csv.CSVServiceProvider".to_string()
//         }
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "sample-position")]
pub struct SamplePosition {
    #[serde(rename = "pos")]
    position: Position
}

impl SamplePosition {
    pub fn new(column: usize, row: usize) -> SamplePosition {
        Self {
            position: Position { col: column, row },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Position {
    #[serde(rename = "@col")]
    col: usize,
    #[serde(rename = "@row")]
    row: usize,
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
    max: f64,
    #[serde(rename = "@min")]
    min: f64,
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            max,
            min,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Color {
    #[serde(rename = "$value")]
    value: String, // e.g. 0,255,0 (rgb? de/serialize value?)
}

impl Color {
    pub fn red() -> Self {
        Self {
            value: "255,0,0".to_string()
        }
    }

    pub fn green() -> Self {
        Self {
            value: "0,255,0".to_string()
        }
    }

    pub fn blue() -> Self {
        Self {
            value: "0,0,255".to_string()
        }
    }

    pub fn black() -> Self {
        Self {
            value: "0,0,0".to_string()
        }
    }
}
