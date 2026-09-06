use serde::{Deserialize, Serialize};

use super::object::Object;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pref {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "$value")]
    value: Value
}

impl Pref {
    pub fn new(key: &str, value: &Value) -> Pref {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Value {
    Boolean(bool),  // XSD type = "booleanType"
    Int(i32),       // XSD type = "intType"
    Long(i64),      // XSD type = "longType"
    Float(f32),     // XSD type = "floatType"
    Double(f64),    // XSD type = "doubleType"
    String(String), // XSD type = "stringType"
    Object(Object), // XSD type = "objectType"
}
