//! Controlled vocabulary.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use quick_xml::SeError;
use serde::{Deserialize, Serialize};
use quick_xml::se::Serializer;
use uuid::Uuid;

use super::Language;
use super::{xmlns_xsi, xsi_no_name_space_schema_location, today};

/// Controlled vocabulary resource for exporting vocabularies to `.ecv` file.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename = "CV_RESOURCE")]
pub struct CvResource {
    // Header attributes

    /// Controlled Vocabulary author
    #[serde(rename="@AUTHOR")]
    pub author: String,

    /// Controlled Vocabulary date
    #[serde(rename="@DATE", default = "today")]
    pub date: String,

    /// Controlled Vocabulary date
    #[serde(rename="@VERSION")]
    pub version: String,

    #[serde(rename="@xmlns:xsi", default="xmlns_xsi")]
    pub xmlns_xsi: String,

    #[serde(rename="@xsi:noNamespaceSchemaLocation",
        default = "xsi_no_name_space_schema_location")]
    pub xsi_nonamespaceschemalocation: String,

    #[serde(rename="LANGUAGE")]
    pub language: Option<Language>,

    #[serde(rename="CONTROLLED_VOCABULARY")]
    pub vocabularies: Vec<ControlledVocabulary>,
}

impl Default for CvResource {
    fn default() -> Self {
        Self {
            author: "CRMarhaceo-writer".to_string(),
            date: today(),
            version: "0.2".to_string(),
            xmlns_xsi: xmlns_xsi(),
            xsi_nonamespaceschemalocation: xsi_no_name_space_schema_location(),
            language: None,
            vocabularies: Vec::new(),
        }
    }
}

impl CvResource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_language(&mut self, language: &Language) {
        self.language = Some(language.to_owned())
    }

    pub fn add_vocabulary(&mut self, vocabulary: &ControlledVocabulary) {
        self.vocabularies.push(vocabulary.to_owned())
    }

    pub fn to_ecv(&self, indent: Option<usize>) -> Result<String, SeError> {
        let mut ecv_str = String::new();
        let mut ser = Serializer::new(&mut ecv_str);
        // Optionally indent serialized XML
        if let Some(ind) = indent {
            ser.indent(' ', ind);
        }

        self.serialize(ser)?; //.map_err(|e| EafError::QuickXMLSeError(e))?;

        Ok([
            "<?xml version='1.0' encoding='UTF-8'?>",
            &ecv_str
            ].join("\n"))
    }

    pub fn write_ecv(&self, path: &Path, indent: Option<usize>) -> std::io::Result<()> {
        let ecv_str = self.to_ecv(indent)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = File::create(path)?;

        file.write_all(ecv_str.as_bytes())
    }
}

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
    pub description_attr: Option<String>,

    /// Description.
    /// Invalid element in EAF <v2.8,
    /// can instead be an attributes in CV header.
    #[serde(rename="DESCRIPTION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_value: Option<Description>,

    // Child nodes
    #[serde(rename = "$value")]
    pub entries: Option<Vec<CvType>>,
}

impl Default for ControlledVocabulary {
    fn default() -> Self {
        Self {
            // cv_id: format!("cveid_{}", Uuid::new_v4().to_string()),
            cv_id: String::default(),
            ext_ref: None,
            description_attr: None,
            description_value: None,
            // entry: vec!(CvType::CvEntryMl(CvEntryMl::default()))
            entries: None,
        }
    }
}

impl ControlledVocabulary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CvType>{
        self.entries.iter().flat_map(|e| e)
    }

    pub fn add_entry(
        &mut self,
        entry: &CvEntryMl
    ) {
        // self.entry.push(CvType::CvEntryMl(entry.to_owned()));
        if let Some(entries) = &mut self.entries {
            entries.push(CvType::CvEntryMl(entry.to_owned()));
        }
    }
}

/// Contains the possibilities for CV entries,
/// depending on EAF version.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CvType {
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
    pub cve_values: Vec<CveValue>, // is this really multiple value or only a single one?
}

impl Default for CvEntryMl {
    fn default() -> Self {
        Self {
            cve_id: format!("cveid_{}", Uuid::new_v4().to_string()),
            ext_ref: None,
            cve_values: Vec::new(),
        }
    }
}

impl CvEntryMl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_value(&mut self, value: &CveValue) {
        self.cve_values.push(value.to_owned());
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

impl Description {
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|v| v.as_str())
    }
}
