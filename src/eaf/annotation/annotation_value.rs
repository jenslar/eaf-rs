use std::str::Chars;

use serde::{Deserialize, Serialize};

/// Annotation value.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename = "ANNOTATION_VALUE")]
pub struct AnnotationValue(String);

impl std::fmt::Display for AnnotationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AnnotationValue {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&str> for AnnotationValue {
    fn from(value: &str) -> Self {
        AnnotationValue(value.to_owned())
    }
}

// impl From<String> for AnnotationValue {
//     fn from(value: String) -> Self {
//         AnnotationValue(value)
//     }
// }

// impl From<&AnnotationValue> for String {
//     fn from(value: &AnnotationValue) -> String {
//         value.0.to_owned()
//     }
// }

impl AnnotationValue {
    /// Returns an iterator over the characters for the annotation value.
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }
    
    /// Returns character count for the annotation value.
    pub fn count(&self) -> usize {
        self.0.chars().count()
    }

    pub fn len(&self) -> usize {
        self.split(None).len()
    }
    
    /// Splits the annotation value on white space or specified pattern.
    pub fn split(&self, pattern: Option<&str>) -> Vec<&str> {
        match pattern {
            Some(p) => self.0.split(p).collect(),
            None => self.0.split_whitespace().collect(),
        }
    }

    /// Replace string pattern for annotation value.
    pub fn replace(&self, from: &str, to: &str) -> Self {
        Self(self.0.replace(from, to))
    }

    /// Mutably replace string pattern for annotation value.
    pub fn replace_mut(&mut self, from: &str, to: &str) {
        self.0 = self.0.replace(from, to)
    }
}