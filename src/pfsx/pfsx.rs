use std::{path::Path, io::Write, fs::File};

use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

use crate::EafError;

use super::pref_value::PrefValue;

/// Default for top-level attribute `xmlns:xsi`.
pub fn xmlns_xsi() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".to_owned()
}

/// Default for top-level attribute `xsi:noNamespaceSchemaLocation`.
/// [ETFv1.1](http://www.mpi.nl/tools/elan/Prefs_v1.1.xsd).
pub fn xsi_no_name_space_schema_location() -> String {
    "http://www.mpi.nl/tools/elan/Prefs_v1.1.xsd".to_owned()
}

/// Represents the ELAN preferences XML-file (`.pfsx`).
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

    /// Pfsx data.
    #[serde(rename = "$value", default)]
    preferences: Vec<PrefValue>,
}

impl Default for Pfsx {
    fn default() -> Self {
        Self {
            version: "1.1".to_owned(),
            xmlns_xsi: xmlns_xsi(),
            xsi_nonamespaceschemalocation: xsi_no_name_space_schema_location(),
            preferences: Default::default()
        }
    }
}

impl Pfsx {    
    /// Deserialize ELAN preferences XML-file (`pfsx`).
    fn de(path: &Path) -> Result<Self, EafError> {
        // Let Quick XML use serde to deserialize
        quick_xml::de::from_str::<Pfsx>(&std::fs::read_to_string(path)?)
            .map_err(|e| EafError::QuickXMLDeError(e))
    }

    /// Serialize to string (internal).
    fn se(&self, indent: Option<usize>) -> Result<String, EafError> {
        let mut pfsx = self.to_owned(); // better to take &mut self as arg...?

        // Should already be set for deserialized EAF:s.
        pfsx.set_ns();

        let mut eaf_str = String::new();
        // Create serializer that indents XML with 4 spaces
        let mut ser = Serializer::new(&mut eaf_str);
        if let Some(ind) = indent {
            ser.indent(' ', ind);
        }

        pfsx.serialize(ser).map_err(|e| EafError::QuickXMLDeError(e))?;

        Ok([
            // Add XML declaration, since not added by quick-xml
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            eaf_str.as_str()
        ].join("\n"))
    }

    /// Set Pfxs XML namespaces.
    fn set_ns(&mut self) {
        self.xmlns_xsi = xmlns_xsi();
        self.xsi_nonamespaceschemalocation =
            format!("http://www.mpi.nl/tools/elan/Prefs_v{}.xsd", self.version);
    }

    /// Read and deserialize pfsx-file.
    pub fn read(path: &Path) -> Result<Pfsx, EafError> {
        Self::de(path)
    }

    /// Serialize and write pfsx-file to disk.
    pub fn write(&self, path: &Path, indent: Option<usize>) -> Result<(), EafError> {
        let content = self.se(indent)?;

        let mut outfile = File::create(&path)?;
        outfile.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Serialize to string.
    pub fn to_string(&self, indent: Option<usize>) -> Result<String, EafError> {
        self.se(indent)
    }

    /// Returns preference values for AAM-LR Phone level audio segmentation.
    pub fn aam_lr(&self) -> Vec<&PrefValue> {
        let key = "AAM-LR Phone level audio segmentation";
        self.preferences.iter()
            .filter(|prefval| if let PrefValue::PrefGroup(grp) = prefval {
                grp.key == key
            } else {
                false
            })
            .collect()
    }
}