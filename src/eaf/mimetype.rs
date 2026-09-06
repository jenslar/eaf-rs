use std::path::Path;

/// Extremly simplified mime type check from file extension
/// that may eventually include more media types.
/// However, this will always be focused on
/// media types that ELAN supports,
/// and is not intended as a general mimetype enum.
pub enum MimeType {
    Wav,
    Mp4,
    Mpeg,
    Csv,
    Xml,
    Other(String), // file extension
}

impl MimeType {
    /// Returns mime type for linked media files.
    ///
    /// This is only intended for determining mime type for
    /// ELAN-compatible multimedia files.
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .map(|o| o.to_string_lossy().to_string())
            .unwrap_or(String::from("none"))
            .to_lowercase();

        match ext.as_ref() {
            "mp4" => MimeType::Mp4,
            "wav" => MimeType::Wav,
            "mpg" | "mpeg" => MimeType::Mpeg,
            "csv" => MimeType::Csv,
            "xml" => MimeType::Xml,
            _ => MimeType::Other(ext),
        }
    }

    /// Returns a mime type string
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Wav => "audio/x-wav".to_owned(),
            MimeType::Mp4 => "video/mp4".to_owned(),
            // presumably not mpeg1...?
            MimeType::Mpeg => "video/mpeg2".to_owned(),
            MimeType::Csv => "text/csv".to_owned(),
            // application/xml is suggested as per https://datatracker.ietf.org/doc/html/rfc7303#section-4.1,
            // but ELAN uses text/xml so far. For timeseries config files ("..._tsconf.xml")
            MimeType::Xml => "text/xml".to_owned(),
            MimeType::Other(s) => format!("application/{}", s.to_owned()),
        }
    }
}
