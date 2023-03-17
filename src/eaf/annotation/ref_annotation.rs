use serde::{Deserialize, Serialize};

use super::AnnotationValue;

/// Reference annotation. An annotation type found in reference tiers,
/// which refers to an annotation in its parent tier. Has no explicit
/// time slot references. These must derived via its parent annotation/s.
/// `AnnotationDocument.derive()` will do this, and populate,
/// `tier_id`, `time_slot_val1`,`time_slot_val2`, and `main_annotation`
/// (annotation ID for the corresponding alignable annotation in
/// the main tier for this hierarchy.)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "REF_ANNOTATION")] // causes double annotation tags
pub struct RefAnnotation{
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
    /// Attribute annotation reference in parent tier.
    #[serde(rename="@ANNOTATION_REF")]
    pub annotation_ref: String,
    /// Attribute previous annotation for tokenized tiers.
    #[serde(rename="@PREVIOUS_ANNOTATION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_annotation: Option<String>,

    // Child nodes

    /// Child node annotation value.
    pub annotation_value: AnnotationValue,
    /// Tier ID.
    #[serde(skip)]
    pub tier_id: Option<String>, // not part of EAF spec
    /// Timeslot start value in milliseconds.
    #[serde(skip)]
    pub time_value1: Option<i64>, // not part of EAF spec, for populating/editing time order
    /// Timeslot end value in milliseconds.
    #[serde(skip)]
    pub time_value2: Option<i64>, // not part of EAF spec, for populating/editing time order
    #[serde(skip)]
    pub main_annotation: Option<String>, // not part of EAF spec, for populating/editing time order
}

impl Default for RefAnnotation {
    fn default() -> Self {
        Self {
            annotation_id: "a2".to_owned(),
            ext_ref: None,
            lang_ref: None,
            cve_ref: None,
            annotation_ref: "a1".to_owned(),
            previous_annotation: None,
            annotation_value: AnnotationValue::default(),
            tier_id: None,
            time_value1: None,
            time_value2: None,
            main_annotation: None
        }
    }
}