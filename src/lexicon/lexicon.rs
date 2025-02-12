//! ELAN Lexicon component: <http://www.mpi.nl/tools/elan/LexiconComponent-1.0.xsd>

use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

use super::{LexiconEntry, LexiconHeader};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lexicon {
    header: LexiconHeader,
    entries: Vec<LexiconEntry>,
}
