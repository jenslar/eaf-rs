//! Time order.
//! 
//! Map of time slot ID to time values (milliseconds).

use std::collections::HashMap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Serialize, Deserialize};

use crate::{EafError, Annotation};

use super::TimeSlot;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimeOrder {
    #[serde(rename = "TIME_SLOT", default)]
    pub time_slots: Vec<TimeSlot>
}

impl TimeOrder {
    // pub fn new(time_slots: &[TimeSlot]) -> Self { // TODO remove borrow, should take Vec<TimeSlot>
    //     TimeOrder{time_slots: time_slots.to_owned()}
    // }
    pub fn new() -> Self {
        TimeOrder::default()
    }

    // /// Pushes a time slot at the end of time slot list,
    // /// unless the ID already exists.
    // /// 
    // /// Timeslots may have the same time value.
    // pub fn add_old(&mut self, id: &str, val: Option<i64>) -> Result<(), EafError> {
    //     // if !self.id_exists(&id) {
    //     if self.id_exists(&id) {
    //         return Err(EafError::TimeSlotIdExists(id.to_owned()))
    //     }
    //     Ok(self.time_slots.push(TimeSlot::new(id, val)))
    // }

    /// Generates a new time slot, and adds this to time order.
    /// Returns the time slot ID.
    pub fn add(&mut self, id: Option<&str>, time_value: Option<i64>) -> Result<String, EafError> {
        let id = match id {
            Some(i) => i.to_owned(),
            None => Self::gen_id(&self)
        };

        if self.id_exists(&id) {
            return Err(EafError::TimeSlotIdExists(id.to_owned()))
        }

        self.time_slots.push(TimeSlot::new(&id, time_value));

        Ok(id)
    }

    /// Generates multiple new time slots, and adds these to time order.
    /// Returns the time slot IDs.
    pub fn add_multi(&mut self, ids: &[String], time_values: Vec<Option<i64>>) -> Vec<String> {
        let ids = match ids.is_empty() {
            true => Self::gen_id_multi(&self, time_values.len()),
            false => ids.to_owned()
        };
        let ts = time_values.iter().zip(&ids)
            .map(|(t, i)| TimeSlot::new(&i, *t))
            .collect::<Vec<_>>();

        self.time_slots.extend(ts);

        ids
    }

    /// Add a single time slot to time order.
    /// 
    /// Does not check whether the time slot
    /// already exists. Use `TimeSlot::add()` to
    /// generate and add a new, unique time slot.
    pub fn push(&mut self, time_slot: &TimeSlot) {
        self.time_slots.push(time_slot.to_owned())
    }

    /// Extends existing time order with time slots.
    /// 
    /// Does not check whether the time slots
    /// already exist. Use `TimeSlot::add_multi()` to
    /// generate and add a new, unique time slots.
    pub fn extend(&mut self, time_slots: Vec<TimeSlot>) {
        self.time_slots.extend(time_slots)
    }

    /// Joins one time order with another.
    pub fn join(&mut self, time_order: &TimeOrder) {
        self.time_slots.extend(time_order.time_slots.to_owned())
    }

    /// Generates a new time order from vector containing millisecond time values.
    /// The optional `start_index` corresponds to e.g. the numerical value `45` in "ts45".
    /// Note that time slot values are optional for EAF, hence `Vec<Option<i64>>`.
    pub fn from_values(time_values: Vec<Option<i64>>, start_index: Option<usize>) -> Self {
        let start = start_index.unwrap_or(0);
        TimeOrder {
            time_slots: time_values.iter()
                .enumerate()
                .map(|(i,t)| 
                    TimeSlot::new(&format!("ts{}", start+i+1), t.to_owned())
                )
                .collect()
        }
    }

    /// Generates a new time order from hashmap where key = numerical index for time slot ID (`2` in "ts2"),
    /// and value = time slot millisecond value.
    /// The optional `start_index` corresponds to e.g. the numerical value `45` in "ts45".
    /// Note that time slot values are optional in EAF, hence `Option<i64>`.
    pub fn from_hashmap(id2values: HashMap<String, Option<i64>>) -> Self {
        let mut time_slots: Vec<TimeSlot> = id2values.iter()
            .map(|(ts_id, ts_val)| TimeSlot::new(&ts_id, *ts_val))
            .collect();

        time_slots
            .sort_by_key(|t|
                t.time_slot_id
                    .replace("ts", "")
                    .parse::<usize>()
                    .ok()
            );

        TimeOrder{time_slots}
    }

    /// Returns number of time slots.
    pub fn len(&self) -> usize {
        self.time_slots.len()
    }

    /// Returns true if there are no time slots.
    pub fn is_empty(&self) -> bool {
        self.time_slots.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TimeSlot> {
        self.time_slots.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = TimeSlot> {
        self.time_slots.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TimeSlot> {
        self.time_slots.iter_mut()
    }

    pub fn first(&self) -> Option<&TimeSlot> {
        self.time_slots.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut TimeSlot> {
        self.time_slots.first_mut()
    }
    
    pub fn last(&self) -> Option<&TimeSlot> {
        self.time_slots.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut TimeSlot> {
        self.time_slots.last_mut()
    }

    /// Returns the time slot with minimum millisecond value,
    /// or `None` if time order is empty or contains no time values.
    pub fn min(&self) -> Option<&TimeSlot> {
        self.iter()
            .filter(|ts| ts.time_value.is_some())
            .min_by_key(|ts| ts.time_value)
    }

    /// Returns minimum millisecond time value or `None`
    /// if time order is empty or contains no time values.
    pub fn min_val(&self) -> Option<i64> {
        self.iter()
            .filter_map(|t| t.time_value)
            .min()
    }

    /// Returns numerical component in "minimum" time slot ID.
    /// E.g. "1" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn min_id_num(&self) -> Option<i64> {
        self.iter()
            .filter_map(|ts|
                ts.time_slot_id
                    .trim_start_matches("ts")
                    .parse::<i64>().ok()
                )
            .min()
    }

    /// Returns "minimum" time slot ID,
    /// e.g. "ts1" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn min_id(&self) -> Option<String> {
        Some(format!("ts{}", self.min_id_num()?))
    }

    /// Returns the time slot with maximum millisecond value,
    /// or `None` if time order is empty or contains no time values.
    pub fn max(&self) -> Option<&TimeSlot> {
        self.iter()
            .filter(|ts| ts.time_value.is_some())
            .max_by_key(|ts| ts.time_value)
    }

    /// Returns maximum millisecond time value or `None`
    /// if time order is empty or contains no time values.
    pub fn max_val(&self) -> Option<i64> {
        self.iter()
            .filter_map(|t| t.time_value)
            .max()
    }

    /// Returns numerical component in "maximum" time slot ID.
    /// E.g. "3" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn max_id_num(&self) -> Option<i64> {
        self.iter()
            .filter_map(|ts|
                ts.time_slot_id
                    .trim_start_matches("ts")
                    .parse::<i64>().ok()
                )
            .max()
    }

    /// Returns "maximum" time slot ID,
    /// e.g. "ts3" will be returned if
    /// the existing IDs are "ts1", "ts2", "ts3".
    pub fn max_id(&self) -> Option<String> {
        Some(format!("ts{}", self.max_id_num()?))
    }

    /// Returns reference to `TimeSlot` with specified ID,
    /// or `None`if it does not exist.
    pub fn find(&self, time_slot_id: &str) -> Option<&TimeSlot> {
        self.time_slots.par_iter()
            .find_any(|t| t.time_slot_id == time_slot_id)
    }

    pub fn id_exists(&self, time_slot_id: &str) -> bool {
        self.find(time_slot_id).is_some()
    }

    /// Generate new numerical component of a time slot ID
    /// that follows "ts<NUMBER>", e.g. "ts231".
    /// 
    /// Note that other patterns will be ignored,
    /// but that this will not impact the uniquness
    /// of the ID, which is all that matters.
    /// 
    /// Increments on current max value.
    pub fn gen_id_idx(&self) -> i64 {
        if let Some(val) = self.max_id_num() {
            val + 1
        } else {
            1
        }
    }

    /// Generate multiple new numerical components of a time slot ID
    /// that follows "ts<NUMBER>", e.g. "ts231".
    /// 
    /// Note that other patterns will be ignored,
    /// but that this will not impact the uniquness
    /// of the ID, which is all that matters.
    /// 
    /// Increments on current max value.
    pub fn gen_id_idx_multi(&self, len: usize) -> Vec<i64> {
        if let Some(val) = self.max_id_num() {
            (val + 1 .. val + 1 + len as i64).into_iter().collect()
        } else {
            (1 .. 1 + len as i64).into_iter().collect()
        }
    }

    /// Generate new time slot ID.
    /// 
    /// Increments on current "max" ID.
    pub fn gen_id(&self) -> String {
        return format!("ts{}", self.gen_id_idx())
    }

    /// Generate multiple new time slot IDs.
    /// 
    /// Increments on current "max" ID.
    pub fn gen_id_multi(&self, len: usize) -> Vec<String> {
        self.gen_id_idx_multi(len)
            .iter()
            .map(|i| format!("ts{}", i))
            .collect()
    }

    /// Remaps time slot ID:s starting on 1, or `start_index`.
    /// Returns hashmap mapping old time slot ID:s to new ones.
    pub fn remap(&mut self, start_index: Option<usize>) -> HashMap<String, String> {
        let mut old2new: HashMap<String, String> = HashMap::new();

        let ts_start_index = start_index.unwrap_or(0);
        
        self.time_slots.iter_mut()
            .enumerate()
            .for_each(|(i,t)| {
                let ts_id_new = format!("ts{}", i+1+ts_start_index);
                old2new.insert(t.time_slot_id.to_owned(), ts_id_new.to_owned());
                t.time_slot_id = ts_id_new;
            });
            
        old2new
    }

    /// Lookup table for time slots.
    /// Time slots with no time value return `None`.
    /// - Key: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    /// - Value: timeslot value in milliseconds.
    pub fn index(&self) -> HashMap<String, Option<i64>> {
        self.time_slots.to_owned().into_iter()
            .map(|ts| (ts.time_slot_id, ts.time_value))
            .collect()
    }
    
    /// Reverse lookup table for time slots.
    /// Important: Only includes time slots with a corresponding time value.
    /// - Key: timeslot value in milliseconds.
    /// - Value: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    pub fn index_rev(&self) -> HashMap<i64, String> {
        self.time_slots.to_owned().into_iter()
            .filter_map(|t|
                if let Some(val) = t.time_value {
                    Some((val, t.time_slot_id))
                } else {
                    None
                }
            )
            .collect()
    }

    /// Shift all time values with the specified value in milliseconds.
    /// `allow_negative` ignores negative time values, otherwise
    /// `EafError::ValueTooSmall(time_value)` is raised.
    pub fn shift(&mut self, shift_ms: i64, allow_negative: bool) -> Result<(), EafError> {
        // Check negative values.
        if !allow_negative {
            let min = match self.min_val() {
                Some(val) => val,
                None => 0
            };

            // if min - shift_ms < 0 {
            // ensure negative shift values are not less than 0
            if min + shift_ms < 0 {
                return Err(EafError::ValueTooSmall(shift_ms))
            }
        }

        // shift values
        self.iter_mut()
            .for_each(|ts| if let Some(val) = ts.time_value {
                ts.time_value = Some(val + shift_ms)
            });

        Ok(())
    }

    // /// Returns `TimeOrder` containing only time slots between, and including,
    // /// millisecond values `start` and `end`. Start and end values may for example
    // /// correspond to new media boundaries when a clip has been extracted from a larger
    // /// media file.
    // /// 
    // /// Note that only time slots with a millisecond value can act as the first or
    // /// final time slot for the specified time span. Between the first and the final
    // /// time slot all time slots will be included, including those without a time value.
    // pub fn filter_old(&self, start: i64, end: i64) -> Option<Self> {
    // // pub fn filter(&self, start: u64, end: u64) -> Option<TimeOrder> {
    //     // Generate hashmap: time slot value (ms) -> time slot id (String>).
    //     // Need time slot values to find start/end, hence `index_rev()`.
    //     // let filtered: HashMap<u64, String> = self.index_rev()
    //     let filtered: HashMap<i64, String> = self.index_rev()
    //         .into_iter()
    //         .filter(|(t, _)| t >= &start && t <= &end)
    //         .collect();

    //     // Need to get around time slot values being optional,
    //     // hence the back and forth below. A simple time value
    //     // comparison would discard time slots with no time value.
            
    //     // Get time slot ID for min/max time slot values.
    //     let id_min = filtered.keys().min()
    //         .and_then(|min| filtered.get(min))?;
    //     let id_max = filtered.keys().max()
    //         .and_then(|max| filtered.get(max))?;

    //     println!("TS ID MIN: {id_min}");
    //     println!("TS ID MAX: {id_max}");

    //     // !!! unless timeslots are ordered on time value below
    //     // !!! indeces will not reflect a "chronologically ordered slice"
    //     // Indeces for slice containing `TimeSlot`:s within time span.
    //     let idx1 = self.time_slots.iter()
    //         .position(|t| &t.time_slot_id == id_min)?;
    //     let idx2 = self.time_slots.iter()
    //         .position(|t| &t.time_slot_id == id_max)?;

    //     println!("TS LEN BEFORE: {}", self.time_slots.len());
        
    //     let time_slots = self.time_slots[idx1 ..= idx2].iter()
    //         .map(|t|
    //             if let Some(val) = t.time_value {
    //                 TimeSlot {
    //                     time_value: Some(val - start),
    //                     ..t.to_owned()
    //                 }
    //             } else {
    //                 t.to_owned()
    //             }
    //         )
    //         .collect::<Vec<TimeSlot>>();
        
    //     println!("TS LEN AFTER: {}", time_slots.len());
        
    //     Some(Self{time_slots})
    // }

    /// Returns a new time order containing only time slots between, and including,
    /// millisecond values `start` and `end`.
    /// 
    /// Note that time slots without a millisecond value will be discarded,
    /// since there is no reliable way to determine whether these exist
    /// within the specified timespan.
    pub fn filter(&self, start_ms: i64, end_ms: i64) -> Self {
        Self {
            time_slots: self.time_slots.iter()
               .filter(|ts| ts.contained_in(start_ms, end_ms))
               .cloned()
               .collect(),
            ..self.to_owned()
        }
    }

    /// Edit the first time slot value, if any exist.
    pub fn set_first(&mut self, time_slot_value: i64) {
        if let Some(ts) = self.first_mut() {
            ts.time_value = Some(time_slot_value);
        }
    }

    /// Edit the last time slot value, if any exist.
    pub fn set_last(&mut self, time_slot_value: i64) {
        if let Some(ts) = self.last_mut() {
            ts.time_value = Some(time_slot_value);
        }
    }

    /// Returns `true` if the exact time slot already exists
    /// (ID and time value).
    pub fn contains(&self, time_slot: &TimeSlot) -> bool {
        self.time_slots.contains(time_slot)
    }

    /// Returns `true` if the time slot ID already exists.
    pub fn contains_id(&self, time_slot_id: &str) -> bool {
        self.time_slots.iter().any(|t| t.time_slot_id == time_slot_id)
    }

    /// Generates time order from annotations.
    pub fn from_annotations(annotations: &[Annotation]) -> Result<Self, EafError> {
        Ok(Self {
            time_slots: annotations.par_iter()
                .map(|a| {
                    if let Some((tr1, tr2)) = a.ts_ref() {
                        let (tv1, tv2) = a.ts_val();
                        Ok([TimeSlot::new(&tr1, tv1), TimeSlot::new(&tr2, tv2)])
                    } else {
                        Err(EafError::TimeslotRefsMissing)
                    }
                })
                .collect::<Result<Vec<_>, EafError>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        })
    }
}
