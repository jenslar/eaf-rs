//! License.

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// License.
pub struct License {
    url: Option<String>
}

impl From<&str> for License {
    fn from(value: &str) -> Self {
        Self { url: Some(String::from(value)) }
    }
}

impl From<String> for License {
    fn from(value: String) -> Self {
        Self { url: Some(String::from(value)) }
    }
}
