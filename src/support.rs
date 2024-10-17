use std::path::{Path, PathBuf};

use url::Url;

use crate::EafError;

/// Adds suffix to existing file stem of path and returns the new path.
/// Returns path untouched if no file stem can be extracted.
pub fn affix_file_name(path: &Path, prefix: Option<&str>, suffix: Option<&str>) -> PathBuf {
    let new_path = match path.file_stem().and_then(|s| s.to_str()) {
        Some(stem) => {
            let prefix = prefix.map(|s| format!("{s}_")).unwrap_or("".to_owned());
            let suffix = suffix.map(|s| format!("_{s}")).unwrap_or("".to_owned());
            path.with_file_name(format!("{prefix}{stem}{suffix}"))
        },
        None => path.to_owned()
    };
    if let Some(ext) = path.extension() {
        return new_path.with_extension(ext)
    }
    new_path
}

/// Create URI from file path.
/// See: <https://en.wikipedia.org/wiki/File_URI_scheme>
pub(crate) fn url_from_path(path: &Path) -> Result<Url, EafError> {
    if let Ok(u) = Url::from_file_path(path) {
        Ok(u)
    } else {
        Err(EafError::UrlError(path.display().to_string()))
    }
}