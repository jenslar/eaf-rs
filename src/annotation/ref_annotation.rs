//! Referred annotation.
//! 
//! References an annotation in the parent tier. I.e. does not contain explicit time slot references.
//! These must be derived via the main annotation. Note that a referred annotation may refer to another
//! referred annotation.

use serde::{Deserialize, Serialize};

use crate::{Annotation, EafError};

use super::{AnnotationValue, AnnotationBuilder};

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
    pub fn from_values(
        annotation_id: &str,
        annotation_ref: &str,
        annotation_value: &str
    ) -> Result<Annotation, EafError> {
        AnnotationBuilder::new()
            .annotation_id(annotation_id)
            .annotation_ref(annotation_ref)
            .annotation_value(annotation_value)
            .build()
    }

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

    /// Edit annotation ID.
    pub fn with_annotation_id(self, annotation_id: &str) -> Self {
        Self {
            annotation_id: annotation_id.to_owned(),
            ..self
        }
    }

    /// Edit external reference.
    pub fn with_ext_ref(self, ext_ref: &str) -> Self {
        Self {
            ext_ref: Some(ext_ref.to_owned()),
            ..self
        }
    }

    /// Edit language reference.
    pub fn with_lang_ref(self, lang_ref: &str) -> Self {
        Self {
            lang_ref: Some(lang_ref.to_owned()),
            ..self
        }
    }

    /// Edit CVE reference.
    pub fn with_cve_ref(self, cve_ref: &str) -> Self {
        Self {
            cve_ref: Some(cve_ref.to_owned()),
            ..self
        }
    }

    /// Edit annotation reference ("parent" annotation ID).
    pub fn with_annotation_ref(self, annotation_ref: &str) -> Self {
        Self {
            annotation_ref: annotation_ref.to_owned(),
            ..self
        }
    }

    /// Edit previous annotation ID (annotation cluster in tokenized tier).
    pub fn with_previous_annotation(self, previous_annotation: &str) -> Self {
        Self {
            previous_annotation: Some(previous_annotation.to_owned()),
            ..self
        }
    }

    /// Edit annotation value.
    pub fn with_annotation_value(self, annotation_value: &str) -> Self {
        Self {
            annotation_value: AnnotationValue::from(annotation_value),
            ..self
        }
    }
}