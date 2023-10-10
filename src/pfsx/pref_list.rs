//! Pfsx preference list.

use serde::{Deserialize, Serialize};

use super::pref::Value;

/// Pfsx preference list.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "prefList")]
pub struct PrefList {
    #[serde(rename="@key", default)]
    pub key: String,
    #[serde(rename="$value", default)]
    pub preferences: Vec<Value>,
}