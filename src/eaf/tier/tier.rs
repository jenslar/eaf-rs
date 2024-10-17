//! Tier.
//! 
//! ELAN tiers can be either a main tier,
//! or a referred tier (refers to a parent tier).
//! 
//! Note that a referred tier may refer to another
//! referred tier.

use std::{collections::{HashMap, HashSet}, io::copy};

use regex::Regex;
use serde::{Deserialize, Serialize};
use rayon::{prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator}, slice::ParallelSliceMut};

use crate::{Annotation, EafError, TimeSlot};

use super::{TierBuilder, builder::TierMain};

// pub(crate) TierAttributes {
//     #[serde(rename="@TIER_ID")]
//     pub tier_id: String,
//     #[serde(rename="@PARTICIPANT")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub participant: Option<String>,
//     #[serde(rename="@ANNOTATOR")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub annotator: Option<String>,
//     #[serde(rename="@LINGUISTIC_TYPE_REF")]
//     pub linguistic_type_ref: String, // more?
//     #[serde(rename="@DEFAULT_LOCALE")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub default_locale: Option<String>, // refers to language_code in Locale
//     #[serde(rename="@PARENT_REF")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub parent_ref: Option<String>,
//     #[serde(rename="@EXT_REF")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub ext_ref: Option<String>,
//     #[serde(rename="@LANG_REF")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub lang_ref: Option<String>,
// }

/// EAF tier.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Tier {
    // /// UUID v4 for this tier used for internal identification.
    // #[serde(skip)]
    // pub(crate) uuid: String,
    // /// UUID v4 for parent tier used for internal identification.
    // pub(crate) uuid_parent: Option<String>,

    // Attributes
    
    #[serde(rename="@TIER_ID")]
    pub tier_id: String,
    #[serde(rename="@PARTICIPANT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<String>,
    #[serde(rename="@ANNOTATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotator: Option<String>,
    #[serde(rename="@LINGUISTIC_TYPE_REF")]
    pub linguistic_type_ref: String, // more?
    #[serde(rename="@DEFAULT_LOCALE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>, // refers to language_code in Locale
    #[serde(rename="@PARENT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<String>,
    #[serde(rename="@EXT_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_ref: Option<String>,
    #[serde(rename="@LANG_REF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_ref: Option<String>,

    // Child nodes

    #[serde(rename = "ANNOTATION", default)] // default required...?
    // #[serde(rename(serialize = "$value"))]
    pub annotations: Vec<Annotation>,
}

impl Default for Tier {
    fn default() -> Self {
        Self {
            // uuid: uuid::Uuid::new_v4().to_string(),
            // uuid_parent: None,
            tier_id: "default".to_owned(),
            participant: None,
            annotator: None,
            linguistic_type_ref: "default-lt".to_owned(), // more?
            default_locale: None,
            parent_ref: None,
            ext_ref: None,
            lang_ref: None,
            annotations: Vec::new(),
        }
    }
}

impl Tier {
    /// Create new tier.
    pub fn new(
        tier_id: &str,
        annotations: Option<&[Annotation]>,
        linguistic_type_ref: Option<&str>,
        parent_ref: Option<&str>,
    ) -> Self {
        let mut tier = Self::default();
        tier.tier_id = tier_id.to_owned();
        if let Some(a) = annotations {
            tier.annotations = a.to_owned()
        }
        if let Some(l) = linguistic_type_ref {
            tier.linguistic_type_ref = l.to_owned()
        }
        if let Some(p) = parent_ref {
            tier.parent_ref = Some(p.to_owned())
        }

        tier
    }

    /// Create a new tier builder.
    pub fn builder() -> TierBuilder<TierMain> {
        TierBuilder::new()
    }

    /// Returns the tier with specified tier ID.
    pub fn with_id(self, tier_id: &str) -> Self {
        Self{
            tier_id: tier_id.to_owned(),
            ..self
        }
    }

    pub fn prefix_id(self, prefix: &str) -> Self {
        let tier_id = format!("{}{}", prefix, self.tier_id);
        self.with_id(&tier_id)
    }

    /// Returns the tier with specified parent tier reference ID.
    /// I.e. the tier effectively becomes a referred tier.
    /// 
    /// Note that annotation type remains unchanged.
    pub fn with_parent_ref(self, parent_ref: &str) -> Self {
        Self{
            parent_ref: Some(parent_ref.to_owned()),
            ..self
        }
    }

    /// Returns the tier with `parent ref` tier ID prefixed.
    /// 
    /// Note that annotation type remains unchanged.
    pub fn prefix_parent_ref(self, prefix: &str) -> Self {
        let parent_ref = format!("{}{}", prefix, self.tier_id);
        self.with_parent_ref(&parent_ref)
    }

    /// Returns the tier with specified linguistic type.
    pub fn with_linguistic_type_ref(self, linguistic_type_ref: &str) -> Self {
        Self{
            linguistic_type_ref: linguistic_type_ref.to_owned(),
            ..self
        }
    }

    /// Returns the tier with annotations stripped,
    /// but with tier ID and other attributes intact.
    /// 
    /// Mainly for generating ETF-files.
    pub fn strip(&self) -> Self {
        Self {
            annotations: Vec::default(),
            ..self.to_owned()
        }
    }

    /// Generates a main tier from
    /// a list of tuples in the form `(annotation_value, start_time_ms, end_time_ms)`,
    /// assumed to be in chronological order.
    /// 
    /// `start_index` deafult to 1 if not set, and will increment by one
    /// (this is not necessarily true for an existing ELAN file).
    pub fn main_from_values(
        values: &[(String, i64, i64)],
        tier_id: &str,
        annotation_start_index: Option<usize>,
    ) -> Result<Tier, EafError> {
        let idx = annotation_start_index.unwrap_or(1);
        Tier::builder()
            .tier_id(tier_id)
            .annotations(
                &values.par_iter().enumerate()
                    .map(|(i, (val, t1, t2))|
                        Annotation::builder()
                            .annotation_id(&format!("a{}", i+idx))
                            .annotation_value(val)
                            .time_slot_refs(
                                // !!! is below correct?
                                &format!("ts{}", (i+idx) * 2 - 1),
                                &format!("ts{}", (i+idx) * 2)
                                // !!! should be:
                                // &format!("ts{}", idx + i*2),
                                // &format!("ts{}", idx + i*2 + 1)
                            )
                            .time_values(*t1, *t2)
                            .build_aligned()
                    ).collect::<Result<Vec<_>, EafError>>()?
            ).build()
    }

    /// Generate a referred tier from annotation values.
    /// 
    /// Important:
    /// - If `start_index` is `None`, an attempt will be made to use
    /// the first annotation ID that follows immediately after the last
    /// one in the parent tier, but no check can be made to ensure
    /// this ID is not already in use.
    /// - In this case the parent tier (`parent`),
    /// must have the same number of annotations.
    /// Otherwise it is not possible to determine which
    /// the "parent" annotation is.
    pub fn ref_from_values(
        values: &[String],
        tier_id: &str,
        parent: &Tier,
        linguistic_type_ref: &str,
        annotation_start_index: Option<usize>,
    ) -> Result<Tier, EafError> {
        // While a referred tier does not necessarily
        // have the same number of annotations as its parent tier
        // (it may have fewer, but not more),
        // in this case it has to, since otherwise
        // we do not know which ref annotation goes with what parent annotation.
        // If the parent tier is empty there is nothing to refer to
        if values.len() != parent.len() {
            return Err(EafError::TierAlignmentError((parent.tier_id.to_owned(), tier_id.to_owned())))
        }

        let mut ref_tier_builder = Tier::builder()
            .tier_id(tier_id)
            // .parent_ref(&parent.tier_id, &parent.uuid)
            .parent_ref(&parent.tier_id)
            .linguistic_type_ref(linguistic_type_ref);

        // If the parent tier contains no annotations there is nothing
        // to refer to, thus no annotations can exist in the referred tier.
        if parent.is_empty() {
            return Err(EafError::TierRefEmptyParent)
        } else {
            let first_idx: usize = if let Some(num) = annotation_start_index {
                num
            } else {
                // parent.max_a_id_num()?
                parent.max_id_num().ok_or(EafError::AnnotationIdMissing)? as usize
            };

            let mut id_count: usize = 0;
            let ref_annotations = values.iter().enumerate().map(|(i, val)| {
                // assume the same number of annotations in parent tier
                // and referred tier, get the annotation at index i,
                // then retrieve its ID
                if let Some(parent_a) = parent.annotations.get(i) {
                    let a = Annotation::builder()
                        .annotation_value(val)
                        .annotation_id(&format!("a{}", first_idx + id_count))
                        .annotation_ref(parent_a.id())
                        .build_referred();
                    id_count += 1;
                    a
                } else {
                    Err(EafError::AnnotationIdMissing)
                }
            }).collect::<Result<Vec<_>, EafError>>()?;

            ref_tier_builder = ref_tier_builder.annotations(&ref_annotations)
        }

        ref_tier_builder.build()
    }

    /// Returns `true` if the tier is a main tier,
    /// i.e. a top-level tier.
    pub fn is_main(&self) -> bool {
        self.parent_ref.is_none()
    }

    /// Returns `true` if the tier is a referred tier,
    /// i.e. it has a parent tier.
    pub fn is_ref(&self) -> bool {
        self.parent_ref.is_some()
    }

    /// Returns `true` if the tier is tokenized.
    pub fn is_tokenized(&self) -> bool {
        self.iter().any(|a| a.previous().is_some())
    }

    /// Returns number of annotations in the tier.
    pub fn len(&self) -> usize {
        self.annotations.len()
    }

    /// Returns `true` if the tier contains no annotations.
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }

    /// Returns average annotation length,
    /// i.e. average number of tokens/words
    /// in each annotation.
    pub fn avr_annot_len(&self) -> f64 {
        let a_len: Vec<usize> = self.annotations.iter()
            .map(|a| a.len())
            .collect();
        match a_len.len() {
            0 => 0.,
            _ => a_len.iter().sum::<usize>() as f64 / a_len.len() as f64
        }
    }

    /// Returns average token/word length,
    /// i.e. average number of characters
    /// in each token/word. Based on extended
    /// grapheme clusters.
    pub fn avr_token_len(&self) -> f64 {
        let t_len: Vec<f64> = self.annotations.iter()
            .map(|a| a.avr_len(true)) // average token length for annotation
            .collect();
        match t_len.len() {
            0 => 0.,
            _ => t_len.iter().sum::<f64>() / t_len.len() as f64
        }
    }

    /// Returns a reference to the first annotation,
    /// if the tier is not empty.
    pub fn first(&self) -> Option<&Annotation> {
        self.annotations.first()
    }

    /// Returns a mutable reference to the first annotation,
    /// if the tier is not empty.
    pub fn first_mut(&mut self) -> Option<&mut Annotation> {
        self.annotations.first_mut()
    }

    /// Returns a reference to the last annotation,
    /// if the tier is not empty.
    pub fn last(&self) -> Option<&Annotation> {
        self.annotations.last()
    }

    /// Returns a mutable reference to the last annotation,
    /// if the tier is not empty.
    pub fn last_mut(&mut self) -> Option<&mut Annotation> {
        self.annotations.last_mut()
    }

    /// Returns a reference to the annotation with specified ID.
    pub fn find(&self, annotation_id: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.id() == annotation_id)
    }

    /// Returns the Annotation with smallest millisecond value,
    /// or `None` if the tier is empty or if annotatations
    /// contain no time values.
    pub fn min(&self) -> Option<&Annotation> {
        self.iter()
            .min_by_key(|a| a.ts_val()) // ts_val returns (Option<i64>, Option<i64>) which works with cmp
    }

    /// Returns smallest millisecond annotation time span
    /// as a tuple,
    /// or `None` if the tier is empty or if annotatations
    /// contain no time values.
    pub fn min_span(&self) -> Option<(i64, i64)> {
        self.iter()
            .filter_map(|a| if let (Some(s), Some(e)) = a.ts_val() {
                Some((s, e))
            } else {
                None
            })
            .min()
    }

    /// Returns smallest numerical component in annotation ID.
    /// E.g. "1" will be returned if
    /// the existing IDs are "a1", "a2", "a3".
    pub fn min_id_num(&self) -> Option<i64> {
        self.iter()
            .filter_map(|a|
                a.id()
                    .trim_start_matches("a")
                    .parse::<i64>().ok()
                )
            .min()
    }

    /// Returns "minimum" annotation ID,
    /// e.g. "ts1" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn min_id(&self) -> Option<String> {
        Some(format!("a{}", self.min_id_num()?))
    }

    /// Returns the Annotation with smallest millisecond value,
    /// or `None` if the tier is empty or if annotatations
    /// contain no time values.
    pub fn max(&self) -> Option<&Annotation> {
        self.iter()
            .max_by_key(|a| a.ts_val()) // ts_val returns (Option<i64>, Option<i64>) which works with cmp
    }

    /// Returns smallest millisecond annotation time span
    /// as a tuple,
    /// or `None` if the tier is empty or if annotatations
    /// contain no time values.
    pub fn max_span(&self) -> Option<(i64, i64)> {
        self.iter()
            .filter_map(|a| if let (Some(s), Some(e)) = a.ts_val() {
                Some((s, e))
            } else {
                None
            })
            .max()
    }

    /// Returns smallest numerical component in time slot ID.
    /// E.g. "1" will be returned if
    /// the existing IDs are "a1", "a2", "a3".
    pub fn max_id_num(&self) -> Option<i64> {
        self.iter()
            .filter_map(|a|
                a.id()
                    .trim_start_matches("a")
                    .parse::<i64>().ok()
                )
            .max()
    }

    /// Returns "minimum" time slot ID,
    /// e.g. "ts1" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn max_id(&self) -> Option<String> {
        Some(format!("a{}", self.max_id_num()?))
    }

    /// Matches annotation values against a pattern.
    /// 
    /// Returns tuples in the form
    /// `(Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID)`.
    /// where index corresponds to annotation order in the EAF-file.
    pub fn query(
        &self,
        pattern: &str,
        ignore_case: bool
    ) -> Vec<(usize, &str, &str, &str, Option<&str>)> {
        // self.iter()
        self.annotations.par_iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let org_val = a.to_str().to_owned();
                let (val, ptn) = match ignore_case {
                    true => (org_val.to_lowercase(), pattern.to_lowercase()),
                    false => (org_val.to_owned(), pattern.to_owned()),
                };
                if val.contains(&ptn) {
                    Some((i + 1, self.tier_id.as_str(), a.id(), a.to_str(), a.ref_id()))
                } else {
                    None
                }
            })
            .collect()
    }
    // pub fn query_old(
    //     &self,
    //     pattern: &str,
    //     ignore_case: bool
    // ) -> Vec<(usize, String, String, String, Option<String>)> {
    //     self.iter()
    //     // self.annotations.par_iter()
    //         .enumerate()
    //         .filter_map(|(i, a)| {
    //             let org_val = a.to_str().to_owned();
    //             let (val, ptn) = match ignore_case {
    //                 true => (org_val.to_lowercase(), pattern.to_lowercase()),
    //                 false => (org_val.to_owned(), pattern.to_owned()),
    //             };
    //             if val.contains(&ptn) {
    //                 Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }
    
    /// Match annotation values against a regular expression.
    /// 
    /// Returns tuples in the form
    /// `(Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID)`.
    /// where index corresponds to annotation order in the EAF-file.
    pub fn query_rx(
        &self,
        regex: &Regex
    ) -> Vec<(usize, &str, &str, &str, Option<&str>)> {
        // self.iter()
        self.annotations.par_iter()
            .enumerate()
            .filter_map(|(i, a)| {
                // let org_val = a.to_str().to_owned();
                if regex.is_match(&a.value().as_ref()) {
                    Some((i + 1, self.tier_id.as_str(), a.id(), a.value().as_ref(), a.ref_id()))
                } else {
                    None
                }
            })
            .collect()
    }
    // pub fn query_rx_old(
    //     &self,
    //     regex: &Regex
    // ) -> Vec<(usize, String, String, String, Option<String>)> {
    //     self.iter()
    //     // self.annotations.par_iter()
    //         .enumerate()
    //         .filter_map(|(i, a)| {
    //             let org_val = a.to_str().to_owned();
    //             if regex.is_match(&org_val) {
    //                 Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts which uses whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// 
    /// - `tier = true` compiles ngrams across annotation boundaries
    /// - `tier = false` compiles per annotation and combines the result
    /// 
    /// Returns `HashMap<ngram_as_string, count>`.
    pub fn ngram(&self, size: usize, regex_remove: Option<&Regex>, tier: bool) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();

        if tier {
            // ngrams per tier
            let tokens = self.annotations.iter()
                .flat_map(|a| a.tokens()
                    .iter()
                    .map(|v| {
                        if let Some(rx) = regex_remove {
                            rx.replace_all(&v.to_lowercase(), "").to_string()
                        } else {
                            v.to_lowercase()
                        }
                    }).collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();
            for value in tokens.windows(size) {
                *ngrams.entry(value.join(" ")).or_insert(0) += 1;
            }
        } else {
            // ngrams per annotation
            self.iter()
                .for_each(|a| ngrams.extend(a.ngram(size, regex_remove)));
        }
        
        ngrams
    }

    /// Returns all words/tokens in all annotation values in the tier.
    /// 
    /// Splits on whitespace, meaning CJK will not work with this method.
    /// 
    /// Optionally specify affixes to strip, only returning unique words,
    /// and/or ignoring case.
    pub fn tokens(
        &self,
        strip_prefix: Option<&str>,
        strip_suffix: Option<&str>,
        unique: bool,
        ignore_case: bool,
    ) -> Vec<String> {
        let prefix: Vec<char> = strip_prefix
            .map(|s| s.chars().collect())
            .unwrap_or(Vec::new());
        let suffix: Vec<char> = strip_suffix
            .map(|s| s.chars().collect())
            .unwrap_or(Vec::new());

        let mut tokens: Vec<String> = self.annotations
            .par_iter() // possibly slower for tiers with few annotations
            .map(|a| {
                a.tokens()
                    .iter()
                    .map(|str_slice| {
                        let string = str_slice
                            .trim_start_matches(prefix.as_slice())
                            .trim_end_matches(suffix.as_slice());

                        if ignore_case {
                            string.to_lowercase()
                        } else {
                            string.to_owned()
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        
        if unique {
            // only sort if unique/dedup() needed
            // Eaf::token() will sort tokens anyway
            tokens.sort();
            tokens.dedup();
        }

        tokens
    }

    /// Adds an annotation to the tier.
    /// 
    /// Does not (can not) evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    /// 
    /// Must have time stamps set. Overlapping timestamps within the tier
    /// raises an error.
    ///
    /// Make sure to add the corresponding time slot value to `Eaf`.
    pub(crate) fn add(&mut self, annotation: &Annotation) -> Result<(), EafError> {
        // ensure annotation/tier types match
        if self.is_ref() != annotation.is_ref() {
            return Err(EafError::AnnotationTypeMismatch)
        }

        if self.overlaps(annotation) {
            return Err(EafError::AnnotationOverlap);
        }

        // Add annotation as last item
        self.annotations.push(annotation.to_owned());

        // Then sort by derived time values
        self.annotations
            .par_sort_by_cached_key(|a|
                // a.ts_val().0.ok_or(EafError::MissingTimeslotVal(a.id())));
                a.ts_val().0.expect("Fatal: Annotation has no time value set."));

        Ok(())
    }

    /// Returns `true` if the annotation's timespan overlaps
    /// with any existing annotation in the tier
    pub(crate) fn overlaps(&self, annotation: &Annotation) -> bool {
        self.annotations.iter().any(|a| a.overlaps(annotation))
    }

    /// Returns tier attributes from other tier
    /// with annotations in tact.
    pub fn with_attributes(self, tier: &Tier) -> Self {
        let parent = match self.is_ref() == tier.is_ref(){
            true => tier.parent_ref.to_owned(),
            false => todo!(),
        };
        Self {
            tier_id: tier.tier_id.to_owned(),
            participant: tier.participant.to_owned(),
            annotator: tier.annotator.to_owned(),
            linguistic_type_ref: tier.linguistic_type_ref.to_owned(),
            default_locale: tier.default_locale.to_owned(),
            parent_ref: tier.parent_ref.to_owned(),
            ext_ref: tier.ext_ref.to_owned(),
            lang_ref: tier.lang_ref.to_owned(),
            ..self
        }
    }

    // /// Return overlapping annotations
    // pub fn get_overlaps(&self) {
        
    // }

    /// Extends existing annotations as the last items in tier.
    /// 
    /// Does not (can not) evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    ///
    /// Make sure to add the corresponding time slot value to `Eaf`.
    ///
    /// To add an annotation at an abitrary position (e.g. in the middle of a tier),
    /// use `AnnotationDocument::add_annotation()` instead,
    /// since time slots may have to be added and re-mapped.
    pub fn extend(&mut self, annotations: &[Annotation]) {
        // !!! ensure annotation type matches before appending
        self.annotations.extend(annotations.to_owned())
    }

    /// Joins two tiers.
    /// 
    /// The first tier's (the one this method is used on)
    /// attributes will be preserved, the second one's will discarded.
    /// 
    /// No checks for duplicate annotations or time slot correctness are made.
    pub fn join(&mut self, tier: &Tier) {
        // !!! ensure tier type matches before appending
        self.annotations.extend(tier.annotations.to_owned());
    }

    /// Merges two tiers.
    /// 
    /// All annotations must have explicit time stamp values set.
    /// 
    /// The first tier's (the one this method is used on)
    /// is prioritized in terms of what attributes will be preserved.
    /// If join is set to `true` overlapping annotations will be joined.
    /// Otherwise, the first annotation in chronological order
    /// will be prioritised. The second annotation's start time stamp.
    /// TODO currently does not remap ID values. Should be optional if implemented.
    pub fn _merge(&mut self, tier: &Tier, _join: bool) -> Result<(), EafError> {
        // Return error if tiers are not the same type
        if self.is_ref() != tier.is_ref() {
            return Err(EafError::TierTypeMismatch((
                self.tier_id.to_owned(),
                tier.tier_id.to_owned(),
            )));
        }

        // dedup, but only for annotations that are exactly the same, including id, timestamps etc
        let mut annotations: HashSet<Annotation> = HashSet::new();
        annotations.extend(self.annotations.to_owned());
        annotations.extend(tier.annotations.to_owned());

        // Ensure timestamps are set
        if let Some(a) = annotations.iter().find(|a| !a.has_ta_val_set()) {
            return Err(EafError::TimeslotValMissing(a.id().to_owned()))
        }

        // create vec and sort remaining annotations
        let mut sorted: Vec<Annotation> = annotations.into_iter().collect();
        sorted.sort_by_key(|a| a.ts_val().0.unwrap());

        self.annotations = sorted;

        // Solve overlaps. Currently hinges on that sorting annotations via ID make them chronologically sorted
        // let mut adjusted: Vec<Annotation> = Vec::new();
        // // sorted.windows(2).inspect(|f| )
        // for annots in sorted.windows(2) {
        //     let a1 = &annots[0];
        //     let a2 = &annots[1];

        //     match (a1.ts_val(), a2.ts_val()) {
        //         ((Some(ts1), Some(te1)), (Some(ts2), Some(te2))) => {

        //         },
        //         _ => return Err(EafError::MissingTimeslotVal(format!("{} and/or {}", a1.id(), a2.id())))
        //     }
        // }

        // self.annotations = sorted;
        // if join {
        //     for annots in sorted.windows(2) {

        //     }
        // }

        // Join annotators
        match (&self.annotator, &tier.annotator) {
            (Some(a1), Some(a2)) => self.annotator = Some(format!("{a1};{a2}")),
            (None, Some(a2)) => self.annotator = Some(a2.to_owned()),
            _ => (),
        }

        // Join participants
        match (&self.participant, &tier.participant) {
            (Some(p1), Some(p2)) => self.participant = Some(format!("{p1};{p2}")),
            (None, Some(p2)) => self.participant = Some(p2.to_owned()),
            _ => (),
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = Annotation> {
        self.annotations.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    /// Returns a hashmap of time slots generated from the annotations
    /// in the tier as `HashMap<time_slot_reference, time_slot_value>`.
    /// 
    /// If timeslot values are not set for an annotation
    /// `TimeSlot.value` will be set to `None`.
    pub fn lookup_timeslots(&self) -> HashMap<String, Option<i64>> {
        let mut ts: HashMap<String, Option<i64>> = HashMap::new();
        self.iter().for_each(|a| {
            if let (Some((ref1, ref2)), (val1, val2)) = (a.ts_ref(), a.ts_val()) {
                ts.insert(ref1, val1);
                ts.insert(ref2, val2);
            }
        });
        ts
    }

    /// Generate time slots from derived time values.
    /// 
    /// Returns an empty vector if the tier does not contain
    /// time alignable annotations. Note that even referred
    /// annotations may be time alignable with constraints
    /// such as "Included_In"
    pub fn ts(&mut self, index: usize) -> Vec<TimeSlot> {
        let index = if index == 0 {1} else {index};
        self.annotations
            .iter_mut()
            // Need to ignore annotations with no time slot refs.
            .filter(|a| a.ts_ref().is_some())
            .enumerate()
            .flat_map(|(i, a)| a.ts(index + i*2))
            .collect()
    }

    pub fn set_a_id(&mut self, index: usize) {
        
    }


    /// Generates and returns timeslots for all annotations in this tier.
    /// For setting e.g. `TimeOrder` in `Eaf`.
    /// 
    /// If timeslot values are not set for an annotation
    /// `TimeSlot.value` will be set to `None`.
    pub fn derive_timeslots(&self) -> Option<Vec<TimeSlot>> {
        let mut ts: Vec<TimeSlot> = Vec::new();
        for a in self.annotations.iter() {
            let (t_ref1, t_ref2) = a.ts_ref()?;
            let (t1, t2) = a.ts_val();
            let ts1 = TimeSlot::new(&t_ref1, t1);
            let ts2 = TimeSlot::new(&t_ref2, t2);
            ts.append(&mut vec![ts1, ts2])
        }

        // self.annotations.iter()
        //     .map(|a| {
        //         let (t_ref1, t_ref2) = a.ts_ref()?;
        //         let (t1, t2) = a.ts_val();
        //         let ts1 = TimeSlot::new(&t_ref1, t1);
        //         let ts2 = TimeSlot::new(&t_ref2, t2);
        //         ts.append(&mut vec![ts1, ts2])
        //     })
        //     .collect()

        Some(ts)
    }

    /// Returns the tier's annotation values as `&str`s.
    pub fn values(&self) -> Vec<&str> {
        self.iter().map(|a| a.to_str()).collect()
    }

    /// Returns the annotation ID for the tier's first annotation.
    pub fn first_a_id(&self) -> Option<&str> {
        self.annotations.first().map(|a| a.id())
    }
    
    /// Returns the annotation ID for the tier's last annotation.
    pub fn last_a_id(&self) -> Option<&str> {
        self.annotations.last().map(|a| a.id())
    }

    /// Attempts to sort annotation IDs via their numerical component (e.g. "39", in "a39").
    /// 
    /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    /// has generated the file, and generates arbitrary annotation IDs that do not conform
    /// to ELAN's own convention.
    fn sort_a_id_numerical(&self) -> Vec<&str> {
        let mut num_id: Vec<(usize, &str)> = self.iter()
            .filter_map(|a| {
                let id = a.id();
                // returns e.g. 39 as usize in "a39"
                let num = a.id_num();

                // Ignore anything that can not be parsed
                match num {
                    Ok(n) => Some((n, id)),
                    Err(_) => None
                }
            })
            .collect();

        assert_eq!(self.len(), num_id.len(), "Annotations and sorted annotation IDs differ in length");

        num_id.sort_by_key(|(n, _)| *n);

        num_id.iter()
            .map(|(_, s)| s)
            .cloned()
            .collect()
    }

    /// Returns hash of all concatenated annotation value.
    /// Used internally as fingerprint or unique ID for a tier.
    pub(crate) fn hash(&self) -> std::io::Result<Vec<u8>> {
        let values = self.values().join("");
        let mut hasher = blake3::Hasher::new();
        let _size = copy(&mut std::io::Cursor::new(values), &mut hasher)?;
        Ok(hasher.finalize().as_bytes().to_ascii_lowercase())
    }

    /// Sets all main annotation IDs to UUID v4 to make
    /// these globally unique. Returns a hashmap
    /// where key = UUID v4 (new annotation IDs),
    /// value = old annotation ID.
    /// 
    /// Note that a second run at EAF-level
    /// is required to set referred annotation IDs
    /// (`ANNOTATION_REF`).
    /// 
    /// For internal use to track annotatations when e.g.
    /// merging tiers.
    pub(crate) fn tag(&mut self) -> HashMap<String, String>{
        self.iter_mut()
            .map(|a| a.tag())
            .collect()
    }

    pub(crate) fn untag(&mut self) {}

    // /// Returns the "minimum" annotation ID via their numerical component,
    // /// (e.g. "39" < "103", in "a39" and "a103", returns "a39").
    // /// 
    // /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    // /// has generated the file with arbitrary annotation IDs that do not conform
    // /// to ELAN's own convention
    // pub fn min_a_id(&self) -> Option<&str> {
    //     self.sort_a_id_numerical().first().copied() // &&str -> &str...
    // }

    // /// Returns the numerical component of the "minimum" annotation ID,
    // /// (e.g. "39" < "103", in "a39" and "a103", returns 39).
    // /// 
    // /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    // /// has generated the file with arbitrary annotation IDs that do not conform
    // /// to ELAN's own convention.
    // pub fn min_a_id_num(&self) -> Result<usize, EafError> {
    //     self.min_a_id().ok_or(EafError::TimeslotSortingError)?
    //         .trim_start_matches(char::is_alphabetic).parse::<usize>().map_err(|e| e.into())
    // }

    // /// Returns the "maximum" annotation ID via their numerical component,
    // /// (e.g. "39" < "103", in "a39" and "a103", returns "a103").
    // /// 
    // /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    // /// has generated the file with arbitrary annotation IDs that do not conform
    // /// to ELAN's own convention.
    // pub fn max_a_id(&self) -> Option<&str> {
    //     self.sort_a_id_numerical().last().copied()
    // }

    // /// Returns the numerical component of the "maximum" annotation ID,
    // /// (e.g. "39" < "103", in "a39" and "a103", returns 103).
    // /// 
    // /// This will return an error in cases where software other than ELAN (such as SIL FLEx)
    // /// has generated the file with annotation IDs that do not conform
    // /// to ELAN's own convention.
    // pub fn max_a_id_num(&self) -> Result<usize, EafError> {
    //     self.max_a_id().ok_or(EafError::TimeslotSortingError)?
    //         .trim_start_matches(char::is_alphabetic).parse::<usize>().map_err(|e| e.into())
    // }
}
