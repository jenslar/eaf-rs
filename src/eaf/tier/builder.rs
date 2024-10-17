use std::marker::PhantomData;

use super::Tier;
use crate::{Annotation, eaf::{annotation::{RefAnnotation, AlignableAnnotation}, overlap}, EafError};

#[derive(Debug)]
pub struct TierMain;
#[derive(Debug)]
pub struct TierReferred;

pub trait TierState {}
impl TierState for TierMain {}
impl TierState for TierReferred {}

#[derive(Debug, Default)]
pub struct TierBuilder<S: TierState> {
    // uuid_parent: Option<String>,
    tier_id: Option<String>,
    participant: Option<String>,
    annotator: Option<String>,
    linguistic_type_ref: String,
    default_locale: Option<String>, // refers to language_code in Locale
    parent_ref: Option<String>,
    ext_ref: Option<String>,
    lang_ref: Option<String>,
    annotations: Vec<Annotation>,

    state: PhantomData<S>
}

impl <S: TierState> TierBuilder<S> {
    /// Set tier ID.
    pub fn tier_id(self, tier_id: &str) -> Self {
        Self {
            tier_id: Some(tier_id.to_owned()),
            ..self
        }
    }

    /// Add participant.
    pub fn participant(self, participant: &str) -> Self {
        Self {
            participant: Some(participant.to_owned()),
            ..self
        }
    }

    /// Add annotator.
    pub fn annotator(self, annotator: &str) -> Self {
        Self {
            annotator: Some(annotator.to_owned()),
            ..self
        }
    }

    /// Set linguistic type.
    pub fn linguistic_type_ref(self, linguistic_type_ref: &str) -> Self {
        Self {
            linguistic_type_ref: linguistic_type_ref.to_owned(),
            ..self
        }
    }

    /// Add default locale.
    pub fn default_locale(self, default_locale: &str) -> Self {
        Self {
            default_locale: Some(default_locale.to_owned()),
            ..self
        }
    }

    /// Add parent tier reference ID.
    ///
    /// Important:
    /// - All existing annotations will be converted
    /// to reference annotations, using the existing alignable
    /// annotation IDs as reference annotation IDs. If these are not valid
    /// reference IDs, the resulting ELAN-file will not be valid.
    /// Moreover, new annotation IDs must be generated or the
    /// converted annotations will refer to themselves.
    /// - Annotation IDs are often not "+1 incremental", meaning
    /// the numerical component of a ref ID must be checked against that
    /// of the parent annotation.
    /// - The number of annotations in a referred tier is not necessarily
    /// equal to the parent tier's (the referred tier may have fewer annotations
    /// than its parent, but not more).
    // pub fn parent_ref(self, parent_ref: &str, uuid_parent: &str) -> TierBuilder<TierReferred> {
    pub fn parent_ref(self, parent_ref: &str) -> TierBuilder<TierReferred> {
        TierBuilder {
            // uuid_parent: Some(uuid_parent.to_owned()),
            parent_ref: Some(parent_ref.to_owned()),
            tier_id: self.tier_id,
            participant: self.participant,
            annotator: self.annotator,
            linguistic_type_ref: self.linguistic_type_ref,
            default_locale: self.default_locale,
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            // Convert annotations to reference annotations
            annotations: self.annotations.iter()
                .map(|a| a.to_referred(&a.id(), None, None))
                .collect(),
            state: PhantomData,
        }
    }

    /// Add external reference.
    pub fn ext_ref(self, ext_ref: &str) -> Self {
        Self {
            ext_ref: Some(ext_ref.to_owned()),
            ..self
        }
    }

    /// Add language reference.
    pub fn lang_ref(self, lang_ref: &str) -> Self {
        Self {
            lang_ref: Some(lang_ref.to_owned()),
            ..self
        }
    }
}

impl TierBuilder<TierMain> {
    pub fn new() -> TierBuilder<TierMain> {
        TierBuilder {
            // uuid_parent: None,
            tier_id: None,
            participant: None,
            annotator: None,
            linguistic_type_ref: String::from("default-lt"),
            default_locale: None,
            parent_ref: None,
            ext_ref: None,
            lang_ref: None,
            annotations: Vec::new(),
            state: PhantomData
        }
    }

    /// Add annotation.
    pub fn annotation(self, annotation: &AlignableAnnotation) -> Self {
        self.annotations(&[annotation.to_owned()])
    }

    /// Add annotations.
    pub fn annotations(self, annotations: &[AlignableAnnotation]) -> Self {
        Self {
            annotations: self.annotations.into_iter()
                .chain(annotations.iter().map(|a| Annotation::from(a.to_owned())))
                .collect(),
            ..self
        }
    }

    /// Build main tier.
    ///
    /// Returns `EafError::AnnotationTypeMismatch`
    /// if any annotation is a referred annotation.
    pub fn build(self) -> Result<Tier, EafError> {
        if self.annotations.iter().any(|a| a.is_ref()) {
            return Err(EafError::AnnotationTypeMismatch)
        }

        Ok(Tier {
            // uuid: uuid::Uuid::new_v4().to_string(),
            // uuid_parent: None,
            tier_id: self.tier_id.ok_or(EafError::TierIdNotSet)?,
            participant: self.participant,
            annotator: self.annotator,
            linguistic_type_ref: self.linguistic_type_ref,
            default_locale: self.default_locale,
            parent_ref: None, // parent_ref ignored
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            annotations: self.annotations,
        })
    }
}

impl TierBuilder<TierReferred> {
    /// Add annotation.
    pub fn annotation(self, annotation: &RefAnnotation) -> Self {
        Self {
            annotations: self.annotations.into_iter()
                .chain(std::iter::once(Annotation::from(annotation.to_owned())))
                .collect(),
            ..self
        }
    }

    /// Add annotations.
    pub fn annotations(self, annotations: &[RefAnnotation]) -> Self {
        Self {
            annotations: self.annotations.into_iter()
                .chain(annotations.iter().map(|a| Annotation::from(a.to_owned())))
                .collect(),
            ..self
        }
    }

    /// Build referred tier.
    ///
    /// Returns `EafError::AnnotationTypeMismatch`
    /// if any annotation is not a referred annotation.
    pub fn build(self) -> Result<Tier, EafError> {
        // Ensure tier type matches annoation type
        if self.annotations.iter().any(|a| !a.is_ref()) {
            return Err(EafError::AnnotationTypeMismatch)
        }

        // Check for overlap (does currently not return which annotations overlap)
        if overlap(&self.annotations) {
            return Err(EafError::AnnotationOverlap)
        }

        Ok(Tier {
            // uuid: uuid::Uuid::new_v4().to_string(),
            // uuid_parent: None,
            tier_id: self.tier_id.ok_or(EafError::TierIdNotSet)?,
            participant: self.participant,
            annotator: self.annotator,
            linguistic_type_ref: self.linguistic_type_ref,
            default_locale: self.default_locale,
            parent_ref: Some(self.parent_ref.ok_or(EafError::TierRefIdNotSet)?),
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            annotations: self.annotations,
        })
    }
}
