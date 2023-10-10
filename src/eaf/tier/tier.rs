//! Tier.
//! 
//! ELAN tiers can be either a main tier,
//! or a referred tier (refers to a parent tier).
//! 
//! Note that a referred tier may refer to another
//! referred tier.

use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde::{Deserialize, Serialize};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

use crate::{Annotation, EafError, TimeSlot};

/// EAF tier.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Tier {
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

    /// Returns an independent copy with specified tier ID.
    pub fn with_id(self, tier_id: &str) -> Self {
        Self{
            tier_id: tier_id.to_owned(),
            ..self
        }
    }

    /// Returns an independent copy with specified parent tier reference ID.
    /// I.e. the tier effectively becomes a referred tier.
    /// 
    /// Note that annotation type remains unchanged.
    pub fn with_parent_ref(self, parent_ref: &str) -> Self {
        Self{
            parent_ref: Some(parent_ref.to_owned()),
            ..self
        }
    }

    /// Returns an independent copy with specified linguistic type.
    pub fn with_linguistic_type_ref(self, linguistic_type_ref: &str) -> Self {
        Self{
            linguistic_type_ref: linguistic_type_ref.to_owned(),
            ..self
        }
    }

    /// Returns an independent copy with annotations stripped,
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
        start_index: Option<usize>,
    ) -> Self {
        let mut tier = Tier::default().with_id(tier_id);
        let idx = start_index.unwrap_or(1);
        
        tier.annotations = values.par_iter().enumerate()
            .map(|(i, (val, t1, t2))| {
                let mut annot = Annotation::alignable(
                    &val,
                    &format!("a{}", i+idx),            // 0+1
                    &format!("ts{}", (i+idx) * 2 - 1), // (0+1)*2-1
                    &format!("ts{}", (i+idx) * 2),     // (0+1)*2
                );
                annot.set_ts_val(Some(*t1), Some(*t2));
                annot
            })
            .collect();

        tier
    }

    /// Generates a new referred tier from values, assumed to be in chronologial order.
    /// 
    /// Number of values are not necessarily equal to the number of annotations
    /// in the parent tier, this depends on linguistic type ref definition.
    /// 
    /// If no start index is specified, the first annotation ID will succeed the last one
    /// in the parent tier, and increment by one.
    /// 
    /// If the parent is empty an empty ref tier is created, since there is nothing
    /// to refer to.
    pub fn ref_from_values(
        values: &[String],
        tier_id: &str,
        parent: &Tier,
        linguistic_type_ref: &str,
        start_index: Option<usize>,
    ) -> Result<Tier, EafError> {
        let mut ref_tier = Tier::default()
            .with_id(tier_id)
            .with_parent_ref(&parent.tier_id)
            .with_linguistic_type_ref(linguistic_type_ref);

        if !values.is_empty() {
            if values.len() > parent.len() {
                return Err(EafError::RefTierAlignmentError((
                    tier_id.to_owned(),
                    parent.tier_id.to_owned(),
                )));
            }

            let first_idx: usize = if let Some(id) = start_index {
                id
            } else {
                match parent.max_a_id() {
                    Some(id) => id.trim_start_matches(char::is_alphabetic).parse()?,
                    None => 1,
                }
            };

            let mut id_count: usize = 0;
            for (i, val) in values.iter().enumerate() {
                if let Some(parent_a) = parent.annotations.get(i) {
                    let a = Annotation::referred(
                        val,
                        &format!("a{}", first_idx + id_count),
                        &parent_a.id(),
                        None,
                    );
                    ref_tier.add(&a);
                    id_count += 1;
                }
            }
        }

        Ok(ref_tier)
    }

    /// Returns `true` if the tier is a referred tier.
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

    /// Returns average annotation length,
    /// i.e. average number of tokens
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

    /// Returns average word/token length,
    /// i.e. average number of characters
    /// in each word/token.
    pub fn avr_token_len(&self) -> f64 {
        let t_len: Vec<f64> = self.annotations.iter()
            .map(|a| a.avr_len()) // average token length for annotation
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

    /// Returns a reference to the annotation with specified ID,
    /// if it exits.
    pub fn find(&self, annotation_id: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.id() == annotation_id)
    }

    /// Matches annotation values against a pattern.
    /// 
    /// Returns tuples in the form
    /// `(Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID)`.
    /// where index corresponds to annotation order in the EAF-file.
    pub fn query(&self, pattern: &str, ignore_case: bool) -> Vec<(usize, String, String, String, Option<String>)> {
        self.iter()
        // self.annotations.par_iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let org_val = a.to_str().to_owned();
                let (val, ptn) = match ignore_case {
                    true => (org_val.to_lowercase(), pattern.to_lowercase()),
                    false => (org_val.to_owned(), pattern.to_owned()),
                };
                if val.contains(&ptn) {
                    Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Match annotation values against a regular expression.
    /// 
    /// Returns tuples in the form
    /// `(Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID)`.
    /// where index corresponds to annotation order in the EAF-file.
    pub fn query_rx(&self, regex: &Regex) -> Vec<(usize, String, String, String, Option<String>)> {
        self.iter()
        // self.annotations.par_iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let org_val = a.to_str().to_owned();
                if regex.is_match(&org_val) {
                    Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
                } else {
                    None
                }
            })
            .collect()
    }

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
    /// and or ignoring case.
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

        tokens.sort();

        if unique {
            tokens.dedup();
        }

        tokens
    }

    /// Adds an annotation as the last item in the tier.
    /// 
    /// Does not evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    ///
    /// Make sure to add the corresponding time slot value to `AnnotationDocument`.
    ///
    /// To add an annotation at an abitrary position (e.g. in the middle of a tier),
    /// use `AnnotationDocument::add_annotation()` instead,
    /// since time slots may have to be added and re-mapped.
    pub fn add(&mut self, annotation: &Annotation) {
        self.annotations.push(annotation.to_owned())
    }

    /// Extends existing annotations as the last items in tier.
    /// 
    /// Does not evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    ///
    /// Make sure to add the corresponding time slot value to `AnnotationDocument`.
    ///
    /// To add an annotation at an abitrary position (e.g. in the middle of a tier),
    /// use `AnnotationDocument::add_annotation()` instead,
    /// since time slots may have to be added and re-mapped.
    pub fn extend(&mut self, annotations: &[Annotation]) {
        self.annotations.extend(annotations.to_owned())
    }

    /// Joins two tiers.
    /// 
    /// The first tier's (the one this method is used on)
    /// attributes will be preserved, the second one's will discarded.
    /// 
    /// No checks for duplicate annotations or time slot correctness are made.
    pub fn join(&mut self, tier: &Tier) {
        self.annotations.extend(tier.annotations.to_owned());
    }

    pub fn _overlaps(&self) {
        // cmp current boundaries with next
        // either:
        // - join
        // - preserve current (shift start or next forwards, back to back)
        // - preserve next (shift end of current backwards, back to back)
        // but what if an annotation's time span from file 1 completely
        unimplemented!()
    }

    /// Merges two tiers.
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
            return Err(EafError::IncompatibleTiers((
                self.tier_id.to_owned(),
                tier.tier_id.to_owned(),
            )));
        }

        // dedup, but only for annotations that are exactly the same, including id, timestamps etc
        let mut annotations: HashSet<Annotation> = HashSet::new();
        println!("SELF BEFORE: {}", self.len());
        annotations.extend(self.annotations.to_owned());
        println!("TIER BEFORE: {}", tier.len());
        annotations.extend(tier.annotations.to_owned());

        // create vec and sort remaining annotations
        let mut sorted: Vec<Annotation> = annotations.into_iter().collect();
        sorted.sort_by_key(|a| a.id()); // should maybe sort by timestamp...? check with Han if annot id is always ordered?
        println!("MERGED: {}", sorted.len());

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

        Some(ts)
    }

    /// Returns the tier's annotation values as `&str`s.
    pub fn values(&self) -> Vec<&str> {
        self.iter().map(|a| a.to_str()).collect()
    }

    /// Returns the annotation ID for the tier's first annotation.
    pub fn first_a_id(&self) -> Option<String> {
        self.annotations.first().map(|a| a.id())
    }
    
    /// Returns the annotation ID for the tier's last annotation.
    pub fn last_a_id(&self) -> Option<String> {
        self.annotations.last().map(|a| a.id())
    }

    /// Attempts to sort annotation IDs via their numerical component (e.g. "39", in "a39").
    /// 
    /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    /// has generated the file, and generates arbitrary annotation IDs that do not conform
    /// to ELAN's own convention.
    fn sort_a_id_numerical(&self) -> Vec<String> {
        let mut id: Vec<(usize, String)> = self.iter()
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

        assert_eq!(self.len(), id.len(), "Annotations and sorted annotation IDs differ in length");

        id.sort_by_key(|(n, _)| *n);

        id.iter()
            .map(|(_, s)| s)
            .cloned()
            .collect()
    }

    /// Returns the "minimum" annotation ID via their numerical component,
    /// (e.g. "39" < "103", in "a39" and "a103").
    /// 
    /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    /// has generated the file, and generates arbitrary annotation IDs that do not conform
    /// to ELAN's own convention
    pub fn min_a_id(&self) -> Option<String> {
        self.sort_a_id_numerical().first().cloned()
    }

    /// Returns the "maximum" annotation ID via their numerical component,
    /// (e.g. "39" < "103", in "a39" and "a103").
    /// 
    /// This will not work in cases where software other than ELAN (such as SIL FLEx)
    /// has generated the file, and generates arbitrary annotation IDs that do not conform
    /// to ELAN's own convention
    pub fn max_a_id(&self) -> Option<String> {
        self.sort_a_id_numerical().last().cloned()
    }
}
