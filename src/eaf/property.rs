//! EAF property.

use serde::{Serialize, Deserialize};

/// Optional key, value store in EAF header.
/// Can be used to store custom information (be sure to pick a
/// unique attribute name).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Property {
    /// Name, value seems optional,
    /// but attribute should exist...?
    /// Currently serializes to `Some("")` if `None`.
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    /// Text content.
    #[serde(rename = "$value")]
    pub value: String,
}
