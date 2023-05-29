# eaf-rs

Rust crate for reading/writing ELAN-files.

This crate was initially created for [GeoELAN](https://gitlab.com/rwaai/geoelan), a tool for annotating action camera GPS-logs using [ELAN](https://archive.mpi.nl/tla/elan). To de/serialize XML [`quick-xml`](https://github.com/tafia/quick-xml)'s serde support is used. This means data structures (and some misguided ideas) are somewhat nested and in a bit of flux at the moment.

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
    // Get annotation in main for specified referred annotation ID
    let main_annotation = eaf.main_annotation("a42");
    println!("{main_annotation:#?}");

    Ok(())
}
```

## eaf components

- **eaf**: Module that contains the main implementation of the crate for reading and writing EAF files.
- **pfsx**: Module that provides functionality for parsing ELAN preference files.
- **ffmpeg**: Module that offers functionality related to FFmpeg, a multimedia library used in ELAN.
- **EafError**: Crate-specific error type for handling EAF-related errors.
- **Eaf**: Main structure that represents an EAF file and provides methods for deserialization and manipulation.
- **Scope**: Enumeration that describes the possible scopes of an EAF object.
- **License**: Structure that represents the license associated with an EAF file.
- **Header**: Structure that contains header information of an EAF file.
- **MediaDescriptor**: Structure that describes a media (e.g., audio or video file) used in an EAF file.
- **Property**: Structure that represents a property associated with an EAF object.
- **TimeOrder**: Structure that defines the order of time objects in an EAF file.
- **TimeSlot**: Structure that represents a timestamp in an EAF file.
- **Tier**: Structure that represents a tier (layer) in an EAF file.
- **Annotation**: Structure that represents an annotation in an EAF file.
- **LinguisticType**: Structure that describes the linguistic type of a tier in an EAF file.
- **Constraint**: Structure that defines a constraint in an EAF file.
- **StereoType**: Enumeration that describes the possible stereotypes of a tier in an EAF file.
- **Language**: Structure that represents the language used in an EAF file.
- **LexiconRef**: Structure that references a lexicon in an EAF file.
- **Index**: Structure that provides indices to access objects in an EAF file.
- **Locale**: Structure that represents the regional settings used in an EAF file.
- **ControlledVocabulary**: Structure that defines a controlled vocabulary in an EAF file.
- **JsonAnnotation**: Structure that represents an annotation in JSON format.


## License

This project is licensed under the MIT License. See the  [LICENSE](https://github.com/jenslar/eaf-rs/blob/main/LICENSE.txt) file for more details.
