//! Time order.
//! 
//! Map of time slot IDs to time values (milliseconds).

mod timeorder;
mod timeslot;
mod builder;

pub use timeorder::TimeOrder;
pub use timeslot::TimeSlot;
pub use builder::TimeOrderBuilder;
