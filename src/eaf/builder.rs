use std::path::{Path, PathBuf};

use crate::{
    Constraint,
    ControlledVocabulary,
    Eaf,
    EafError,
    Header,
    Index,
    Language,
    LexiconRef,
    License,
    LinguisticType,
    Locale,
    Tier,
    TimeOrder,
};

use super::{
    eaf,
    ts_duplicates,
    overlap,
};

#[derive(Debug, Default)]
pub struct EafBuilder {
    // path: Option<PathBuf>,
    xmlns_xsi: Option<String>, // required
    xsi_nonamespaceschemalocation: Option<String>, // required
    author: Option<String>, // required, can be ""
    date: Option<String>,  // required? YYYY-MM-DD
    format: Option<String>, // required
    version: Option<String>, // required
    license: Option<License>,
    header: Option<Header>, // required
    time_order: Option<TimeOrder>, // required
    tiers: Vec<Tier>, // requires corresponding time order if annotations
    linguistic_types: Vec<LinguisticType>, // corresponding lt required by tier
    locales: Vec<Locale>,
    languages: Vec<Language>,
    constraints: Vec<Constraint>, // corresponding constr required by lt
    controlled_vocabularies: Vec<ControlledVocabulary>,
    lexicon_refs: Vec<LexiconRef>,
}

impl EafBuilder {
    /// Create new `EafBuilder`.
    pub fn new() -> Self {
        Self {
            xmlns_xsi: Some("http://www.w3.org/2001/XMLSchema-instance".to_owned()),
            xsi_nonamespaceschemalocation: Some("http://www.mpi.nl/tools/elan/EAFv3.0.xsd".to_owned()),
            ..Self::default()
        }
    }

    // /// Add path. Not part of EAF specification.
    // pub fn path(self, path: &Path) -> Self {
    //     Self {
    //         path: Some(path.to_owned()),
    //         ..self
    //     }
    // }

    /// Add author.
    pub fn author(self, author: impl Into<String>) -> Self {
        Self {
            author: Some(author.into()),
            ..self
        }
    }

    /// Add date.
    pub fn date(self, date: impl Into<String>) -> Self {
        Self {
            // should be time::OffsetDateTime?
            date: Some(date.into()),
            ..self
        }
    }
    
    /// Add format.
    pub fn format(self, format: impl Into<String>) -> Self {
        Self {
            format: Some(format.into()),
            ..self
        }
    }

    /// Add version.
    pub fn version(self, version: impl Into<String>) -> Self {
        Self {
            version: Some(version.into()),
            ..self
        }
    }

    /// Add license.
    pub fn license(self, license: impl Into<String>) -> Self {
        Self {
            license: Some(License::from(license.into())),
            ..self
        }
    }

    /// Add header.
    pub fn header(self, header: Header) -> Self {
        Self {
            header: Some(header),
            ..self
        }
    }

    /// Add time order.
    pub fn time_order(self, time_order: TimeOrder) -> Self {
        Self {
            time_order: Some(time_order),
            ..self
        }
    }

    /// Add tier.
    pub fn tier(self, tier: Tier) -> Self {
        self.tiers(vec![tier])
    }

    /// Add tiers.
    pub fn tiers(self, tiers: Vec<Tier>) -> Self {
        Self {
            tiers: self.tiers.into_iter()
                .chain(tiers.into_iter())
                .collect(),
            ..self
        }
    }

    /// Add linguistic types.
    pub fn linguistic_types(self, linguistic_types: Vec<LinguisticType>) -> Self {
        Self {
            linguistic_types,
            ..self
        }
    }

    /// Add locales.
    pub fn locales(self, locales: Vec<Locale>) -> Self {
        Self {
            locales,
            ..self
        }
    }

    /// Add languages.
    pub fn languages(self, languages: Vec<Language>) -> Self {
        Self {
            languages,
            ..self
        }
    }

    /// Add constraints.
    pub fn constraints(self, constraints: Vec<Constraint>) -> Self {
        Self {
            constraints,
            ..self
        }
    }

    /// Add controlled vocabularies.
    pub fn controlled_vocabularies(self, controlled_vocabularies: Vec<ControlledVocabulary>) -> Self {
        Self {
            controlled_vocabularies,
            ..self
        }
    }

    /// Add lexicon refs.
    pub fn lexicon_refs(self, lexicon_refs: Vec<LexiconRef>) -> Self {
        Self {
            lexicon_refs,
            ..self
        }
    }

    fn verify(&self,
        annotation_overlap: bool,
        timeslot_id_duplicates: bool,
    ) -> Result<(), EafError>  {
        if !self.tiers.is_empty() && self.time_order.is_none() {
            return Err(EafError::TimeOrderMissing)
        }

        if annotation_overlap {
            if self.tiers.iter().any(|t| overlap(&t.annotations)) {
                return Err(EafError::AnnotationOverlap)
            }
        }

        if timeslot_id_duplicates {
            if let Some(to) = &self.time_order {
                if ts_duplicates(&to.time_slots) {
                    return Err(EafError::TimeslotIDDuplicated)
                }
            }
        }

        Ok(())
    }

    pub fn build(self) -> Result<Eaf, EafError> {
        self.verify(true, true)?;

        Ok(Eaf {
            // path: self.path,
            xmlns_xsi: self.xmlns_xsi.ok_or_else(|| EafError::XmlNameSpaceMissing)?,
            xsi_nonamespaceschemalocation: self.xsi_nonamespaceschemalocation
                .ok_or_else(|| EafError::XmlNoNameSpaceMissing)?,
            author: self.author.unwrap_or_default(),
            date: self.date.unwrap_or(eaf::today()),
            format: self.format.unwrap_or("3.0".to_owned()),
            version: self.version.unwrap_or("3.0".to_owned()),
            license: self.license,
            header: self.header.ok_or_else(|| EafError::HeaderMissing)?,
            time_order: self.time_order.unwrap_or_default(),
            tiers: self.tiers,
            linguistic_types: if self.linguistic_types.is_empty() {
                vec![LinguisticType::default()]
            } else {
                self.linguistic_types
            },
            locales: self.locales,
            languages: self.languages,
            constraints: if self.constraints.is_empty() {
                Constraint::defaults()
            } else {
                self.constraints
            },
            controlled_vocabularies: self.controlled_vocabularies,
            lexicon_refs: self.lexicon_refs,
            derived: false,
            index: Index::default(),
            indexed: false,
        })
    }
}