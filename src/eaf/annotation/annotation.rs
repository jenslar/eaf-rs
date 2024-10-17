//! Annotation.
//!
//! ELAN annotations can be either an alignable annotation (part of a main tier),
//! or a referred annotation (part of a referred tier).
//! 
//! Aligned annotations contain references to time slots (the annotation's time span),
//! whereas referred annotations reference an annotation in the parent tier.
//! 
//! Note that some functionality only applies to whitespace delimited scripts, 
//! such as anything that requires counting words/tokens (if solutions arise that do not
//! require language specific parsers or models I will add this, but as of now it is not
//! a simple problem to wolve).

use std::{collections::HashMap, fmt::Display};
use regex::Regex;
use serde::{Serialize, Deserialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::{EafError, TimeSlot};

use super::{
    AnnotationType,
    RefAnnotation,
    AlignableAnnotation,
    AnnotationValue,
    AnnotationBuilder,
    AnnotationAlignable
};

/// Annotation. Two types exist:
/// - Aligned: part of a main tier, with explicit time slot references
/// - Referred: part of a referred tier, references an annotation in the parent tier
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename = "ANNOTATION")]
pub struct Annotation {
    #[serde(rename = "$value")]
    annotation_type: AnnotationType,
}

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.to_str())
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => write!(f,
                "ID: {} | Timeslots: {}, {} ({} - {} ms) | '{}'",
                a.annotation_id,
                a.time_slot_ref1,
                a.time_slot_ref2,
                a.time_value1.unwrap_or_default(),
                a.time_value2.unwrap_or_default(),
                a.annotation_value.as_ref()
            ),
            AnnotationType::RefAnnotation(a) => write!(f,
                "ID: {} | REF ID: {} ({} - {} ms) | '{}'",
                a.annotation_id,
                a.annotation_ref,
                a.time_value1.unwrap_or_default(),
                a.time_value2.unwrap_or_default(),
                a.annotation_value.as_ref()
            ),
        }
    }

}

impl From<RefAnnotation> for Annotation {
    fn from(ref_annotation: RefAnnotation) -> Self {
        Self {
            annotation_type: AnnotationType::RefAnnotation(ref_annotation)
        }
    }
}

impl From<AlignableAnnotation> for Annotation {
    fn from(alignable_annotation: AlignableAnnotation) -> Self {
        Self {
            annotation_type: AnnotationType::AlignableAnnotation(alignable_annotation)
        }
    }
}

impl Annotation {
    /// Creates a new annotation from `AnnotationType`.
    pub fn new(annotation_type: &AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::AlignableAnnotation(a) => Self::from(a.to_owned()),
            AnnotationType::RefAnnotation(a) => Self::from(a.to_owned())
        }
    }

    /// Creates an annotation builder.
    pub fn builder() -> AnnotationBuilder<AnnotationAlignable> {
        AnnotationBuilder::new()
    }

    /// Creates a new aligned annotation.
    pub fn alignable(
        annotation_value: &str,
        annotation_id: &str,
        time_slot_ref1: &str,
        time_slot_ref2: &str,
    ) -> Self {
        Self::builder()
            .annotation_id(annotation_id) // error if not set
            .annotation_value(annotation_value)
            .time_slot_refs(time_slot_ref1, time_slot_ref2) // error if not set
            .build()
            .expect("Failed to build alignable annotation (this is a bug)") // unwrap ok, id + ts refs set
    }

    /// Creates a new referred annotation.
    /// 
    /// `previous` is used to set ID
    /// for previous annotation
    /// when building a tokenized tier.
    pub fn referred(
        annotation_value: &str,
        annotation_id: &str,
        annotation_ref: &str,
        previous: Option<&str>,
    ) -> Self {
        let mut builder = Self::builder()
            .annotation_id(annotation_id) // error if not set
            .annotation_value(annotation_value)
            .annotation_ref(annotation_ref); // error if not set
        
        if let Some(prev) = previous {
            builder = builder.previous_annotation(prev)
        }

        builder
            .build()
            .expect("Failed to build referred annotation (this is a bug)") // unwrap ok, id + ref set
    }

    /// Converts a ref annotation to an alignable annotation.
    /// If input annotation is already an alignable annotation,
    /// a copy is returned untouched.
    /// 
    /// Does not validate provided time slot references.
    pub fn to_alignable(
        &self,
        time_slot_ref1: &str,
        time_slot_ref2: &str
    ) -> Annotation {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(_) => self.to_owned(),
            AnnotationType::RefAnnotation(a) => {
                Annotation::from(
                    AlignableAnnotation {
                        annotation_id: a.annotation_id.to_owned(),
                        ext_ref: a.ext_ref.to_owned(),
                        lang_ref: a.lang_ref.to_owned(),
                        cve_ref: a.cve_ref.to_owned(),
                        time_slot_ref1: time_slot_ref1.to_owned(),
                        time_slot_ref2: time_slot_ref2.to_owned(),
                        annotation_value: a.annotation_value.to_owned(),
                        tier_id: a.tier_id.to_owned(),
                        time_value1: a.time_value1,
                        time_value2: a.time_value2,
                    }
                )
            }
        }
    }

    /// Converts an alignable annotation to a ref annotation.
    /// If input annotation is already a ref annotation,
    /// a copy is returned untouched.
    /// 
    /// Does not validate specified reference annotation ID (`ref_id`)
    /// or previous annotation (`prev`).
    pub fn to_referred(
        &self,
        ref_id: &str,
        previous_annotation: Option<&str>,
        main_annotation: Option<&str>
    ) -> Annotation {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                Annotation::from(
                    RefAnnotation {
                        annotation_id: a.annotation_id.to_owned(),
                        ext_ref: a.ext_ref.to_owned(),
                        lang_ref: a.lang_ref.to_owned(),
                        cve_ref: a.cve_ref.to_owned(),
                        annotation_ref: ref_id.to_owned(),
                        previous_annotation: previous_annotation.map(String::from),
                        annotation_value: a.annotation_value.to_owned(),
                        tier_id: a.tier_id.to_owned(),
                        time_value1: a.time_value1,
                        time_value2: a.time_value2,
                        main_annotation: main_annotation.map(String::from)
                    }
                )
            },
            AnnotationType::RefAnnotation(_) => self.to_owned()
        }
    }

    /// Returns the annotation value as string.
    pub fn to_str(&self) -> &str {
        self.value().as_ref()
    }

    /// Returns all words/tokens in the annotation value.
    /// Restricted to whitespace delimited scripts.
    pub fn tokens(&self) -> Vec<&str> {
        self.to_str()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
    }
    
    /// Returns number of words/tokens in the annotation value.
    /// Restricted to whitespace delimited scripts.
    pub fn len(&self) -> usize {
        self.tokens()
            .len()
    }

    /// Returns average token length.
    /// 
    /// Restriction: Only applies to whitespace delimited scripts,
    /// and may return the wrong for values for some scripts.
    /// Use `graphemes = true` in these cases.
    /// 
    /// `graphemes = true` uses <https://crates.io/crates/unicode-segmentation>
    /// to determine grapheme clusters (currently set to find extended grapheme clusters)
    /// as opposed to Rust's built-in `char` type.
    pub fn avr_len(&self, graphemes: bool) -> f64 {
        let tkn_len: Vec<usize> = self.to_str()
            .split_ascii_whitespace()
            .map(|s| if graphemes {
                s.graphemes(true).count()
            } else {
                s.chars().count()
            })
            .collect();

        match tkn_len.len() {
            0 => 0.,
            _ => tkn_len.iter().sum::<usize>() as f64 / tkn_len.len() as f64
        }
    }

    /// Returns the annotation value.
    pub fn value(&self) -> &AnnotationValue {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => &a.annotation_value,
            AnnotationType::RefAnnotation(a) => &a.annotation_value
        }
    }

    /// Sets the annotation value.
    pub fn set_value(&mut self, annotation_value: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.annotation_value = AnnotationValue::from(annotation_value);
            },
            AnnotationType::RefAnnotation(a) => {
                a.annotation_value = AnnotationValue::from(annotation_value);
            }
        }
    }

    /// Returns the annotation ID.
    pub fn id(&self) -> &str {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.annotation_id.as_str(),
            AnnotationType::RefAnnotation(a) => a.annotation_id.as_str()
        }
    }

    /// Returns the numerical component of the annotation ID.
    /// May fail in cases where third-party software does not
    /// follow ELAN's naming conventions.
    pub fn id_num(&self) -> Result<usize, EafError> {
        self.id().trim_start_matches(char::is_alphabetic)
            .parse()
            .map_err(|e| EafError::ParseIntError(e))
    }

    /// Sets annotation ID.
    pub fn set_id(&mut self, annotation_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.annotation_id = annotation_id.to_owned(),
            AnnotationType::RefAnnotation(a) => a.annotation_id = annotation_id.to_owned()
        };
    }

    /// Returns referred annotation ID for a referred annotation,
    /// and `None` for an aligned annotation.
    /// 
    /// I.e. the value for attribute `ANNOTATION_REF`,
    /// if the annotation is a `REF_ANNOTATION`.
    pub fn ref_id(&self) -> Option<&str> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => Some(a.annotation_ref.as_str()),
            _ => None,
        }
    }

    /// Sets annotation ref ID if annotation is a "referred annotation".
    pub fn set_ref_id(&mut self, ref_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.annotation_ref = ref_id.to_owned(),
            _ => (),
        };
    }

    /// Returns true if annotation is a "referred annotation".
    pub fn is_ref(&self) -> bool {
        matches!(self.annotation_type, AnnotationType::RefAnnotation(_))
    }

    /// Returns annotation ID for "previous annotation" if it exists
    /// and annotation is a referred annotation, and None otherwise.
    /// 
    /// If this attribute is set it indicates that the parent tier is tokenzied.
    /// 
    /// Note that the first annotation for a series of tokenized annotations does not
    /// contain the `PREVIOUS_ANNOTATION` attribute, only those that follow do.
    pub fn previous(&self) -> Option<&str> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.previous_annotation.as_deref(),
            _ => None,
        }
    }

    /// Sets annotation ref ID if annotation is a ref annotation.
    pub fn set_previous(&mut self, prev_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => {
                a.previous_annotation = Some(prev_id.to_owned())
            },
            _ => (),
        };
    }

    /// Generates start, end timeslots for annotation
    /// and sets generated time slot references.
    /// Time values must have been be derived and set.
    pub(crate) fn ts(&mut self, index: usize) -> [TimeSlot; 2] {
        let (t1, t2) = self.ts_val();
        let (r1, r2) = (format!("ts{}", index), format!("ts{}", index+1));
        
        self.set_ts_ref(&r1, &r2);
        
        [
            TimeSlot::new(&r1, t1),
            TimeSlot::new(&r2, t2),
        ]
    }

    /// Returns time slot references for an alignable annotation,
    /// and `None` for a referred annotation.
    /// I.e. the attributes `TS_REF1` and `TS_REF2` if annotation is an alignable annotation.
    pub fn ts_ref(&self) -> Option<(String, String)> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                Some((a.time_slot_ref1.to_owned(), a.time_slot_ref2.to_owned()))
            },
            _ => None
        }
    }

    /// Sets (new) time slot references for an alignable annotation.
    /// I.e. the attributes `TS_REF1` and `TS_REF2` if annotation is an alignable annotation.
    pub fn set_ts_ref(&mut self, time_slot_ref1: &str, time_slot_ref2: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.time_slot_ref1 = time_slot_ref1.to_owned();
                a.time_slot_ref2 = time_slot_ref2.to_owned();
            },
            _ => ()
        }
    }

    /// Returns external reference if it exists.
    pub fn ext_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.ext_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.ext_ref.to_owned(),
        }
    }

    /// Sets external reference.
    pub fn set_ext_ref(&mut self, ext_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.ext_ref = ext_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.ext_ref = ext_ref.map(String::from),
        }
    }

    /// Returns language reference if it exists.
    pub fn lang_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.lang_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.lang_ref.to_owned(),
        }
    }

    /// Sets language reference.
    pub fn set_lang_ref(&mut self, lang_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.lang_ref = lang_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.lang_ref = lang_ref.map(String::from),
        }
    }

    /// Returns CVE reference if it exists.
    pub fn cve_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.cve_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.cve_ref.to_owned(),
        }
    }

    /// Sets CVE reference.
    pub fn set_cve_ref(&mut self, cve_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.cve_ref = cve_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.cve_ref = cve_ref.map(String::from),
        }
    }

    /// Returns main annotation ID for a referred annotation,
    /// and `None` for an aligned annotation (or if e.g. `Eaf::derive()`
    /// has not been run).
    /// 
    /// I.e. the annotation at the top of the hierarchy in the main tier.
    /// 
    /// Note that the field that is checked is not part of the EAF specification,
    /// and is populated when calling e.g. `Eaf::derive()`.
    pub fn main(&self) -> Option<&str> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.main_annotation.as_deref(),
            _ => None,
        }
    }

    /// Sets "main" annotation ID for a referred annotation.
    /// 
    /// I.e. if the annotation is deep in a nested hierarchy
    /// of referred tiers, this sets the specified ID
    /// as representing the alignable annotation "at the top"
    /// in the main tier. Mostly for internal use, since "main annotation"
    /// is derived and set via `Eaf::derive()`.
    /// 
    /// Note that this field is not part of the EAF specification.
    pub fn set_main(&mut self, main_annotation: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => {
                a.main_annotation = Some(main_annotation.to_owned());
            },
            _ => (),
        }
    }

    /// Returns annotation start and end time in milliseconds if set.
    /// 
    /// Note that the field that is checked is not part of the EAF specification,
    /// and is populated when calling e.g. `Eaf::derive()`.
    pub fn ts_val(&self) -> (Option<i64>, Option<i64>) {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                (a.time_value1, a.time_value2)
            },
            AnnotationType::RefAnnotation(a) => {
                (a.time_value1, a.time_value2)
            },
        }
    }

    /// Mutably sets annotation start and end time in milliseconds.
    /// 
    /// Note that these fields are not part of the EAF specification
    /// and are ignored when de/serializing, but creates a more independent
    /// `Annotation` whenever it is used outside the `Eaf` context.
    pub fn set_ts_val(&mut self, time_value1: Option<i64>, time_value2: Option<i64>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.time_value1 = time_value1;
                a.time_value2 = time_value2;
            },
            AnnotationType::RefAnnotation(a) => {
                a.time_value1 = time_value1;
                a.time_value2 = time_value2;
            },
        }
    }

    /// Sets annotation start and end time in milliseconds.
    /// 
    /// Note that these fields are not part of the EAF specification
    /// and are ignored when de/serializing, but creates a more independent
    /// `Annotation` whenever it is used outside the `Eaf` context.
    pub fn with_ts_val(self, time_value1: i64, time_value2: i64) -> Self {
        Self {
            annotation_type: {
                match self.annotation_type {
                    AnnotationType::AlignableAnnotation(a) => {
                       AnnotationType::AlignableAnnotation(a.with_time_values(time_value1, time_value2))
                    },
                    AnnotationType::RefAnnotation(a) => {
                       AnnotationType::RefAnnotation(a.with_time_val(time_value1, time_value2))
                    }
                }
            },
            ..self
        }
    }

    /// Returns `true` if start and end
    /// timestamps are both set.
    pub fn has_ta_val_set(&self) -> bool {
        let (ts1, ts2) = self.ts_val();
        if ts1.is_some() && ts2.is_some() {
            return true
        }
        false
    }

    /// Returns tier ID.
    /// 
    /// Note that this field is not part of the EAF specification,
    /// and is ignored when de/serializing.
    pub fn tier_id(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.tier_id.to_owned()
            },
            AnnotationType::RefAnnotation(a) => {
                a.tier_id.to_owned()
            },
        }
    }
    
    /// Sets tier ID.
    /// 
    /// Note that this field is not part of the EAF specification,
    /// and is ignored when de/serializing.
    pub fn set_tier_id(&mut self, tier_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.tier_id = Some(tier_id.to_owned());
            },
            AnnotationType::RefAnnotation(a) => {
                a.tier_id = Some(tier_id.to_owned());
            },
        }
    }

    /// Returns the annotation with specified tier ID.
    /// 
    /// Note that this field is not part of the EAF specification,
    /// and is ignored when de/serializing.
    pub fn with_tier_id(self, tier_id: &str) -> Self {
        Self {
            annotation_type: {
                match self.annotation_type {
                    AnnotationType::AlignableAnnotation(a) => {
                       AnnotationType::AlignableAnnotation(a.with_tier_id(tier_id))
                    },
                    AnnotationType::RefAnnotation(a) => {
                       AnnotationType::RefAnnotation(a.with_tier_id(tier_id))
                    }
                }
            },
            ..self
        }
    }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts that use whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// Returns `HashMap<ngram, count>`.
    pub fn ngram(&self, size: usize, delete_regex: Option<&Regex>) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();
        let split: Vec<String> = self.tokens()
            .iter()
            .map(|v| {
                if let Some(rx) = delete_regex {
                    rx.replace_all(&v.to_lowercase(), "").to_string()
                } else {
                    v.to_lowercase()
                }
            })
            .collect();

        for value in split.windows(size) {
            *ngrams.entry(value.join(" ")).or_insert(0) += 1;
        }

        ngrams
    }

    /// Returns `true` if the annotation value is identical
    /// 
    /// If `compare_time` is set to `true, start time and end time
    /// are also compared.
    pub fn is_identical(&self, annotation: &Self, compare_time: bool) -> bool {
        match compare_time {
            // Both annotation value and time stamps must match
            true => self.ts_val() == annotation.ts_val() && self.value() == annotation.value(),
            // Only annotation value must match
            false => self.value() == annotation.value()
        }
    }

    /// Returns `true` if annotation timepans overlap
    /// (assuming these are set).
    pub(crate) fn overlaps(&self, annotation: &Self) -> bool {
        if let (Some(t_self1), Some(t_self2)) = self.ts_val() {
            if let (Some(t_other1), Some(t_other2)) = annotation.ts_val() {
                let range = t_self1 .. t_self2;
                if range.contains(&t_other1) || range.contains(&t_other2) {
                    return true
                }
            }
        }
        
        false
    }

    /// Returns the [Blake3](https://github.com/BLAKE3-team/BLAKE3)
    /// hash for the annotation value.
    pub(crate) fn hash(&self) -> std::io::Result<Vec<u8>> {
        let mut value = self.value().to_owned();
        value.hash()
    }

    /// Sets annotation ID to UUID v4 and returns
    /// the tuple `(UUID, old_annotation_ID)`.
    /// 
    /// For internal use when e.g. merging tiers
    /// as a quick(-ish) way to ensure unique annotation IDs.
    pub(crate) fn tag(&mut self) -> (String, String) {
        let old_id = self.id().to_owned();
        self.set_id(&uuid::Uuid::new_v4().to_string());
        (self.id().to_owned(), old_id)
    }

    // /// Sets annotation ID to UUID v4 and returns
    // /// the tuple `(UUID, old_annotation_ID)`.
    // /// 
    // /// For internal use when e.g. merging tiers.
    // pub(crate) fn untag(&mut self, id: &str, ref_id: Option<&str>) -> (String, String) {
    //     let old_id = self.id().to_owned();
    //     self.set_id(&uuid::Uuid::new_v4().to_string());
    //     (self.id().to_owned(), old_id)
    // }
}