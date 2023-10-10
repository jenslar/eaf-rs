use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Object {
    #[serde(rename = "@class")]
    class: String,
    #[serde(rename = "$value")]
    value: String
}