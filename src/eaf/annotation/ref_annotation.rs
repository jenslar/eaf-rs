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

    // Annotations (child nodes)

    /// Child node annotation value.
    pub annotation_value: AnnotationValue,

    /// Tier ID (not part of EAF spec).
    #[serde(skip)]
    pub tier_id: Option<String>,
    /// Timeslot start value in milliseconds, for populating/editing time order (not part of EAF spec).
    #[serde(skip)]
    pub time_value1: Option<i64>,
    /// Timeslot end value in milliseconds, for populating/editing time order (not part of EAF spec).
    #[serde(skip)]
    pub time_value2: Option<i64>,
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

impl RefAnnotation {
    /// Add optional time values in millisseconds. Not part of EAF spec.
    pub fn with_time_val(self,time_value1: i64, time_value2: i64) -> Self {
        Self {
            time_value1: Some(time_value1),
            time_value2: Some(time_value2),
            ..self
        }
    }

    /// Add optional tier ID for the tier the annotation belongs to.
    /// Not part of EAF spec.
    pub fn with_tier_id(self, tier_id: &str) -> Self {
        Self {
            tier_id: Some(tier_id.to_owned()),
            ..self
        }
    }
}