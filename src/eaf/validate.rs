//! A few simple validation checks for various sections of an EAF-file.
//! Does not substitue validating against the corresponding XML-schema.

use std::{ops::Range, collections::HashSet};

use crate::{Annotation, TimeSlot, Tier, Eaf};

/// Returns `true` if any annotation timespans overlap.
pub(crate) fn overlap(annotations: &[Annotation]) -> bool {
    let mut ranges: Vec<Range<i64>> = annotations.iter()
        .filter_map(|a| if let (Some(start), Some(end)) = a.ts_val() {
            Some(start..end)
        } else {
            None
        })
        .collect();

    // filter_map silently discards annotations
    // with no explicit timestamps set (was possible on old EAF formats versions)
    assert_eq!(ranges.len(), annotations.len(), "Failed to compile annotation timestamps for overlap check");

    // Sort ranges on start value to be able to check overlaps,
    // return true on first overlap between end of one and start of next
    // ranges.sort_by(|a, b| a.start.cmp(&b.start));
    ranges.sort_by_key(|r| r.start);
    ranges.windows(2).any(|w| {
        let overlaps = w[0].end > w[1].start;
        if overlaps {
            panic!("OVERLAP {w:?}");

        }
        overlaps
    })
}


/// Returns `true` if any time slot ID occurs more than once.
pub(crate) fn ts_duplicates(timeslots: &[TimeSlot]) -> bool {
    timeslots.iter()
        .map(|ts| ts.time_slot_id.as_str())
        .collect::<HashSet<&str>>()
        .len() < timeslots.len()
}

/// Returns `true` if any tier ID occurs more than once.
pub(crate) fn tier_duplicates(tiers: &[Tier]) -> bool {
    tiers.iter()
        .map(|t| t.tier_id.as_str())
        .collect::<HashSet<&str>>()
        .len() < tiers.len()
}

/// Returns `false` is any annotation type does not match its tier type.
pub(crate) fn tier_type_match(tier: &Tier) -> bool {
    if tier.annotations.iter().any(|a| tier.is_ref() != a.is_ref()) {
        return false
    }
    true
}

/// Returns `false` is any timeslot
/// referred to by an annotation does not exist.
pub(crate) fn ts_exists(eaf: &Eaf) {
    // let to =
}
