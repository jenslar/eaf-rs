//! Constraint.

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

    pub fn from_string(stereotype: &String) -> Self {
        StereoType::from_string(stereotype).to_constraint()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum StereoType {
    #[serde(rename = "Included_In")]
    IncludedIn, // time alignable: true
    #[serde(rename = "Symbolic_Association")]
    SymbolicAssociation, // time alignable: true
    #[serde(rename = "Symbolic_Subdivision")]
    SymbolicSubdivision, // time alignable: true
    #[serde(rename = "Time_Subdivision")]
    TimeSubdivision, // time alignable: true
}

impl From<String> for StereoType {
    fn from(stereotype: String) -> Self {
        match stereotype.as_str() {
            "Included_In" => Self::IncludedIn,
            "Time_Subdivision" => Self::TimeSubdivision,
            "Symbolic_Subdivision" => Self::SymbolicSubdivision,
            "Symbolic_Association" => Self::SymbolicAssociation,
            s => panic!("(!) No such stereotype '{}'", s) // TODO return Result instead?
        }
    }
}

impl Into<String> for StereoType {
    fn into(self) -> String {
        match &self {
            Self::IncludedIn=> "Included_In".to_owned(),
            Self::TimeSubdivision=> "Time_Subdivision".to_owned(),
            Self::SymbolicSubdivision=> "Symbolic_Subdivision".to_owned(),
            Self::SymbolicAssociation=> "Symbolic_Association".to_owned(),
        }
    }
}

impl StereoType {
    pub fn to_constraint(&self) -> Constraint {
        match &self {
            Self::IncludedIn => Constraint{
                description: "Time alignable annotations within the parent annotation's time interval, gaps are allowed".to_owned(),
                stereotype: StereoType::IncludedIn},
                // stereotype: "Included_In".to_owned()},
            Self::SymbolicSubdivision => Constraint{
                description: "Symbolic subdivision of a parent annotation. Annotations refering to the same parent are ordered".to_owned(),
                stereotype: StereoType::SymbolicSubdivision},
                // stereotype: "Symbolic_Subdivision".to_owned()},
            Self::SymbolicAssociation => Constraint{
                description: "1-1 association with a parent annotation".to_owned(),
                stereotype: StereoType::SymbolicAssociation},
                // stereotype: "Symbolic_Association".to_owned()},
            Self::TimeSubdivision => Constraint{
                description: "Time subdivision of parent annotation's time interval, no time gaps allowed within this interval".to_owned(),
                stereotype: Self::TimeSubdivision},
                // stereotype: "Time_Subdivision".to_owned()},
        }
    }

    /// Checks whether a constraint is time alignable.
    pub fn time_alignable(&self) -> bool {
        match &self {
            StereoType::IncludedIn | StereoType::TimeSubdivision => true,
            StereoType::SymbolicAssociation | StereoType::SymbolicSubdivision => false,
        }
    }

    pub fn from_string(stereotype: &str) -> Self {
        match stereotype {
            "Included_In" => Self::IncludedIn,
            "Time_Subdivision" => Self::TimeSubdivision,
            "Symbolic_Subdivision" => Self::SymbolicSubdivision,
            "Symbolic_Association" => Self::SymbolicAssociation,
            s => panic!("(!) No such stereotype '{}'", s) // TODO return Result instead?
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Self::IncludedIn => "Included_In".to_owned(),
            Self::TimeSubdivision => "Time_Subdivision".to_owned(),
            Self::SymbolicSubdivision => "Symbolic_Subdivision".to_owned(),
            Self::SymbolicAssociation => "Symbolic_Association".to_owned(),
        }
    }
}

