//! Language.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// Language.
pub struct Language {
    #[serde(rename="@LANG_ID")]
    pub lang_id: String,
    #[serde(rename="@LANG_DEF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_def: Option<String>,
    #[serde(rename="@LANG_LABEL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_label: Option<String>,
}
