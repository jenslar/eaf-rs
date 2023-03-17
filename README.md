# eaf-rs

Rust crate for reading/writing ELAN-files.

This crate was initially created for [GeoELAN](https://gitlab.com/rwaai/geoelan), a tool for annotating action camera GPS-logs using ELAN. To de/serialize XML [`quick-xml`](https://github.com/tafia/quick-xml)'s serde support is used. This means data structures (and some misguided ideas) are somewhat nested and in a bit of flux at the moment.

Parsing pfsx-file (ELAN preferences) is not yet ready, but any EAF-file v2.7+ should be ok.

Example:
```rust
use eaf_rs::Eaf;
fn main() -> std::io::Result<()> {
    let path = std::path::Path::new("MYEAF.eaf");
    let eaf = Eaf::deserialize(&path, true)?;
    println!("{:#?}", eaf);
    Ok(())
}
```