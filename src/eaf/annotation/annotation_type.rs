use serde::{Deserialize, Serialize};

use super::{AlignableAnnotation, RefAnnotation};

/// `AnnotationType` refers to either an `AlignableAnnotation`
/// or a `RefAnnotation`. For internal use to deserialize
/// and serialize the document correctly. Every annotation in the EAF
/// must one of these annotation types.
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