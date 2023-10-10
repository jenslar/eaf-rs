//! Time slot, part of time order.
//! 
//! Maps a time slot ID to a time value (milliseconds).

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimeSlot {
    #[serde(rename = "@TIME_SLOT_ID")]
    pub time_slot_id: String,
    #[serde(rename = "@TIME_VALUE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_value: Option<i64>
}

impl TimeSlot {
    /// Creates a new time slot from time slot ID and optional millisecond value.
    /// 
    /// ELAN's convention is to format the  `"ts1"`, `"ts2"`, ..., `"ts23"`, ...,
    /// `"ts10234"`, etc, with no leading zeros.
    pub fn new(id: &str, val: Option<i64>) -> Self {
        TimeSlot {
            time_slot_id: id.to_owned(),
            time_value: val,
        }
    }

    /// Returns `true` if the a time value specified.
    pub fn has_val(&self) -> bool {
        self.time_value.is_some()
    }
}