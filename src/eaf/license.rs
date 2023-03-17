//! EAF license.

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF license.
pub struct License {
    url: Option<String>
}
