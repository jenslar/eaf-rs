use std::{io::{self, Write}, fmt::Display};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use regex::{self, Regex};

#[derive(Debug, Default)]
pub struct Token<'a> {
    value: &'a str,
    is_match: bool,
    color: Option<Color>
}

#[derive(Debug, Default)]
pub struct QueryResult<'a> {
    // annotation_index: usize,
    // tier_id: &'a str,
    // annotation_id: &'a str,
    // // annotation_value: &'a str,
    // ref_annotation_id: &'a str,
    /// Each word/token from the original annotation,
    /// separated, tagged whether it is a match or not
    tokens: Vec<Token<'a>>
}

impl <'a> Display for QueryResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl <'a> QueryResult<'a> {
    pub fn new(value: &str, rx: &Regex) {
        // regex.split(haystack)
        // regex.
        // for (i, m) in value.match_indices(rx) {

        // }
    }

    // fn set_match_color(&mut self, color: Color) -> io::Result<()> {
    fn set_match_color(&mut self, color: Color) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap(); //?;
        write!(&mut stdout, "green text!");
        self.tokens.iter_mut()
            .for_each(|t| if t.is_match {
                t.color = Some(color)
            })
    }


    pub fn is_match(&self) -> bool {
        self.tokens.iter().any(|t| t.is_match)
    }
}
