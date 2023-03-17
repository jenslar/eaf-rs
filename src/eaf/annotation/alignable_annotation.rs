use serde::{Deserialize, Serialize};

use super::AnnotationValue;

/// Alignable annotation. An annotation type found in a main tier,
/// with explicit time slot references.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "ALIGNABLE_ANNOTATION")] // causes double annotation tags
pub struct AlignableAnnotation {
    // Attributes

    /// Attribute annotation ID.
    #[serde(rename="@ANNOTATION_ID")]
    pub annotation_id: String,
    /// Attribute external reference.
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    /// Attribute language reference.
    #[serde(rename="@LANG_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_ref: Option<String>,
    /// Attribute CVE reference.
    #[serde(rename="@CVE_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_ref: Option<String>,
    /// Time slot reference 1 (start of annotation)
    #[serde(rename="@TIME_SLOT_REF1")]
    pub time_slot_ref1: String,
    /// Time slot reference 2 (end of annotation)
    #[serde(rename="@TIME_SLOT_REF2")]
    pub time_slot_ref2: String,
    
    // Child nodes

    /// Child node annotation value.
    pub annotation_value: AnnotationValue,
    // pub annotation_value: String
    #[serde(skip)]
    pub tier_id: Option<String>, // not part of EAF spec
    #[serde(skip)]
    pub time_value1: Option<i64>, // not part of EAF spec, for populating/editing time order
    #[serde(skip)]
    pub time_value2: Option<i64>, // not part of EAF spec, for populating/editing time order
}

impl Default for AlignableAnnotation {
    fn default() -> Self {
        Self {
            annotation_id: "a1".to_owned(),
            ext_ref: None,
            lang_ref: None,
            cve_ref: None,
            time_slot_ref1: "ts1".to_owned(),
            time_slot_ref2: "ts2".to_owned(),
            annotation_value: AnnotationValue::default(),
            tier_id: None,
            time_value1: None,
            time_value2: None
        }
    }
}