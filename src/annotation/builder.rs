use std::marker::PhantomData;

use crate::{
    Annotation,
    EafError
};
use super::{
    AnnotationValue,
    AlignableAnnotation,
    RefAnnotation,
    AnnotationType
};

// TODO make state structs sealed?
#[derive(Debug)]
pub struct AnnotationAlignable;
#[derive(Debug)]
pub struct AnnotationReferred;

pub trait AnnotationState {}
impl AnnotationState for AnnotationAlignable {}
impl AnnotationState for AnnotationReferred {}

#[derive(Debug)]
pub struct AnnotationBuilder<S: AnnotationState> {
    // Values that apply to both aligned and referred annotations

    /// Applicable to: Aligned/referred. Required.
    annotation_id: Option<String>,
    /// Applicable to: Aligned/referred. Optional.
    ext_ref: Option<String>,
    /// Applicable to: Aligned/referred. Optional.
    lang_ref: Option<String>,
    /// Applicable to: Aligned/referred. Optional.
    cve_ref: Option<String>,
    /// Applicable to: Aligned/referred.
    /// Required (can be an empty string).
    annotation_value: Option<AnnotationValue>,
    
    // Values that apply to referred annotations -> <S: Referred>

    /// Applicable to: Referred. Required.
    annotation_ref: Option<String>,
    /// Applicable to: Referred. Required.
    previous_annotation: Option<String>,
    
    // Values that apply to aligned annotations -> <S: Aligned>

    /// Applicable to: Aligned. Required.
    time_slot_ref1: Option<String>,
    /// Applicable to: Aligned. Required.
    time_slot_ref2: Option<String>,
    
    // Values that are not part of the EAF format.
    // Applicable to both aligned and referred annotations.

    /// ID for the tier this annotation belongs to.
    /// Applicable to: Aligned/referred.
    /// Not part of EAF specification.
    tier_id: Option<String>,
    /// Annotation start time in milliseconds.
    /// For populating/editing time order.
    /// Applicable to: Aligned/referred.
    /// Not part of EAF specification.
    time_value1: Option<i64>,
    /// Annotation end time in milliseconds.
    /// For populating/editing time order.
    /// Applicable to: Aligned/referred.
    /// Not part of EAF specification.
    time_value2: Option<i64>,
    
    /// Applicable to: Referred.
    /// Not part of EAF specification.
    /// Main annotation ID (i.e. corresponding annotation
    /// in top-level tier, may or may not be parent).
    main_annotation: Option<String>,

    // state: PhantomData<Kind>
    state: PhantomData<S>
}

// impl AnnotationBuilder<Generic> {
impl <S: AnnotationState> AnnotationBuilder<S>
    where
        AnnotationBuilder<AnnotationAlignable>: From<AnnotationBuilder<S>>,
        AnnotationBuilder<AnnotationReferred>: From<AnnotationBuilder<S>>
{
    /// Set annotation ID.
    pub fn annotation_id(self, annotation_id: impl Into<String>) -> Self {
        Self {
            annotation_id: Some(annotation_id.into()),
            ..self
        }
    }

    /// Set external reference.
    pub fn ext_ref(self, ext_ref: impl Into<String>) -> Self {
        Self {
            ext_ref: Some(ext_ref.into()),
            ..self
        }
    }

    /// Set language reference.
    pub fn lang_ref(self, lang_ref: impl Into<String>) -> Self {
        Self {
            lang_ref: Some(lang_ref.into()),
            ..self
        }
    }

    /// Set CVE reference.
    pub fn cve_ref(self, cve_ref: impl Into<String>) -> Self {
        Self {
            cve_ref: Some(cve_ref.into()),
            ..self
        }
    }

    /// Set annotation value.
    pub fn annotation_value(self, annotation_value: impl Into<String>) -> Self {
        Self {
            annotation_value: Some(AnnotationValue::from(annotation_value.into())),
            ..self
        }
    }

    /// Set tier ID. Aligned/referred annotation.
    /// Not part of EAF specification.
    pub fn tier_id(self, tier_id: impl Into<String>) -> Self {
        Self {
            tier_id: Some(tier_id.into()),
            ..self
        }
    }

    /// Set time value for start of annotation.
    /// Not part of EAF specification.
    pub fn time_start(self, start_ms: i64) -> Self {
        Self {
            time_value1: Some(start_ms),
            ..self
        }
    }
    
    /// Set time value for end of annotation.
    /// Not part of EAF specification.
    pub fn time_end(self, end_ms: i64) -> Self {
        Self {
            time_value2: Some(end_ms),
            ..self
        }
    }

    /// Set time values for start/end of annotation.
    /// Not part of EAF specification.
    pub fn time_values(self, time_value1: i64, time_value2: i64) -> Self {
        Self {
            time_value1: Some(time_value1),
            time_value2: Some(time_value2),
            ..self
        }
    }

    /// Set reference for corresponding annotation in parent tier.
    pub fn annotation_ref(
        self,
        annotation_ref: impl Into<String>
    ) -> AnnotationBuilder<AnnotationReferred> {
        AnnotationBuilder::<AnnotationReferred>::from(Self {
            annotation_ref: Some(annotation_ref.into()),
            ..self
        })
    }

    /// Set previous annotation in sequence for tokenized tier.
    pub fn previous_annotation(
        self,
        previous_annotation: impl Into<String>
    ) -> AnnotationBuilder<AnnotationReferred> {
        AnnotationBuilder::<AnnotationReferred>::from(Self {
            previous_annotation: Some(previous_annotation.into()),
            ..self
        })
    }

    /// Set time slot reference for annotation start.
    pub fn time_slot_ref1(
        self,
        time_slot_ref1: impl Into<String>
    ) -> AnnotationBuilder<AnnotationAlignable> {
        AnnotationBuilder::<AnnotationAlignable>::from(Self {
            time_slot_ref1: Some(time_slot_ref1.into()),
            ..self
        })
    }

    /// Set time slot reference for annotation end.
    pub fn time_slot_ref2(
        self,
        time_slot_ref2: impl Into<String>
    ) -> AnnotationBuilder<AnnotationAlignable> {
        AnnotationBuilder::<AnnotationAlignable>::from(Self {
            time_slot_ref2: Some(time_slot_ref2.into()),
            ..self
        })
    }
    
    /// Set time slot references for annotation start/end.
    pub fn time_slot_refs(
        self,
        time_slot_ref1: impl Into<String>,
        time_slot_ref2: impl Into<String>
    ) -> AnnotationBuilder<AnnotationAlignable> {
        AnnotationBuilder::<AnnotationAlignable>::from(Self {
            time_slot_ref1: Some(time_slot_ref1.into()),
            time_slot_ref2: Some(time_slot_ref2.into()),
            ..self
        })
    }

    /// Set main annotation.
    /// Not part of EAF specification.
    pub fn main_annotation(
        self,
        main_annotation: impl Into<String>
    ) -> AnnotationBuilder<AnnotationReferred> {
        AnnotationBuilder::<AnnotationReferred>::from(Self {
            main_annotation: Some(main_annotation.into()),
            ..self
        })
    }
}

impl AnnotationBuilder<AnnotationAlignable> {
    pub fn new() -> Self {
        Self {
            annotation_id: None,
            ext_ref: None,
            lang_ref: None,
            cve_ref: None,
            annotation_value: None,
            annotation_ref: None,
            previous_annotation: None,
            time_slot_ref1: None,
            time_slot_ref2: None,
            tier_id: None,
            time_value1: None,
            time_value2: None,
            main_annotation: None,
            state: PhantomData
        }
    }

    /// Convert to referred annotation builder.
    /// 
    /// Requires setting referred annotation ID to
    /// generate a valid annotation.
    pub fn to_referred(self) -> AnnotationBuilder<AnnotationReferred> {
        AnnotationBuilder::<AnnotationReferred>::from(self)
    }

    /// Build alignable annotation.
    pub fn build_aligned(self) -> Result<AlignableAnnotation, EafError> {
        Ok(AlignableAnnotation {
            annotation_id: self.annotation_id.ok_or_else(|| EafError::AnnotationIdNotSet)?,
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            cve_ref: self.cve_ref,
            annotation_value: self.annotation_value.unwrap_or_default(),
            tier_id: self.tier_id,
            time_value1: self.time_value1,
            time_value2: self.time_value2,
            time_slot_ref1: self.time_slot_ref1.ok_or_else(|| EafError::TimeslotRef1Missing)?,
            time_slot_ref2: self.time_slot_ref2.ok_or_else(|| EafError::TimeslotRef2Missing)?,
        })
    }

    /// Build annotation.
    pub fn build(self) -> Result<Annotation, EafError> {
        let alig_annot = self.build_aligned()?;
        Ok(Annotation::new(&AnnotationType::AlignableAnnotation(alig_annot)))
    }
}

impl AnnotationBuilder<AnnotationReferred> {
    /// Convert to referred annotation builder.
    /// 
    /// Requires setting time slot reference 1 and 2 to
    /// generate a valid annotation.
    pub fn to_aligned(self) -> AnnotationBuilder<AnnotationAlignable> {
        AnnotationBuilder::<AnnotationAlignable>::from(self)
    }

    /// Build referred annotation.
    pub fn build_referred(self) -> Result<RefAnnotation, EafError> {
        Ok(RefAnnotation {
            annotation_id: self.annotation_id.ok_or_else(|| EafError::AnnotationIdNotSet)?,
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            cve_ref: self.cve_ref,
            annotation_ref: self.annotation_ref.ok_or_else(|| EafError::AnnotationRefMissing)?,
            previous_annotation: self.previous_annotation,
            annotation_value: self.annotation_value.unwrap_or_default(),
            tier_id: self.tier_id,
            time_value1: self.time_value1,
            time_value2: self.time_value2,
            main_annotation: self.main_annotation,
        })
    }

    /// Build annotation.
    pub fn build(self) -> Result<Annotation, EafError> {
        let ref_annot = self.build_referred()?;
        Ok(Annotation::new(&super::AnnotationType::RefAnnotation(ref_annot)))
    }
}

impl From<AnnotationBuilder<AnnotationAlignable>> for AnnotationBuilder<AnnotationReferred> {
    fn from(value: AnnotationBuilder<AnnotationAlignable>) -> Self {
        Self {
            annotation_id: value.annotation_id,
            ext_ref: value.ext_ref,
            lang_ref: value.lang_ref,
            cve_ref: value.cve_ref,
            annotation_value: value.annotation_value,
            annotation_ref: value.annotation_ref,
            previous_annotation: value.previous_annotation,
            time_slot_ref1: value.time_slot_ref1,
            time_slot_ref2: value.time_slot_ref2,
            tier_id: value.tier_id,
            time_value1: value.time_value1,
            time_value2: value.time_value2,
            main_annotation: value.main_annotation,
            state: PhantomData,
        }
    }
}

impl From<AnnotationBuilder<AnnotationReferred>> for AnnotationBuilder<AnnotationAlignable> {
    fn from(value: AnnotationBuilder<AnnotationReferred>) -> Self {
        Self {
            annotation_id: value.annotation_id,
            ext_ref: value.ext_ref,
            lang_ref: value.lang_ref,
            cve_ref: value.cve_ref,
            annotation_value: value.annotation_value,
            annotation_ref: value.annotation_ref,
            previous_annotation: value.previous_annotation,
            time_slot_ref1: value.time_slot_ref1,
            time_slot_ref2: value.time_slot_ref2,
            tier_id: value.tier_id,
            time_value1: value.time_value1,
            time_value2: value.time_value2,
            main_annotation: value.main_annotation,
            state: PhantomData,
        }
    }
}
