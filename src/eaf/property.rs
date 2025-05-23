//! Property.
//!
//! Optional key, value store specifying position at exit,
//! but can be used for custom user data,
//! as long as a unique "name" attribute is used.

use serde::{Serialize, Deserialize};

/// Optional key, value store specifying position at exit,
/// but can be used for custom user data,
/// as long as a unique "name" attribute is used.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Property {
    /// Name (must be unique)
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    /// Text content.
    #[serde(rename = "$value")]
    pub value: String,
}
