//! NOTE: Not yet used anywhere.

use std::cmp::Ordering;

use crate::Annotation;

/// Overlap variants. Values represent
/// the shift in milliseconds required to shift the boundaries
/// of the source annotation to those of the target annotation.
pub enum Overlap {
    /// The source annotation's start and end timestamps,
    /// are smaller and larger than those of the comparison target annotation.
    /// I.e. it surrounds the comparison annotation time wise.
    Surrounds(i64, i64), // relative timestamps in ms
    // Surrounds,
    /// The source annotation's start end and timestamps,
    /// are larger and smaller than those of comparison target annotation.
    /// I.e. it is contained by the comparison annotation time wise
    Contained(i64, i64), // relative timestamps in ms
    // Contained,
    /// The source annotation's start time stamp is contained
    /// within the boundaries of the comparison target annotation,
    /// OR its start time is equal to the target's start time but less than
    /// its end time,
    /// OR its start time is less than the target's
    Start(i64), // relative timestamps in ms
    // Start,
    /// The source annotation's end time stamp is contained
    /// within the boundaries of the comparison target annotation.
    /// or its end time is equal to the target's start time but less than
    /// its end time.
    End(i64), // relative timestamps in ms
    Equal,
    // End,
    // /// Any of the conditions the other variants represent.
    // Any(Option<i64>, Option<i64>),
}

impl Overlap {
    pub fn resolve(source: &Annotation, target:&Annotation) -> Option<Self> {
        let (source_start, source_end) = source.ts_val();
        let (source_start, source_end) = (source_start?, source_end?);
        let (target_start, target_end) = target.ts_val();
        let (target_start, target_end) = (target_start?, target_end?);

        let overlap_start_start = source_start.cmp(&target_start);
        let overlap_start_end = source_start.cmp(&target_end);
        let overlap_end_start = source_end.cmp(&target_start);
        let overlap_end_end = source_end.cmp(&target_end);

        // intersect

        match (overlap_start_start, overlap_start_end, overlap_end_start, overlap_end_end) {
            (Ordering::Less, .., Ordering::Greater) => Some(Self::Surrounds(target_start-source_start, target_end-source_end)),
            (Ordering::Greater, .., Ordering::Less) => Some(Self::Contained(target_start-source_start, target_end-source_end)),
            // source's start time is contained within the target's boundaries
            // or equal, but source end time is larger
            (Ordering::Greater, Ordering::Less, ..)
            | (Ordering::Greater, Ordering::Equal, ..)
            | (Ordering::Equal, Ordering::Less, ..) => Some(Self::Start(target_start-source_start)),
            // source's end time is contained within the target's boundaries
            // or equal, but source start time is
            (.., Ordering::Greater, Ordering::Less)
            | (.., Ordering::Greater, Ordering::Equal)
            | (.., Ordering::Equal, Ordering::Less) => Some(Self::End(target_end-source_end)),
            (Ordering::Equal, .., Ordering::Equal) => Some(Self::Equal),
            _ => None,
            // (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Less, Ordering::Greater, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Less, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Equal, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Equal, Ordering::Greater, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Equal, Ordering::Greater, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Greater) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Greater, Ordering::Less) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Greater, Ordering::Equal) => todo!(),
            // (Ordering::Greater, Ordering::Greater, Ordering::Greater, Ordering::Greater) => todo!(),
        }
    }

    pub fn intersects(source: &Annotation, target: &Annotation) -> bool {
        Self::resolve(source, target).is_some()
    }
}
