//! Pfsx preference group.

use serde::{Deserialize, Serialize};

use super::pref::Pref;

/// Pfsx preference group.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "prefGroup")]
pub struct PrefGroup {
    #[serde(rename="@key", default)]
    pub key: String,
    #[serde(rename="pref", default)]
    pub preferences: Vec<Pref>,
}