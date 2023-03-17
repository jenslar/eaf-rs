//! EAF linguistic type.

use serde::{Serialize, Deserialize};

use super::StereoType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
/// EAF linguistic type.
pub struct LinguisticType {
    #[serde(rename="@LINGUISTIC_TYPE_ID")]
    pub linguistic_type_id: String,
    #[serde(rename="@TIME_ALIGNABLE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_alignable: Option<bool>,
    #[serde(rename="@CONSTRAINTS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<String>, // ideally Constraint enum
    #[serde(rename="@GRAPHIC_REFERENCES")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_references: Option<bool>,
    #[serde(rename="@CONTROLLED_VOCABULARY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled_vocabulary: Option<String>,
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    #[serde(rename="@LEXICON_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_ref: Option<String>, // refers to REF ID in top-level element LEXICON_REF
}

impl Default for LinguisticType {
    fn default() -> Self {
        Self {
            linguistic_type_id: "default-lt".to_owned(),
            time_alignable: Some(true),
            constraints: None,
            graphic_references: None,
            controlled_vocabulary: None,
            ext_ref: None,
            lexicon_ref: None,
        }
    }
}

impl LinguisticType {
    /// Checks whether the linguistic type is time alignable, depending on constraints.
    pub fn time_alignable(stereotype: StereoType, has_constraint: bool) -> bool {
        match (stereotype, has_constraint) {
            (StereoType::IncludedIn, true) => true, // time alignable: true
            (StereoType::SymbolicAssociation, true) => false, // time alignable: true
            (StereoType::SymbolicSubdivision, true) => false, // time alignable: true
            (StereoType::TimeSubdivision, true) => true, // time alignable: true
            (_, false) => true
        }
    }
    
    pub fn new(id: &str, stereotype: Option<&StereoType>) -> Self {
        let alignable = match stereotype {
            Some(s) => Some(s.time_alignable()),
            None => Some(true)
        };
        Self{
            linguistic_type_id: id.to_owned(),
            time_alignable: alignable,
            constraints: stereotype.map(|s| s.to_owned().into()),
            // constraints: stereotype.map(|s| s.to_constraint().stereotype),
            graphic_references: None,
            controlled_vocabulary: None,
            ext_ref: None,
            lexicon_ref: None, // refers to element LEXICON_REF
        }
    }
}
