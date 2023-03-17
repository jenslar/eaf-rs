use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(rename = "pref")]
pub struct Pref {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "$value")]
    value: Value
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "prefGroup")]
pub struct PrefGroup {
    #[serde(rename="pref")]
    preferences: Vec<Pref>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "prefList")]
pub struct PrefList {
    #[serde(rename="$value")]
    preferences: Vec<Value>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Prefs {
    #[serde(rename = "pref", default)]
    preferences: Vec<Pref>,
    #[serde(rename="prefGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pref_group: Option<PrefGroup>,
    #[serde(rename="prefList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pref_list: Option<PrefList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Value {
    Boolean(bool), //" type="booleanType"/>
    Int(i32), //" type="intType"/>
    Long(i64), //" type="longType"/>
    Float(f32), //" type="floatType"/>
    Double(f64), //" type="doubleType"/>
    String(String), //" type="stringType"/>
    Object(Object), //" type="objectType"/>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Object {
    #[serde(rename = "@class")]
    class: String,
    #[serde(rename = "$value")]
    value: String
}