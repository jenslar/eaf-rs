# v0.6.3
- NEW: Write controlled vocabulary ecv-files (`CvResource::write_ecv()`), that can be imported in ELAN.
- Bumped crates.

# v0.6.2
- Bumped crates.

# v0.6.1
- Bumped crates.

# v0.6.0
- NEW: Testing merging arbitrary number of EAF-files. Caveat: overlapping annotations on the same (merged) tier raises error.
- NEW: Testing tier, eaf builder patterns.
- NEW: Use extended grapheme clusters to determine word/token length via the <https://crates.io/crates/unicode-segmentation> crate.
- NOTE: Media extraction may be removed in a future update. Generating the corresponding crosscut EAF will not be removed `eaf-rs`. Any mediafile processing should be moved to whatever tool that makes use of `eaf-rs` instead.
- NOTE: This is an update to be able to publish tools depending on `eaf-rs`. There are still things that require refactoring, or removal.

# v0.5.0
- BREAKING: `Eaf:de()`, `Eaf:se()` are now private, use `Eaf::read()`, `Eaf::to_string()` (and `Eaf::write()`) instead.
- NEW: Initial test of generating EAF-files from multi-tier CSV-files (one column specifies Tier ID).
- NEW: De/serialize ELAN settings-files (`.pfsx`).
- NEW: De/serialize ELAN time series configuration files (`MYELANFILE_tsconf.xml`).

# v0.4.1
- Bump Rust edition, update crates, readme.
