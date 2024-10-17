//! Annotation value.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io::{copy, Read}, str::Chars};
use unicode_segmentation::{UnicodeSegmentation, Graphemes};

/// Annotation value.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename = "ANNOTATION_VALUE")]
pub struct AnnotationValue(String);

impl std::fmt::Display for AnnotationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Read for AnnotationValue {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::Cursor::new(self.to_string()).read(buf)
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

impl From<String> for AnnotationValue {
    fn from(value: String) -> Self {
        AnnotationValue(value)
    }
}

impl AnnotationValue {
    /// Returns an iterator over the characters for the annotation value.
    /// May produce different results that `AnnotationValue::graphemes()`
    /// depending on writing system.
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }

    /// Returns character count for the annotation value.
    pub fn char_count(&self) -> usize {
        self.0.chars().count()
    }

    /// Returns an iterator over the graphemes for the annotation value.
    /// May produce different results that `AnnotationValue::chars()`
    /// depending on writing system.
    /// 
    /// Uses extended grapheme clusters as adivised in
    /// [UAX#29](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
    /// (via <https://crates.io/crates/unicode-segmentation>)
    pub fn graphemes(&self) -> Graphemes {
        self.0.graphemes(true)
    }

    /// Returns character count for the annotation value.
    pub fn grapheme_count(&self) -> usize {
        self.0.graphemes(true).count()
    }

    /// Returns token/word count (splits annotation on whitespace).
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

    pub fn query(&self, regex: &Regex) {
        if let Some(captures) = regex.captures(self.0.as_str()) {
            for capture in captures.iter() {
    
            }
        };
    }

    /// Returns the [Blake3](https://github.com/BLAKE3-team/BLAKE3)
    /// hash for the annotation value.
    pub(crate) fn hash(&mut self) -> std::io::Result<Vec<u8>> {
        let mut hasher = blake3::Hasher::new();
        let _size = copy(self, &mut hasher)?;
        Ok(hasher.finalize().as_bytes().to_ascii_lowercase())
    }
}