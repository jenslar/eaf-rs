use std::collections::{HashMap, HashSet};

use crate::{
    Annotation, Constraint, ControlledVocabulary, Eaf, EafError, Language, LinguisticType, Locale,
    StereoType, Tier, TimeOrder, TimeSlot,
};

use super::{controlled_vocabulary, language, overlap};

/// Merge multiple Eaf-structs into one.
///
/// Tiers with the same tier ID across Eaf-structs
/// will be merged into one, including referred tiers
/// (same behviour as ELAN).
///
/// Media files from the first Eaf will be used.
///
/// Currently returns error on
/// annotation overlaps in the same tier.
///
/// TODO 1. Handle overlaps, e.g. pass enum with values such as
/// TODO    'Join', 'DiscardFirst', 'DiscardLast' ...annotations contained
/// TODO    within timespan of other annotations?
///
/// TODO 2. Option for prefixing referred tier ID in order not
/// TODO    to collide with other Eaf when parent tier ID is not
/// TODO    the same.
///
/// TODO 3. Option to specify new media to link.
// pub(crate) fn merge_eafs(eafs: Vec<Eaf>, overlap: OverlapStrategy) -> Result<Eaf, EafError> {
pub(crate) fn merge_eafs(eafs: Vec<Eaf>) -> Result<Eaf, EafError> {
    if eafs.is_empty() {
        return Err(EafError::NoData);
    }

    // !!! 1a. add "tagged" and stripped tiers to hashmap, where key: original tier ID, value tiers with tier ID changed to uuid as unique ref
    // !!! 1b. lookup table key: uuid (annotation or tier uuid), value: original ID (original annotation or tier ID)
    // !!! 1b. add tagged annotations (i.e. tier uuid as parent tier) from above tier, where key: original tier ID, values: tagged annotations annotations
    // !!! generate hashmap of all annotations (derived with timestamps) with key: annot uuid, value annotation
    // !!! first get main tier annotations into hashmap, then run eaf.child_tiers recursively on those gradually digging down and adding?

    let mut eafs = eafs;
    // Preserve unique instances of references and CV.
    // !!! May raise errors if types with the same name
    // !!! contain different values.
    let mut linguistic_types: HashSet<LinguisticType> = HashSet::new();
    let mut locales: HashSet<Locale> = HashSet::new();
    let mut languages: HashSet<Language> = HashSet::new();
    let mut constraints: HashSet<Constraint> = HashSet::new();
    let mut controlled_vocabulary: HashSet<ControlledVocabulary> = HashSet::new();
    // key: tier ID, value: tiers with same tier ID
    // let mut tiers: HashMap<String, Vec<Tier>> = HashMap::new();
    let mut tiers: HashMap<String, Tier> = HashMap::new();
    // note: index may be removed in future release
    for eaf in eafs.iter_mut() {
        eaf.index();
        eaf.derive()?; // sets explicit timestamps for all annotations
        eaf.tag(); // generates and sets annotation IDs to UUID v4 as unique ID across files

        linguistic_types.extend(eaf.linguistic_types.to_owned());
        locales.extend(eaf.locales.to_owned());
        languages.extend(eaf.languages.to_owned());
        constraints.extend(eaf.constraints.to_owned());
        controlled_vocabulary.extend(eaf.controlled_vocabularies.to_owned());

        for tier in eaf.tiers.iter() {
            tiers
                .entry(tier.tier_id.to_owned())
                // Add tier without annotations if not in hashmap...
                .or_insert(tier.strip())
                .annotations
                // ...then add annotations to avoid adding twice for first item
                .extend(tier.annotations.to_owned())
        }
    }

    let mut merged_eaf = Eaf::default();

    merged_eaf.linguistic_types = linguistic_types.into_iter().collect();
    merged_eaf.locales = locales.into_iter().collect();
    merged_eaf.languages = languages.into_iter().collect();
    merged_eaf.constraints = constraints.into_iter().collect();
    merged_eaf.controlled_vocabularies = controlled_vocabulary.into_iter().collect();

    merged_eaf.tiers = tiers.values().cloned().collect::<Vec<Tier>>();
    merged_eaf.generate_timeorder();

    merged_eaf.tiers.sort_by_key(|t| t.tier_id.to_owned());
    merged_eaf.remap(None, None)?; // ok?

    Ok(merged_eaf)
}

/// Merge tiers. Must be of the same type,
/// i.e. a referred tier can not be merged with
/// a main tier. (or...? if timestamps are set it may be ok...)
/// Annotations must have explicit time values set.
pub(crate) fn merge_tiers(tiers: &[Tier]) -> Result<Tier, EafError> {
    if tiers.is_empty() {
        return Err(EafError::NoData);
    }
    let first = tiers.first().unwrap();
    let is_ref = first.is_ref();
    if tiers.iter().any(|t| t.is_ref() != is_ref) {
        return Err(EafError::TypeMismatch);
    }

    // Use attributes from first tier in order
    let mut merged_tier = Tier::default().with_attributes(first);
    for tier in tiers.iter() {
        merged_tier.extend(&tier.annotations)
    }

    if overlap(&merged_tier.annotations) {
        return Err(EafError::AnnotationOverlap);
    }

    merged_tier.annotations.sort_by_key(|a| a.ts_val());

    Ok(merged_tier)
}

pub(crate) fn merge_annotations(annotations: &[Annotation], overlap: OverlapStrategy) {}

pub enum OverlapStrategy {
    /// Do nothing. For e.g. raising
    /// errors on overlaps.
    None,
    /// Join annotations, i.e.
    /// new time span will be first annotation's
    /// start time, and last annotation's end time.
    Join,
    /// Discard first annotation.
    DiscardFirst,
    /// Discard last annotation.
    DiscardLast,
    /// First annotation's end time will move
    /// that of the second annotation's start time.
    PrioritizeFirst,
    /// Second annotation's start time will move
    /// that of the first annotation's end time.
    PrioritizeLast,
    // annotation contained within other annotation's timespan?
}

pub(crate) fn merge2(eafs: Vec<Eaf>) {}
