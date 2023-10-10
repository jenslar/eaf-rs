//! Lexicon reference.

use serde::{Serialize, Deserialize};
use super::eaf::unspecified;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "LEXICON_REF")]
/// Lexicon reference.
pub struct LexiconRef {
    #[serde(rename = "@LEX_REF_ID")]
    pub lex_ref_id: String,
    #[serde(rename = "@NAME")]
    pub name: String,
    #[serde(rename = "@TYPE")]
    pub component_type: String,
    #[serde(rename = "@URL")]
    pub url: String,
    #[serde(rename = "@LEXICON_ID")]
    pub lexicon_id: String,
    #[serde(rename = "@LEXICON_NAME")]
    pub lexicon_name: String,
    #[serde(rename = "@DATCAT_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datcat_id: Option<String>,
    #[serde(rename = "@DATCAT_NAME")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datcat_name: Option<String>,
}

impl Default for LexiconRef {
    fn default() -> Self {
        Self {
            lex_ref_id: String::default(),
            name: String::default(),
            component_type: String::default(),
            url: unspecified(),
            lexicon_id: String::default(),
            lexicon_name: String::default(),
            datcat_id: None,
            datcat_name: None,
        }
    }
}
