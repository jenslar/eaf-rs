//! Alignable annotation.
//! 
//! Part of a main tier, with explicit time slot references.

use serde::{Deserialize, Serialize};

use crate::{Annotation, EafError};

use super::{AnnotationValue, AnnotationBuilder};

/// Alignable annotation. An annotation type found in a main tier,
/// with time slot references.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "ALIGNABLE_ANNOTATION")]
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

impl AlignableAnnotation {
    pub fn from_values(
        annotation_id: &str,
        time_slot_ref1: &str,
        time_slot_ref2: &str,
        annotation_value: &str
    ) -> Result<Annotation, EafError> {
        AnnotationBuilder::new()
            .annotation_id(annotation_id)
            .time_slot_ref1(time_slot_ref1)
            .time_slot_ref2(time_slot_ref2)
            .annotation_value(annotation_value)
            .build()
    }

    /// Add optional time values in millisseconds.
    /// Not part of EAF spec.
    pub fn with_time_values(self,time_value1: i64, time_value2: i64) -> Self {
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

    /// Edit time slot references (start/end of annotation).
    pub fn with_time_slot_refs(self, time_slot_ref1: &str, time_slot_ref2: &str) -> Self {
        Self {
            time_slot_ref1: time_slot_ref1.to_owned(),
            time_slot_ref2: time_slot_ref2.to_owned(),
            ..self
        }
    }

    /// Edit time slot 1 reference (start of annotation).
    pub fn with_time_slot_ref1(self, time_slot_ref1: &str) -> Self {
        Self {
            time_slot_ref1: time_slot_ref1.to_owned(),
            ..self
        }
    }

    /// Edit time slot 2 reference (end of annotation).
    pub fn with_time_slot_ref2(self, time_slot_ref2: &str) -> Self {
        Self {
            time_slot_ref2: time_slot_ref2.to_owned(),
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