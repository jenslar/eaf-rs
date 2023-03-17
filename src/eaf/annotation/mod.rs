//! EAF Annotation.
//! 
//! Can be either an 'aligned annotation' (part of a main/top-level tier),
//! or a 'referred annotation' (part of a referred tier).

mod annotation;
mod alignable_annotation;
mod ref_annotation;
mod annotation_type;
mod annotation_value;

pub use annotation::Annotation;
pub use alignable_annotation::AlignableAnnotation;
pub use ref_annotation::RefAnnotation;
pub use annotation_type::AnnotationType;
pub use annotation_value::AnnotationValue;