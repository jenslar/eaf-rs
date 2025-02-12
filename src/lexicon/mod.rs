//! ELAN Lexicon component: <http://www.mpi.nl/tools/elan/LexiconComponent-1.0.xsd>

pub mod entry;
pub mod lexicon;
pub mod header;

pub use lexicon::Lexicon;
pub use header::LexiconHeader;
pub use entry::LexiconEntry;