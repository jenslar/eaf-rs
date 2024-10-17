//! Controlled vocabulary.

use serde::{Serialize, Deserialize};

// using enum for annotation type
// see: https://users.rust-lang.org/t/serde-deserializing-a-vector-of-enums/51647
// TODO errors on optional <DESCRIPTION ...> child element in v2.8 eaf, parallel to entry:
// /Users/jens/dev/TESTDATA/eaf/2014-10-13_1800_US_CNN_Newsroom_12-493.eaf
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CONTROLLED_VOCABULARY")]
pub struct ControlledVocabulary {
    // Attributes

    /// Controlled Vocabulary ID
    #[serde(rename="@CV_ID")]
    pub cv_id: String,
    /// External reference
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    /// Description.
    /// Invalid attribute in EAF v2.8+,
    /// can instead be a value parallel to entry.
    #[serde(rename="@DESCRIPTION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    // Child nodes
    #[serde(rename = "$value")]
    pub entry: Vec<CVType>,
}

impl Default for ControlledVocabulary {
    fn default() -> Self {
        Self {
            cv_id: String::default(),
            ext_ref: None,
            description: None,
            entry: vec!(CVType::CvEntryMl(CvEntryMl::default()))
        }
    }
}

/// Contains the possibilities for CV entries,
/// depending on EAF version.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CVType {
    /// EAF v2.8+ Description, 0 - 1 occurrences.
    // #[serde(rename(deserialize = "DESCRIPTION"))] // why only deserialize?
    #[serde(rename = "DESCRIPTION")]
    Description(Description),
    /// EAF v2.7-, 0+ occurrences.
    // #[serde(rename(deserialize = "CV_ENTRY"))]
    #[serde(rename = "CV_ENTRY")]
    CvEntry(CvEntry),
    /// EAF v2.8+, 0+ occurrences.
    // #[serde(rename(deserialize = "CV_ENTRY_ML"))]
    #[serde(rename = "CV_ENTRY_ML")]
    CvEntryMl(CvEntryMl),
}

/// Controlled Vocabulary Entry for EAF v2.7
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CV_ENTRY")]
pub struct CvEntry {
    #[serde(rename="@DESCRIPTION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    #[serde(rename = "$value")]
    pub value: String
}

/// Controlled Vocabulary Entry for EAF v2.8+
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CV_ENTRY_ML")]
pub struct CvEntryMl {
    #[serde(rename="@CVE_ID")]
    pub cve_id: String,
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    #[serde(rename = "CVE_VALUE")]
    pub cve_values: Vec<CveValue>,
}

impl Default for CvEntryMl {
    fn default() -> Self {
        Self {
            cve_id: String::default(),
            ext_ref: None,
            cve_values: Vec::new(),
        }
    }
}

// !!! this currently creates the wrong structure in KebabCase:
// !!! e.g. <Description><Description lang_reg="eng"/></Description>, rather than
// !!! e.g. <DESCRIPTION lang_reg="eng"/>
/// EAF v2.8+
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CVE_VALUE")]
pub struct CveValue {
    #[serde(rename="@DESCRIPTION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename="@LANG_REF")]
    pub lang_ref: String,
    #[serde(rename = "$value")]
    pub value: String
}

// EAF v2.8+
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub struct Description {
    #[serde(rename="@LANG_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_ref: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}
