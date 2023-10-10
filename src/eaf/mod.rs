//! The core data structure for a deserialized
//! [ELAN-file](https://www.mpi.nl/tools/elan/EAF_Annotation_Format_3.0_and_ELAN.pdf).
//! 
//! Example:
//! ```
//! use eaf_rs::Eaf;
//! fn main() -> std::io::Result<()> {
//!     let path = std::path::Path::new("MYEAF.eaf");
//!     // Deserialize
//!     let eaf = Eaf::read(&path)?;
//!     println!("{:#?}", eaf);
//!     Ok(())
//! }
//! ```
//!
//! Note that some methods expect `Eaf::index()` and `Eaf::derive()`
//! to be called before they are run. This is done automatically for most methods and on deserialization.
//! `Eaf::index()` indexes the EAF speeding up many "getter" methods,
//! whereas and `Eaf::derive()` derives values such as time values
//! for annotation boundaries and sets these directly at the annotation level to make them more independent.

pub mod eaf;
pub mod errors;
pub mod license;
pub mod header;
pub mod media_descriptor;
pub mod linked_file_descriptor;
pub mod property;
pub mod timeorder;
pub mod tier;
pub mod annotation;
pub mod linguistic_type;
pub mod constraint;
pub mod language;
pub mod lexicon_ref;
pub mod index;
pub mod locale;
pub mod controlled_vocabulary;
pub mod json;

pub use eaf::{Eaf, Scope};
pub use errors::EafError;
pub use license::License;
pub use header::Header;
pub use media_descriptor::MediaDescriptor;
pub use linked_file_descriptor::LinkedFileDescriptor;
pub use property::Property;
pub use timeorder::{TimeOrder, TimeSlot};
pub use tier::Tier;
pub use annotation::Annotation;
pub use linguistic_type::LinguisticType;
pub use constraint::{Constraint, StereoType};
pub use language::Language;
pub use lexicon_ref::LexiconRef;
pub use index::Index; // should perhaps not be public
pub use locale::Locale;
pub use controlled_vocabulary::ControlledVocabulary;
pub use json::{JsonAnnotation, JsonEaf, JsonTier};