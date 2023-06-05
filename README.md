# eaf-rs

Rust crate for reading/writing ELAN-files.

This crate was initially created for [GeoELAN](https://github.com/jenslar/geoelan), a tool for annotating action camera GPS-logs using [ELAN](https://archive.mpi.nl/tla/elan). To de/serialize XML [`quick-xml`](https://github.com/tafia/quick-xml)'s serde support is used. This means data structures (and some misguided ideas) are somewhat nested and in a bit of flux at the moment.

Some `Eaf`-methods are not yet ready and may be private.

Parsing pfsx-file (ELAN preferences) is not yet ready, but any EAF-file v2.7+ should be ok.

Usage (not yet on crates.io):

`Cargo.toml`:
```toml
[dependencies]
eaf-rs = {git = "https://github.com/jenslar/eaf-rs.git"}
```

`src/main.rs`:
```rust
use eaf_rs::Eaf;
fn main() -> std::io::Result<()> {
    let path = std::path::Path::new("MYEAF.eaf");

    // Read EAF, index relations between annotations, tiers and derive time slot values etc...
    let eaf = Eaf::deserialize(&path, true)?;
    println!("{eaf:#?}");

    // Get all annotations for specified tier ID
    let annotations = eaf.annotations(Some("my_tier"))?;
    println!("{annotations:#?}");

    // Get annotation in main tier for specified referred annotation ID
    let main_annotation = eaf.main_annotation("a42");
    println!("{main_annotation:#?}");

    Ok(())
}
```