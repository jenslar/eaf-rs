//! Annotation type.
//! 
//! ELAN annotations can be either an alignable annotation (part of a main tier),
//! or a referred annotation (part of a referred tier).
//! 
//! Aligned annotations contain references to time slots (the annotation's time span),
//! whereas referred annotations reference an annotation in the parent tier.
//! 
//! For internal use to deserialize and serialize the document correctly.

use serde::{Deserialize, Serialize};

use super::{AlignableAnnotation, RefAnnotation};

/// Annotation type. Refers to either an `AlignableAnnotation`
/// or a `RefAnnotation`.
/// 
/// For internal use to deserialize and serialize the document correctly.
/// An annotation is either "alignable" (part of a main tier),
/// or "referred" (part of a referred tier).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
pub enum AnnotationType {
    #[serde(rename = "ALIGNABLE_ANNOTATION")] // required, but causes double annotation tags
    AlignableAnnotation(AlignableAnnotation),
    #[serde(rename = "REF_ANNOTATION")] // required, but causes double annotation tags
    RefAnnotation(RefAnnotation)
}

impl Default for AnnotationType {
    fn default() -> Self {
        Self::AlignableAnnotation(AlignableAnnotation::default())
    }
}