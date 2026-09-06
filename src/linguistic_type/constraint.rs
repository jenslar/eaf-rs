//! Constraint.

use serde::{Serialize, Deserialize};

use super::StereoType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub struct Constraint {
    #[serde(rename="@DESCRIPTION")]
    pub description: String,
    #[serde(rename="@STEREOTYPE")]
    pub stereotype: StereoType,
}

impl Constraint {
    pub fn from_stereotype(stereotype: &StereoType) -> Self {
        stereotype.to_constraint()
    }

    pub fn defaults() -> Vec<Self> {
        vec![
            Self::from_stereotype(&StereoType::TimeSubdivision),
            Self::from_stereotype(&StereoType::SymbolicSubdivision),
            Self::from_stereotype(&StereoType::IncludedIn),
        ]
    }

    pub fn from_string(stereotype: &String) -> Self {
        StereoType::from_string(stereotype).to_constraint()
    }
}
