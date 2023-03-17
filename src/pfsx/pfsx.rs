use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::EafError;

use super::pref::{Pref, PrefGroup, PrefList, Prefs};

/// Default for top-level attribute `xmlns:xsi`.
pub fn xmlns_xsi() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".to_owned()
}

/// Default for top-level attribute `xsi:noNamespaceSchemaLocation`.
/// [ETFv1.1](http://www.mpi.nl/tools/elan/Prefs_v1.1.xsd).
pub fn xsi_no_name_space_schema_location() -> String {
    "http://www.mpi.nl/tools/elan/Prefs_v1.1.xsd".to_owned()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "preferences")]
pub struct Pfsx {
    #[serde(rename = "@version")]
    version: String,
    /// Default namespace.
    /// ELAN (and Pfsx schema) accepts this out of order,
    /// quick-xml does not, hence the default.
    #[serde(rename = "@xmlns:xsi", default="xmlns_xsi")]
    xmlns_xsi: String,
    /// Schema location.
    /// ELAN (and Pfsx schema) accepts this out of order,
    /// quick-xml does not, hence the default.
    #[serde(rename = "@xsi:noNamespaceSchemaLocation", default="xsi_no_name_space_schema_location")]
    xsi_nonamespaceschemalocation: String,
    // TODO below doesn't work since pfsx mixes pref + prefGroup + prefList
    // TODO in any order and number
    #[serde(rename = "$unflatten", default)]
    preferences: Vec<Prefs>,

    // #[serde(rename = "pref", default)]
    // preferences: Vec<Pref>,
    
    // #[serde(rename="prefList")]
    // pref_list: Vec<PrefList>,

    // #[serde(rename = "prefGroup")]
    // pref_group: Vec<PrefGroup>,
}

impl Pfsx {
    /// Deserialize ELAN preferences XML-file (`pfsx`).
    pub fn deserialize(path: &Path) -> Result<Self, EafError> {
        // Let Quick XML use serde to deserialize
        quick_xml::de::from_str::<Pfsx>(&std::fs::read_to_string(path)?)
            .map_err(|e| EafError::QuickXMLDeError(e))
    }
}