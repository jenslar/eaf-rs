# eaf-rs

Rust crate for reading/writing [ELAN](https://archive.mpi.nl/tla/elan).

To de/serialize XML [`quick-xml`](https://github.com/tafia/quick-xml)'s serde support is used. As quick-xml is currently seeing rapid change, data structures are somewhat nested and in a bit of flux at the moment.

Some `Eaf`-methods are not yet ready and may be private.

Any EAF-file v2.7+ should be ok.

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
    let eaf = Eaf::de(&path, true)?;
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
