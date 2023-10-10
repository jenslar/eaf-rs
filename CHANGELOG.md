# 0.5.0
- Breaking: `Eaf:de()`, `Eaf:se()` are now private, use `Eaf::read()`, `Eaf::to_string()` (and `Eaf::write()`) instead.
- Inital test of generating EAF-files from multi-tier CSV-files (one column specifies Tier ID).
- De/serialize ELAN settings-files (`.pfsx`).
- De/serialize ELAN time series configuration files (`MYELANFILE_tsconf.xml`).

# v0.4.1
- Bump Rust edition, update crates, readme.