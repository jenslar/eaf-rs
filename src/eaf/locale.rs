//! Locale.
//! 
//! Specifies country, language, language variant.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF locale.
pub struct Locale {
    #[serde(rename="@LANGUAGE_CODE")]
    pub language_code: String,
    #[serde(rename="@COUNTRY_CODE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename="@VARIANT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            language_code: "eng".to_owned(),
            country_code: None,
            variant: None,
        }
    }
}