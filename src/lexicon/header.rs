//! ELAN Lexicon component: <http://www.mpi.nl/tools/elan/LexiconComponent-1.0.xsd>
//! NOT YET IMPLEMENTED


use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexiconHeader {
    name: String,
    language: String,
    version: String,
    #[serde(rename="custom-fields")]
    custom_fields: Vec<LexiconHeaderField>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename="field-spec")]
pub struct LexiconHeaderField {
    #[serde(rename="@name")]
    name: String,
    #[serde(rename="@level")]
    level: String,
}