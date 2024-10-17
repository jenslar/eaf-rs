# eaf-rs

Rust crate for reading/writing [ELAN](https://archive.mpi.nl/tla/elan) files. Also supports reading/writing the settings file (`.pfsx`), and the time series configuration file (if time series data is imported into ELAN for `MYFILE.eaf`, a file called `MYFILE_tsconf.xml` will be generated).

To de/serialize XML [`quick-xml`](https://github.com/tafia/quick-xml)'s serde support is used. As quick-xml is currently seeing rapid change, data structures may change slightly if necessary.

Some `Eaf`-methods are not yet ready and may be private.

Any EAF-file v2.7+ should be ok. When creating new EAF-files with `eaf-rs` EAF v3.0 is the default.

Usage (not yet on crates.io):

`Cargo.toml`:
```toml
[dependencies]
eaf-rs = {git = "https://github.com/jenslar/eaf-rs.git"}
```

`src/main.rs`:
```rust
use eaf_rs::{Eaf, Pfsx};
fn main() -> std::io::Result<()> {
    // Parse an ELAN file.
    let eaf_path = std::path::Path::new("MYEAF.eaf");
    let eaf = Eaf::read(&eaf_path)?;
    println!("{eaf:#?}");

    // Get all annotations for specified tier ID
    let annotations = eaf.annotations(Some("my_tier"))?;
    println!("{annotations:#?}");

    // Get annotation in main tier for specified referred annotation ID
    let main_annotation = eaf.main_annotation("a42");
    println!("{main_annotation:#?}");

    // Parse the corresponding settings file
    let pfsx_path = std::path::Path::new("MYEAF.pfsx");
    let pfsx = Pfsx::read(&pfsx_path)?;
    println!("{pfsx:#?}");

    Ok(())
}
```