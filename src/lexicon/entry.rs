//! ELAN Lexicon component: <http://www.mpi.nl/tools/elan/LexiconComponent-1.0.xsd>

use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexiconEntry {
    // Entry attributes
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@dateCreated")]
    date_created: String,
    #[serde(rename = "@dateModified")]
    date_modified: String,
    #[serde(rename = "@order")]
    order: String,

    #[serde(rename = "lexical-unit")]
    lexical_unit: String,
    citation: Option<String>,
    #[serde(rename = "morph-type")]
    morph_type: Option<String>,
    #[serde(rename = "variant")]
    variants: Option<Vec<String>>,
    #[serde(rename = "phonetic")]
    phonetics: Option<Vec<String>>, // not fully supported yet according to xsd
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Variant {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sense {

}