use super::Tier;
use crate::Annotation;

#[derive(Debug, Default)]
pub struct TierBuilder {
    tier_id: String,
    participant: Option<String>,
    annotator: Option<String>,
    linguistic_type_ref: String, // more?
    default_locale: Option<String>, // refers to language_code in Locale
    parent_ref: Option<String>,
    ext_ref: Option<String>,
    lang_ref: Option<String>,
    annotations: Vec<Annotation>,
}

impl TierBuilder {
    pub fn new() -> Self {
        Self::default().tier_id("default")
    }

    /// Set tier ID.
    pub fn tier_id(self, tier_id: &str) -> Self {
        Self {
            tier_id: tier_id.to_owned(),
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

    /// Add parent tier ID.
    pub fn parent_ref(self, parent_ref: &str) -> Self {
        Self {
            parent_ref: Some(parent_ref.to_owned()),
            ..self
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

    /// Add annotations.
    pub fn annotations(self, annotations: &[Annotation]) -> Self {
        Self {
            annotations: self.annotations.into_iter()
                .chain(annotations.to_owned())
                .collect(),
            ..self
        }
    }

    /// Add annotation.
    pub fn annotation(self, annotation: &Annotation) -> Self {
        Self {
            annotations: self.annotations.into_iter()
                .chain(std::iter::once(annotation.to_owned()))
                .collect(),
            ..self
        }
    }

    /// Build tier.
    pub fn build(self) -> Tier {
        Tier {
            tier_id: self.tier_id,
            participant: self.participant,
            annotator: self.annotator,
            linguistic_type_ref: self.linguistic_type_ref,
            default_locale: self.default_locale,
            parent_ref: self.parent_ref,
            ext_ref: self.ext_ref,
            lang_ref: self.lang_ref,
            annotations: self.annotations,
        }
    }
}