//! Pfsx preference value.

use serde::{Deserialize, Serialize};

use super::{pref_group::PrefGroup, pref_list::PrefList, pref::Pref};

/// Pfsx preference value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PrefValue {
    #[serde(rename = "pref")]
    Pref(Pref),
    #[serde(rename = "prefList")]
    PrefList(PrefList),
    #[serde(rename = "prefGroup")]
    PrefGroup(PrefGroup),
}