//! External reference.

use serde::{Deserialize, Serialize};

/// An element that represents a reference to an external entity.
/// There are a number of predefined entity types as part of the schema.
///
/// Attributes:
/// - `ext_ref_id`: the id of the element
/// - `type`: the type of the external entity, one of the following:
///     - `iso12620`: the id of an ISO Data Category
///     - `ecv`: an external (closed) controlled vocabulary
///     - `cve_id`: reference to the id of an entry in an external Controlled Vocabulary
///     - `lexen_id`: reference to the id of a lexical entry
///     - `resource_url`: a url or hyperlink to any type of document
/// - `value` - the value of the element, the interpretation of the value depends on the type
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "EXTERNAL_REF")]
pub struct ExternalRef {
    #[serde(rename = "@EXT_REF_ID")]
    pub ext_ref_id: String,
    #[serde(rename = "@TYPE")]
    pub ref_type: String, // should ideally be ReferenceType enum
    #[serde(rename = "@VALUE")]
    pub value: String,
}

pub enum ReferenceType {
    Iso12620,
    Ecv,
    CveId,
    LexenId,
    ResourceUrl,
}

// impl From<String> for Option<ReferenceType> {
//     fn from(value: String) -> Option<ReferenceType> {
//         match value.as_str() {
//             "iso12620" => Some(ReferenceType::Iso12620),
//             "ecv" => Some(ReferenceType::Ecv),
//             "cve_id" => Some(ReferenceType::CveId),
//             "lexen_id" => Some(ReferenceType::LexenId),
//             "resource_url" => Some(ReferenceType::ResourceUrl),
//             _ => None
//         }
//     }
// }
