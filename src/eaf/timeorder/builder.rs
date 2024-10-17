use std::collections::HashSet;

use crate::{TimeSlot, TimeOrder, EafError, eaf::ts_duplicates};

pub struct TimeOrderBuilder {
    timeslots: Vec<TimeSlot>
}

impl TimeOrderBuilder {
    pub fn new() -> Self {
        Self { timeslots: Vec::default() }
    }

    /// Add time slot (to existing time slots if there are any).
    pub fn timeslot(self, timeslot: TimeSlot) -> Self {
        Self {
            timeslots: self.timeslots.into_iter()
                .chain(std::iter::once(timeslot))
                .collect::<Vec<_>>()
        }
    }

    /// Add multiple time slots (to existing time slots if there are any).
    pub fn timeslots(self, timeslots: Vec<TimeSlot>) -> Self {
        Self {
            timeslots: self.timeslots.into_iter()
                .chain(timeslots).collect()
        }
    }

    /// Build `TimeOrder`.
    pub fn build(self) -> Result<TimeOrder, EafError> {
        if ts_duplicates(&self.timeslots) {
            return Err(EafError::TimeslotIDDuplicated)
        }

        Ok(TimeOrder {
            time_slots: self.timeslots
        })
    }
}