//! Linguistic Type.
//! 
//! Together with Constraint and StereoType, Linguistic Type determines
//! the characteristics of an annotation, such as whether it is time alignable
//! (only applies to referred annotations).

mod constraint;
mod linguistic_type;
mod stereotype;

pub use constraint::Constraint;
pub use linguistic_type::LinguisticType;
pub use stereotype::StereoType;