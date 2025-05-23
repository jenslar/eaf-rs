//! Language.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
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

impl Language {
    pub fn english() -> Self {
        Self {
            lang_id: "eng".to_string(),
            lang_def: Some("http://cdb.iso.org/lg/CDB-00138502-001".to_string()),
            lang_label: Some("English (eng)".to_string()),
        }
    }
}
