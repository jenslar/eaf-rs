//! License.

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// License.
pub struct License {
    url: Option<String>
}
