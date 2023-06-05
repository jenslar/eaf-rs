use super::AnnotationValue;

// /// Methods that apply to bothe referred and aligned annotations.
// trait Generic {
//     fn annotation_id(annotation_id: &str) -> AnnotationBuilder<dyn Generic>;
//     fn ext_ref(ext_ref: &str) -> AnnotationBuilder<dyn Generic>;
//     fn lang_ref(lang_ref: &str) -> AnnotationBuilder<dyn Generic>;
//     fn cve_ref(cve_ref: &str) -> AnnotationBuilder<dyn Generic>;
//     fn annotation_value(annotation_value: &str) -> AnnotationBuilder<dyn Generic>;
//     fn tier_id(tier_id: &str) -> AnnotationBuilder<dyn Generic>;
//     fn time_value1(time_value1: i64) -> AnnotationBuilder<dyn Generic>;
//     fn time_value2(time_value2: i64) -> AnnotationBuilder<dyn Generic>;
// }

// trait Alignable {
//     fn time_slot_ref1(time_slot_ref1: &str) -> AnnotationBuilder<dyn Alignable>;
//     fn time_slot_ref2(time_slot_ref2: &str) -> AnnotationBuilder<dyn Alignable>;
// }

// trait Referred {
//     fn annotation_ref(annotation_ref: &str) -> AnnotationBuilder<dyn Referred>;
//     fn previous_annotation(previous_annotation: &str) -> AnnotationBuilder<dyn Referred>;
// }

// /// Type state that applies to both aligned and referred.
// struct Any;
// struct Alignable;
// struct Referred;

// pub struct AnnotationBuilder<T> {
#[derive(Debug, Default)]
// pub struct AnnotationBuilder<State> { // TODO add type state fore returning ref annot vs aligned, e.g. if annotation_ref is set change to <Referred>, return referred annotation
// pub struct AnnotationBuilder<Kind> { // TODO add type state fore returning ref annot vs aligned, e.g. if annotation_ref is set change to <Referred>, return referred annotation
pub struct AnnotationBuilder { // TODO add type state fore returning ref annot vs aligned, e.g. if annotation_ref is set change to <Referred>, return referred annotation
    // Both aligned and referred annotation
    /// Aligned/referred, required
    pub annotation_id: Option<String>,
    /// Aligned/referred, optional
    pub ext_ref: Option<String>,
    /// Aligned/referred, optional
    pub lang_ref: Option<String>,
    /// Aligned/referred, optional
    pub cve_ref: Option<String>,
    /// Aligned/referred, required (can be an empty string)
    pub annotation_value: Option<AnnotationValue>,
    
    // Referred annotation -> <Referred>
    /// Referred, required
    pub annotation_ref: Option<String>,
    /// Referred, required
    pub previous_annotation: Option<String>,
    
    // Aligned annotation -> <Aligned>
    /// Aligned, required
    pub time_slot_ref1: Option<String>,
    /// Aligned, required
    pub time_slot_ref2: Option<String>,
    
    // Not part of EAF format: both aligned and referred annotation.
    /// Aligned/referred. Not part of EAF spec.
    pub tier_id: Option<String>,
    /// Aligned/referred. Not part of EAF spec. For populating/editing time order.
    pub time_value1: Option<i64>,
    /// Aligned/referred. Not part of EAF spec. For populating/editing time order.
    pub time_value2: Option<i64>,
    
    /// Referred. Not part of EAF format.
    /// Main annotation ID (may or may not be parent).
    pub main_annotation: Option<String>,

    // state: PhantomData<Kind>
}

// impl <Alignable>AnnotationBuilder <Alignable>{

// }

// impl Default for AnnotationBuilder<Any> {
//     fn default() -> Self {
//         Self {
//             annotation_id: None,
//             ext_ref: None,
//             lang_ref: None,
//             cve_ref: None,
//             annotation_value: None,
//             annotation_ref: None,
//             previous_annotation: None,
//             time_slot_ref1: None,
//             time_slot_ref2: None,
//             tier_id: None,
//             time_value1: None,
//             time_value2: None,
//             main_annotation: None,
//             // state: PhantomData
//         }
//     }
// }
// See: https://rustype.github.io/notes/notes/rust-typestate-series/rust-typestate-part-1.html
// impl AnnotationBuilder<Any> {
// impl AnnotationBuilder<Any> {
// impl <Generic> AnnotationBuilder<Generic> {
impl AnnotationBuilder {
    // pub fn new(self) -> AnnotationBuilder<Any> {
    pub fn new(self) -> Self {
        Self::default()
    }

    // pub fn annotation_id(self, annotation_id: &str) -> AnnotationBuilder<Any> {
    pub fn annotation_id(self, annotation_id: &str) -> Self {
        Self {
            annotation_id: Some(annotation_id.to_owned()),
            ..self
        }
    }

    pub fn ext_ref(self, ext_ref: &str) -> Self {
        Self {
            ext_ref: Some(ext_ref.to_owned()),
            ..self
        }
    }

    pub fn lang_ref(self, lang_ref: &str) -> Self {
        Self {
            lang_ref: Some(lang_ref.to_owned()),
            ..self
        }
    }

    pub fn cve_ref(self, cve_ref: &str) -> Self {
        Self {
            cve_ref: Some(cve_ref.to_owned()),
            ..self
        }
    }

    /// Annotation value.
    pub fn annotation_value(self, annotation_value: &str) -> Self {
        Self {
            annotation_value: Some(AnnotationValue::from(annotation_value)),
            ..self
        }
    }

    /// Referred annotation only. Corresponding annotation in parent tier.
    pub fn annotation_ref(self, annotation_ref: &str) -> Self {
        Self {
            annotation_ref: Some(annotation_ref.to_owned()),
            ..self
        }
    }

    /// Referred annotation only. For specifying
    /// previous annotation in sequence for tokenized tier.
    pub fn previous_annotation(self, previous_annotation: &str) -> Self {
        Self {
            previous_annotation: Some(previous_annotation.to_owned()),
            ..self
        }
    }

    /// Aligned annotation only. Time slot reference for annotation start.
    pub fn time_slot_ref1(self, time_slot_ref1: &str) -> Self {
        Self {
            time_slot_ref1: Some(time_slot_ref1.to_owned()),
            ..self
        }
    }
    
    /// Aligned annotation only. Time slot reference for annotation end.
    pub fn time_slot_ref2(self, time_slot_ref2: &str) -> Self {
        Self {
            time_slot_ref2: Some(time_slot_ref2.to_owned()),
            ..self
        }
    }

    pub fn tier_id(self, tier_id: &str) -> Self {
        Self {
            tier_id: Some(tier_id.to_owned()),
            ..self
        }
    }

    pub fn time_value1(self, time_value1: i64) -> Self {
        Self {
            time_value1: Some(time_value1),
            ..self
        }
    }

    pub fn time_value2(self, time_value2: i64) -> Self {
        Self {
            time_value2: Some(time_value2),
            ..self
        }
    }

    pub fn main_annotation(self, main_annotation: &str) -> Self {
        Self {
            main_annotation: Some(main_annotation.to_owned()),
            ..self
        }
    }

    // !!! below should only be usable state1 and produce state2 and vice versa

    pub fn to_referred(self, annotation_ref: &str) -> Self {
        Self {
            annotation_ref: Some(annotation_ref.to_owned()),
            ..self
        }
    }

    pub fn to_aligned(self, time_slot_ref1: &str, time_slot_ref2: &str) -> Self {
        Self {
            time_slot_ref1: Some(time_slot_ref1.to_owned()),
            time_slot_ref2: Some(time_slot_ref2.to_owned()),
            ..self
        }
    }
}