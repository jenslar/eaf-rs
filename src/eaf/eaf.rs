//! The core data structure for a deserialized
//! [ELAN-file](https://www.mpi.nl/tools/elan/EAF_Annotation_Format_3.0_and_ELAN.pdf).
//! 
//! Example:
//! ```
//! use eaf_rs::Eaf;
//! fn main() -> std::io::Result<()> {
//!     let path = std::path::Path::new("MYEAF.eaf");
//!     // Read ELAN-file
//!     let eaf = Eaf::read(&path)?;
//!     println!("{:#?}", eaf);
//!     Ok(())
//! }
//! ```
//!
//! Note that some methods expect `Eaf::index()` and `Eaf::derive()`
//! to be called before they are run. This is done automatically for most methods and on deserialization.
//! `Eaf::index()` indexes the EAF speeding up many "getter" methods,
//! whereas and `Eaf::derive()` derives values such as time values
//! for annotation boundaries and sets these directly at the annotation level to make them more independent.

use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator, IntoParallelIterator};
use regex::Regex;
use time::format_description;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::TimeSlot;

use super::{
    Annotation,
    Constraint,
    StereoType,
    ControlledVocabulary,
    EafError,
    Header,
    Index,
    JsonEaf,
    Language,
    LexiconRef,
    License,
    LinguisticType,
    Locale,
    Tier,
    TimeOrder,
};

/// Returns "unspecified" as `String`
/// To get around quick-xml not adding attributes with
/// empty strings ("") as value.
pub fn unspecified() -> String {
    "unspecified".to_owned()
}

/// Returns today's date and time in `xs:dateTime` form:
/// `YYYY-MM-DDTHH:mm:ss.fff+ZZ:ZZ`, see
/// <https://www.ibm.com/docs/en/i/7.5?topic=types-xsdatetime>.
/// Panics since a default must be returned.
pub fn today() -> String {
    let format = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
            sign:mandatory]:[offset_minute]")
        .expect("Failed to create date time format");
    time::OffsetDateTime::now_utc().format(&format)
        .expect("Failed to create default date time string")
}

/// Default for top-level attribute `xmlns:xsi`.
/// quick-xml expects field order as specified in struct,
/// which EAF may not follow (?)
pub fn xmlns_xsi() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".to_owned()
}

/// Default for top-level attribute `xsi:noNamespaceSchemaLocation`.
/// quick-xml expects field order as specified in struct,
/// which EAF may not follow. Note this sets version to
/// [EAFv3.0](http://www.mpi.nl/tools/elan/EAFv3.0.xsd).
pub fn xsi_no_name_space_schema_location() -> String {
    "http://www.mpi.nl/tools/elan/EAFv3.0.xsd".to_owned()
}

/// Return path as string with optional prefix, e.g. 'file://' for EAF media URLs.
///
/// Currently only handles Unicode paths. Always returns a string, but failed
/// unwraps for `Path::file_name()` return "NONE" as a dummy value.
pub fn path_to_string(path: &Path, prefix: Option<&str>, filename_only: bool) -> String {
    let path_str = match filename_only {
        true => path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or("NONE".to_owned()),
        false => path.as_os_str().to_string_lossy().to_string(),
    };
    format!("{}{}", prefix.unwrap_or(""), path_str)
}

/// Used for methods and function where
/// scope is important, e.g. token
/// or ngram stats.
pub enum Scope {
    /// Scope is a single annotation.
    /// Depending on usage,
    /// contained value can be
    /// e.g. internal annotation ID
    /// or tier ID.
    Annotation(Option<String>),
    /// Scope is a single tier.
    /// Depending on usage,
    /// contained value can be
    /// e.g. internal annotation ID
    /// or tier ID.
    Tier(Option<String>),
    /// Scope is the entire EAF-file.
    File
}

/// Core data structure for an ELAN annotation format file (`.eaf`).
/// De/Serializable. Make sure to validate output, since breaking changes
/// were introduced in EAF v2.8. E.g. valid EAF v2.7 documents with
/// controlled vocabularies do not validate against EAF v2.8+
/// schemas.
///
/// Example:
/// ```
/// use eaf_rs::Eaf;
/// fn main() -> std::io::Result<()> {
///     let path = std::path::Path::new("MYEAF.eaf");
///     let eaf = Eaf::de(&path, true)?;
///     println!("{:#?}", eaf);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "ANNOTATION_DOCUMENT")]
pub struct Eaf {
    // Attributes

    /// Default namespace.
    /// ELAN (and EAF schema) accepts this out of order,
    /// quick-xml does not, hence the default.
    #[serde(rename = "@xmlns:xsi", default="xmlns_xsi")]
    xmlns_xsi: String,

    /// Schema location.
    /// ELAN (and EAF schema) accepts this out of order,
    /// quick-xml does not, hence the default.
    #[serde(rename = "@xsi:noNamespaceSchemaLocation", default="xsi_no_name_space_schema_location")]
    xsi_nonamespaceschemalocation: String,

    /// EAF author attribute.
    /// Required even if only an empty string.
    #[serde(rename="@AUTHOR", default = "unspecified")] // TODO change to Option<String> which from 0.27.1 de (?)/serializes as Some("") if empty
    pub author: String,

    /// EAF ISO8601 date time attribute.
    /// Must be in the form: `YYYY-MM-DDTHH:mm:ss.fff+ZZ:ZZ`,
    /// where `fff` is one or more sub-second digits,
    /// and `+ZZ:ZZ` is time zone in hours and optional minutes -
    /// `Z` (the actual character) is also valid and equal to
    /// `+00:00` or `-00:00`.
    #[serde(rename="@DATE", default = "today")]
    pub date: String, // should be time::OffsetDateTime with serde support

    /// EAF format attribute, e.g. "3.0".
    #[serde(rename="@FORMAT")]
    pub format: String,

    /// EAF version attribute, e.g. "3.0".
    #[serde(rename="@VERSION")]
    pub version: String,
    
    // Child nodes
    
    /// EAF license.
    #[serde(rename="LICENSE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// EAF header. Contains media paths.
    pub header: Header,

    /// EAF time slots, used to specify annotation boundaries (defaults to milliseconds).
    /// Note that time values are optional.
    pub time_order: TimeOrder,

    /// EAF tier. Contains annotations.
    #[serde(rename = "TIER", default)]
    pub tiers: Vec<Tier>,

    /// EAF linguistic type. Referred to in tier attributes and specifies
    /// e.g. if the tier is time-alignable.
    #[serde(rename = "LINGUISTIC_TYPE", default)]
    pub linguistic_types: Vec<LinguisticType>,

    /// EAF locale.
    #[serde(rename = "LOCALE", default)]
    pub locales: Vec<Locale>,

    /// EAF languages.
    #[serde(rename = "LANGUAGE", default)]
    pub languages: Vec<Language>,

    /// EAF constraints.
    #[serde(rename = "CONSTRAINT", default)]
    pub constraints: Vec<Constraint>,

    /// EAF controlled vocabularies.
    #[serde(rename = "CONTROLLED_VOCABULARY", default)]
    pub controlled_vocabularies: Vec<ControlledVocabulary>,

    /// EAF lexicon references.
    #[serde(rename = "LEXICON_REF", default)]
    pub lexicon_refs: Vec<LexiconRef>,

    /// Not part of EAF specification. Toggle to check whether annotations have
    /// e.g. derived time slot values set.
    #[serde(skip)]
    derived: bool,

    /// Not part of EAF specification. Index with mappings for
    /// e.g. annotation ID to time slot values.
    #[serde(skip)]
    pub index: Index, // should ideally be 'pub(crate): Index'

    /// Not part of EAF specification.
    /// Toggle to check whether `Eaf` is indexed.
    #[serde(skip)]
    indexed: bool,
}

impl Default for Eaf {
    fn default() -> Self {
        Self {
            xmlns_xsi: "xmlns:xsi".to_owned(),
            xsi_nonamespaceschemalocation: "xsi:noNamespaceSchemaLocation".to_owned(),
            author: unspecified(), // required so must fill with e.g. "" as default if no value
            date: today(),
            format: "3.0".to_owned(),
            version: "3.0".to_owned(),
            license: None,
            header: Header::default(),
            time_order: TimeOrder::default(),
            tiers: vec![Tier::default()],
            linguistic_types: vec![LinguisticType::default()],
            locales: vec![Locale::default()],
            languages: Vec::new(),
            constraints: Vec::new(),
            controlled_vocabularies: Vec::new(),
            lexicon_refs: Vec::new(),
            derived: false,
            index: Index::default(),
            indexed: false,
        }
    }
}

impl AsRef<Eaf> for Eaf {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Eaf {
    /// Deserialize [ELAN-file](https://www.mpi.nl/tools/elan/EAF_Annotation_Format_3.0_and_ELAN.pdf).
    /// 
    /// If `derive` is set, all annotations will have the following derived and set:
    /// - Explicit time values.
    /// - Tier ID.
    /// - Main annotation ID for referred annotations (i.e. the ID for the alignable annotation in the main tier of the hierarchy).
    ///
    /// While `derive` is convenient if working on a single file,
    /// parsing will take \~2x the time.
    fn de(path: &Path, derive: bool) -> Result<Self, EafError> {
        // Let Quick XML use serde to deserialize
        let mut eaf: Eaf = quick_xml::de::from_str(&std::fs::read_to_string(path)?)
            .map_err(|e| EafError::QuickXMLDeError(e))?;

        // index file first...
        eaf.index();

        // ...then derive (uses Eaf::index)
        if derive {
            // Could return Eaf without deriving if it fails,
            // with the caveat that the serialized file
            // may not work in ELAN.
            // Tested pympi's Eaf.merge(), which merged tiers
            // and generated EAFs that validate against schema,
            // but do not load in ELAN, since there were
            // "ref annotations" referring to non-existing
            // "main annotations".
            eaf.derive()?;
        }

        Ok(eaf)
    }

    /// Serialize `Eaf` to indented string.
    fn se(&self, indent: Option<usize>) -> Result<String, EafError> {
        let mut eaf = self.to_owned(); // better to take &mut self as arg...?
        if eaf.author == "" {
            eaf.author = unspecified() // quick-xml ignores attr with empty string ""
        }
        for lexref in eaf.lexicon_refs.iter_mut() {
            if lexref.url == "" {
                lexref.url = unspecified()
            }
        }

        // Should already be set for deserialized EAF:s.
        eaf.set_ns();

        let mut eaf_str = String::new();
        let mut ser = Serializer::new(&mut eaf_str);
        // Optionally indent serialized XML
        if let Some(ind) = indent {
            ser.indent(' ', ind);
        }

        eaf.serialize(ser).map_err(|e| EafError::QuickXMLDeError(e))?;

        Ok([
            // Add XML declaration, since not added by quick-xml
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            eaf_str.as_str()
        ].join("\n"))
    }

    /// Returns `Eaf` that can be serialized into an ETF
    /// (ELAN Template File).
    /// 
    /// Strips annotations from tiers and linked files,
    /// but leaves attributes, controlled vocabulary etc.
    pub fn to_etf(&self) -> Self {
        Self {
            header: Header::default(),
            time_order: TimeOrder::default(),
            tiers: self.tiers.iter().map(|t| t.strip()).collect::<Vec<_>>(),
            ..self.to_owned()
        }
    }

    /// Serializes to JSON in either a simplified form (`simple` = `true`)
    /// or the full `Eaf` structure.
    pub fn to_json(&self, simple: bool) -> serde_json::Result<String> {
        match simple {
            true => serde_json::to_string(&JsonEaf::from(self)),
            false => serde_json::to_string(&self),
        }
    }

    /// Read an ELAN-file from disk.
    pub fn read(path: &Path) -> Result<Eaf, EafError> {
        Self::de(path, true)
    }

    /// Serialize to an XML-string (single line),
    /// and optionally specify indentation (multi-line).
    pub fn to_string(&self, indent: Option<usize>) -> Result<String, EafError> {
        self.se(indent)
    }

    /// Serialize and write file to disk.
    /// Returns `Ok(true)` if file was written to disk,
    /// otherwise either `Ok(false)` (user aborted write, but no errors),
    /// or `Err<EafError>` for IO/EAF errors.
    pub fn write(&self, path: &Path, indent: Option<usize>) -> Result<(), EafError> {
        let content = self.se(indent)?;

        let mut outfile = File::create(&path)?;
        outfile.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Set EAF XML namespaces.
    fn set_ns(&mut self) {
        self.xmlns_xsi = "http://www.w3.org/2001/XMLSchema-instance".to_owned();
        self.xsi_nonamespaceschemalocation =
            format!("http://www.mpi.nl/tools/elan/EAFv{}.xsd", self.version);
    }

    // /// Generate new `Eaf` from multiple `Tiers`.
    // /// These must have annotation timespans set.
    // pub fn from_tiers(tiers: &[Tier]) {
    //     let mut eaf = Eaf::default();
    // }

    /// Generate new `Eaf` with a single tier created from
    /// a list of tuples in the form `(annotation_value, start_time_ms, end_time_ms)`.
    pub fn from_values(
        values: &[(String, i64, i64)],
        tier_id: Option<&str>,
    ) -> Result<Self, EafError> {
        let mut eaf = Eaf::default();
        // let tier_id_str = tier_id.unwrap_or("default");
        // eaf.change_tier_id(tier_id_str, "default")?; // ???
        // eaf.index(); // adds changed tier_id_str to index

        // let mut count = 1;

        // !!! 230406 changed to par_iter ? collect, then setting tier, timeorder directly

        let (annotations, timeslot_pairs): (Vec<_>, Vec<_>) = values.par_iter().enumerate()
        // let (annotations, timeslot_pairs): (Vec<_>, Vec<_>) = values.iter().enumerate()
            .map(|(i, (value, ts_val1, ts_val2))| {
                // println!("{} {} {}",value, ts_val1, ts_val2);
                let ts_ref1 = &format!("ts{}", (i + 1) * 2 - 1); // 2x annotation_id - 1
                let ts_ref2 = &format!("ts{}", (i + 1) * 2);     // 2x annotation_id
                let a = Annotation::alignable(
                    value,
                    &format!("a{}", i + 1),          // i=0: 1, i=1: 2, i=2: 3, i=3: 4
                    ts_ref1,                         // i=0: 1, i=1: 3, i=2: 5, i=3: 7
                    ts_ref2,                         // i=0: 2, i=1: 4, i=2: 6, i=3: 8
                )
                .with_ts_val(*ts_val1, *ts_val2);

                let ts1 = TimeSlot::new(ts_ref1, Some(*ts_val1));
                let ts2 = TimeSlot::new(ts_ref2, Some(*ts_val2));

                (a, vec![ts1, ts2])
            })
            .unzip();

        let timeslots: Vec<TimeSlot> = timeslot_pairs.into_par_iter()
            .flatten()
            .collect();
        let timeorder = TimeOrder::new(&timeslots); // !!! should not borrow

        let tier = Tier::new(
            tier_id.unwrap_or("default"),
            Some(&annotations), // !!! should not borrow
            Some("default-lt"),
            None
        );

        eaf.time_order = timeorder;
        eaf.tiers = vec![tier];

        // !!! old, slower (?) behaviour below
        // for (i, (value, ts_val1, ts_val2)) in values.iter().enumerate() {
        //     let mut a = Annotation::alignable(
        //         value,
        //         &format!("a{}", i + 1),
        //         &format!("ts{}", count + i),
        //         &format!("ts{}", count + i + 1),
        //     );

        //     count += 1;

        //     // Set AlignableAnnotation.
        //     a.set_ts_val(Some(*ts_val1), Some(*ts_val2));

        //     eaf.add_annotation(&a, tier_id_str, false)?;
        // }

        eaf.index();
        eaf.derive()?;

        Ok(eaf)
    }

    // pub fn from_csv(csv: &Path) {
    //     // let mut eaf = Eaf::default();
    //     // let mut rdr = csv::Reader::from_path(csv).unwrap();
    //     // for result in rdr.records() {
    //     //     let record = result.unwrap();
    //     //     println!("{:?}", record);
    //     // }
    // }

    /// Generate new `Eaf` with one or more tiers created from
    /// a list of tuples in the form `(annotation_value, start_time_ms, end_time_ms, tier_id)`.
    pub fn from_values_multi(
        values: &[(String, i64, i64, String)]
    ) -> Result<Eaf, EafError> {
        // Generate tiers, values only (annotation value, timestamps in milliseconds)
        let mut groups: HashMap<String, Vec<(String, i64, i64)>> = HashMap::new();
        // println!("RAW: {values:?}");
        values.iter()
            .for_each(|(a, t1, t2, t_id)| groups
                .entry(t_id.to_string()).or_insert(Vec::new()).push((a.to_owned(), *t1, *t2)));
                // .and_modify(|v| v.push((a.to_owned(), *t1, *t2)) ));
                // .or_insert(Vec::new()));
        // for (i, (k, g)) in groups.iter().enumerate() {
        //     println!("{}. {k} {g:?}", i+1, );
        // }
        // timeslot and annotation references need re-indexing
        let mut start_index: usize = 1;
        let tiers: Vec<Tier> = groups.iter()
            .map(|(tier_id, values)| {
                let t = Tier::main_from_values(values, tier_id, Some(start_index));
                start_index += t.len(); // update start index
                t
            })
            .collect();
            
        // just checking... remove this before publishing
        for t in tiers.iter() {
            println!("{}", t.tier_id);
            for a in t.annotations.iter() {
                println!("  {}\n    ID: {}, TS: {:?} | {:?}", a.value(), a.id(), a.ts_ref(), a.ts_val())
            }
        }
        let mut time_order = Vec::new();
        for t in tiers.iter() {
            let mut ts = t.derive_timeslots()
                .ok_or_else(|| EafError::MissingTimeslotRefs)?;
            time_order.append(&mut ts)
        }

        let mut eaf = Self::default();
        eaf.tiers = tiers;
        eaf.time_order = TimeOrder::new(&time_order);

        eaf.index();
        eaf.derive()?; // probably not needed?

        Ok(eaf)
    }

    /// Derives and sets the following in all `Annotation` structs:
    /// - Time values in milliseconds.
    /// - Annotation ID for referred main annotation (may or may not be the same as annotation_ref)
    /// - Tier ID
    ///
    /// Mostly for internal use. Also makes annotations less dependent,
    /// since they now contain explicit time slot values etc.
    pub fn derive(&mut self) -> Result<(), EafError> {
        // copy since otherwise error is raised
        // better solution would be nice
        let eaf_copy = self.to_owned();

        // Iter mut over self...
        for tier in self.tiers.iter_mut() {
            for annotation in tier.annotations.iter_mut() {
                // ...check if annotion is ref annotation...
                let (ref1, ref2) = match annotation.is_ref() {
                    true => {
                        // ...then use the copy for deriving main annotation for ref annotations.
                        let ma = eaf_copy.main_annotation(&annotation.id()).ok_or(
                            EafError::MissingMainAnnotation((annotation.id(), annotation.ref_id())),
                        )?;

                        // Set main annotion ID for ref annotation...
                        annotation.set_main(&ma.id());

                        // ...then get annotation ID for main annotation.
                        ma.ts_ref()
                            .ok_or(EafError::MissingTimeslotRef(annotation.id()))?
                    }

                    // Raise error if annotation in main tier returns no time slot references.
                    false => annotation
                        .ts_ref()
                        .ok_or(EafError::MissingTimeslotRef(annotation.id()))?,
                };

                let val1 = eaf_copy.ts_val(&ref1);
                let val2 = eaf_copy.ts_val(&ref2);
                annotation.set_ts_val(val1, val2);
                annotation.set_tier_id(&tier.tier_id);
            }
        }

        self.derived = true; // set check for .derive()

        Ok(())
    }

    /// Indexes the ELAN-file with the following mappings:
    /// - `a2t`: Annotation ID to tier ID
    /// - `a2ref`: Annotation ID to ref annotation ID
    /// - `t2a`: Tier ID to list of annotation ID:s
    /// - `t2ref`: Tier ID to ref tier ID
    /// - `id2ts`: Time slot ID to time slot value
    /// - `ts2id`: Time slot value to Time slot ID
    /// - `a2ts`: Annotation ID to time slot id/ref tuple, `(time_slot_ref1, time_slot_ref2)`.
    /// - `a2idx`: Annotation ID to `(idx1, idx2)` in `Eaf.tiers[idx1].annotations[idx2]`
    /// - `t2idx`: Tier ID to `idx` in `Eaf.tiers[idx]`
    ///
    /// Speeds up many "getter" methods, such as finding cross referenced annotations,
    /// time values for referred annotations etc. Done automatically on deserialization.
    /// Re-run as necessary, after external edit etc. Automatic for internal methods,
    /// such as adding an annotation or a tier.
    pub fn index(&mut self) {
        let mut a2t: HashMap<String, String> = HashMap::new();
        let mut a2ref: HashMap<String, String> = HashMap::new();
        let mut t2a: HashMap<String, Vec<String>> = HashMap::new();
        let mut t2ref: HashMap<String, String> = HashMap::new();
        let mut a2ts: HashMap<String, (String, String)> = HashMap::new(); // Annotation ID -> time slot ref1, ref2
        let mut a2idx: HashMap<String, (usize, usize)> = HashMap::new(); // Annotation ID -> tier idx, annot idx
        let mut t2idx: HashMap<String, usize> = HashMap::new(); // Tier ID -> tier idx

        self.tiers.iter().enumerate().for_each(|(idx_t, t)| {
            // Tier ID -> tier idx in self.tiers
            t2idx.insert(t.tier_id.to_owned(), idx_t);

            // Tier ID -> Ref tier ID
            if let Some(t_id) = t.parent_ref.to_owned() {
                t2ref.insert(t.tier_id.to_owned(), t_id);
            }

            // Used for Tier ID -> [Annotation ID, ...]
            let mut a_id: Vec<String> = Vec::new();

            t.annotations.iter().enumerate().for_each(|(idx_a, a)| {
                let id = a.id();

                // Annotation ID -> (Tier index, Annotation index)
                a2idx.insert(id.to_owned(), (idx_t, idx_a));

                // Annotation ID -> Annotation ref ID
                if let Some(ref_id) = a.ref_id() {
                    a2ref.insert(id.to_owned(), ref_id);
                };

                // Annotation ID -> Tier ID
                a2t.insert(id.to_owned(), t.tier_id.to_owned());

                // Annotation ID -> (time slot ref 1, time slot ref2)
                if let Some((ref1, ref2)) = a.ts_ref() {
                    a2ts.insert(id.to_owned(), (ref1, ref2));
                }

                a_id.push(id);
            });

            // Tier ID -> [Annotation ID, ...]
            t2a.insert(t.tier_id.to_owned(), a_id);
        });

        self.index = Index {
            a2t,
            a2ref,
            t2a,
            t2ref,
            ts2tv: self.time_order.index(),
            tv2ts: self.time_order.index_rev(),
            a2ts,
            a2idx,
            t2idx,
        };

        self.indexed = true;
    }

    /// Generates empty ELAN-file with specified media files linked.
    pub fn with_media(media_paths: &[PathBuf]) -> Self {
        let mut eaf = Self::default();
        for path in media_paths.iter() {
            eaf.add_media(path, None);
        }
        eaf
    }

    /// Links specified media files.
    pub fn with_media_mut(&mut self, media_paths: &[PathBuf]) {
        for path in media_paths.iter() {
            self.add_media(path, None);
        }
    }

    /// Adds new media path to header as a new media descriptor.
    pub fn add_media(&mut self, path: &Path, extracted_from: Option<&str>) {
        self.header.add_media(path, extracted_from)
    }

    /// Removes specific media file from header if it is set.
    /// Matches on file name, not the entire path.
    pub fn remove_media(&mut self, path: &Path) {
        self.header.remove_media(path)
    }

    /// Scrubs absolute media paths in header, and optionally relative ones.
    /// Absolute paths sometimes contain personal information, such as user name.
    /// If both paths are scrubbed media files have to be completely re-linked in ELAN.
    pub fn scrub_media(&mut self, keep_filename: bool) {
        self.header.scrub_media(keep_filename)
    }

    /// Returns all media paths as string tuples,
    /// `(media_url, relative_media_url)`.
    /// `media_url` is optional.
    pub fn media_paths(&self) -> Vec<(String, Option<String>)> {
        self.header.media_paths()
    }

    /// Returns all linked absolute media paths as strings.
    pub fn media_abs_paths(&self) -> Vec<String> {
        self.header.media_abs_paths()
    }

    /// Returns all linked relative media paths (optional value) as strings.
    pub fn media_rel_paths(&self) -> Vec<String> {
        self.header.media_rel_paths()
    }

    /// Returns a hashmap (name: value) of all properties in header.
    /// Key: name (`NAME` attribute)
    /// Value: value (element text value)
    pub fn properties(&self) -> HashMap<String, String> {
        self.header
            .properties
            .iter()
            .map(|p| (
                p.name.as_deref().unwrap_or("").to_owned(),
                p.value.to_owned()
            ))
            .collect()
    }

    /// Retrurns hashmap of all time slots.
    /// - Key: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    /// - Value: timeslot value in milliseconds (may be `None`).
    pub fn timeslots(&self) -> HashMap<String, Option<i64>> {
        if self.indexed {
            self.index.ts2tv.to_owned()
        } else {
            self.time_order.index()
        }
    }

    /// Reverse lookup table for time slot values.
    /// - Key: timeslot value in milliseconds.
    /// - Value: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    ///
    /// Only includes time slots with a time value set.
    pub fn timeslots_rev(&self) -> HashMap<i64, String> {
        if self.indexed {
            self.index.tv2ts.to_owned()
        } else {
            self.time_order.index_rev()
        }
    }

    /// Returns the time slot ID for specified time slot value.
    /// Note that a time slot value is not required according to the EAF specification.
    ///
    /// Requires that `Eaf.index()` has been run.
    pub fn ts_id(&self, ts_val: i64) -> Option<String> {
        self.index.tv2ts.get(&ts_val).cloned()
    }

    /// Returns the time value if one is specified for the time slot id,
    /// `None` otherwise, or if there are no time slots.
    /// Note that a time slot value is not required according to the EAF specification.
    pub fn ts_val(&self, ts_id: &str) -> Option<i64> {
        if self.indexed {
            *self.index.ts2tv.get(ts_id)?
        } else {
            self.time_order.find(ts_id)?.time_value
        }
    }

    /// Returns the smallest time slot value.
    /// Does not provide media boundaries,
    /// only the first time slot with a time value.
    pub fn ts_min_val(&self) -> Option<i64> {
        if self.indexed {
            self.index.tv2ts.keys().min().cloned() // use ts2id.keys() to ensure value
        } else {
            self.time_order.min_val()
        }
    }

    /// Returns the largest time slot value.
    /// Does not provide media boundaries,
    /// only the last time slot with a time value.
    pub fn ts_max_val(&self) -> Option<i64> {
        // pub fn ts_max(&self) -> Option<u64> {
        if self.indexed {
            self.index.tv2ts.keys().max().cloned() // use ts2id.keys() to ensure value
        } else {
            self.time_order.max_val()
        }
    }

    /// Shift all time values with the specified value in milliseconds.
    /// `allow_negative` ignores if the resulting time values are negative,
    /// otherwise `EafError::ValueTooSmall(time_value)` is raised.
    pub fn shift(&mut self, shift_ms: i64, allow_negative: bool) -> Result<(), EafError> {
        self.time_order.shift(shift_ms, allow_negative)
    }

    /// NOT YET READY FOR MERGING TWO ARBITRARY EAF-FILES.
    /// ASSUME MERGE CURRENTLY DOES NOT WORK FOR YOUR PURPOSES.
    /// 
    /// Merges two ELAN-files if possible. This is done with the assumption that both
    /// files link the same media files and that tiers with identical tier ID:s have the same attributes,
    /// which is important for tier hierarchy, linguistic types etc. This is up to the user
    /// to ensure.
    ///
    /// Caveats:
    /// - Linked files and any tier attributes will be inherited from the first file only.
    /// - Time slots without a time value will be discarded.
    fn _merge(paths: &[PathBuf; 2]) -> Result<Self, EafError> {
        // let mut eaf1 = Eaf::de(&paths[0], true)?;
        // let eaf1_a_len = eaf1.a_len();
        // let eaf1_ts_len = eaf1.time_order.len();

        // // remap eaf2 annotation id:s + time slot id:s to start after eaf1
        // // at this point there may be duplicate annotations and time slots
        // let mut eaf2 = Eaf::de(&paths[1], true)?;
        // eaf2.remap(Some(eaf1_a_len+1), Some(eaf1_ts_len+1))?;

        // eaf1.time_order.join(&eaf2.time_order);

        // for tier2 in eaf2.tiers.iter() {
        //     if let Some(tier1) = eaf1.get_tier_mut(&tier2.tier_id) {
        //         // Either join tiers if both have same tier id or...
        //         tier1.join(tier2, true);
        //     } else {
        //         // ... add as new tier (adds timeslots as well)
        //         eaf1.add_tier(Some(tier2.to_owned()))?
        //     }
        // }

        let mut eaf1 = Eaf::de(&paths[0], true)?;
        let eaf1_a_len = eaf1.a_len();
        let eaf1_ts_len = eaf1.time_order.len();
        let mut eaf2 = Eaf::de(&paths[1], true)?;
        eaf2.remap(Some(eaf1_a_len + 1), Some(eaf1_ts_len + 1))?;

        // Check for matching tier ID:s: merge if match, add as new tier otherwise
        for tier2 in eaf2.tiers.iter() {
            if let Some(tier1) = eaf1.get_tier_mut(&tier2.tier_id) {
                println!("Merging {} & {}", tier1.tier_id, tier2.tier_id);
                tier1._merge(tier2, false)?;
            } else {
                println!("Adding {}", tier2.tier_id);
                eaf1.add_tier(Some(tier2.to_owned()), None)?;
            }
        }

        // let timeslots = TimeOrder::

        eaf1.remap(None, None)?;

        // TODO comparing annotation overlaps, duplication: dedup, by somehow comparing annotation + time values
        // TODO comparing timeslots: eaf1 + eaf2 *must* be derived, then use annotations' derived ts values as basis for what to keep
        // TODO fix overlapping annotation boundaries, option to join these

        Ok(eaf1)
    }

    // fn sort_annotations(&mut self, join: bool) -> Result<(), EafError> {
    //     if !self.indexed {self.index()}
    //     if !self.derived {self.derive()?} // adds time values to annotations for sort

    //     Ok(())
    // }

    /// Match annotation values against a string.
    /// Returns a vec with tuples: `(Annotation Index, Tier ID, Annotation ID, Annotation value)`.
    pub fn query(&self, pattern: &str, ignore_case: bool) -> Vec<(usize, String, String, String, Option<String>)> {
        // (Annotation Index, Tier ID, Annotation ID, Annotation value)
        self.tiers
            // .iter()
            .par_iter()
            .filter_map(|t| {
                let results = t.query(pattern, ignore_case);
                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            })
            .flatten()
            .collect()
    }

    /// Match annotation values against a regular expression.
    /// Returns a vec with tuples: `(Annotation Index, Tier ID, Annotation ID, Annotation value)`.
    pub fn query_rx(&self, regex: &Regex) -> Vec<(usize, String, String, String, Option<String>)> {
        // (Annotation Index, Tier ID, Annotation ID, Annotation value)
        self.tiers
            // .iter()
            .par_iter()
            .filter_map(|t| {
                let results = t.query_rx(regex);
                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            })
            .flatten()
            .collect()
    }

    /// Returns all words/tokens in ELAN-file. Does not work with languages
    /// that do not use white space to delimit words/tokens.
    /// Optionally, `strip_prefix` and `strip_suffix` are strings containing characters
    /// that will be stripped, so that for `strip_prefix = Some("<*")`: "<hi", "*hi", "hi"
    /// all become "hi" in the output.
    pub fn tokens(
        &self,
        strip_prefix: Option<&str>,
        strip_suffix: Option<&str>,
        unique: bool,
        ignore_case: bool,
    ) -> Vec<String> {
        let mut tokens: Vec<String> = self.tiers
            // .iter()
            .par_iter()
            .map(|t| t.tokens(strip_prefix, strip_suffix, unique, ignore_case))
            .flatten()
            .collect();

        tokens.sort();

        if unique {
            tokens.dedup();
        }

        tokens
    }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts which uses whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// 
    /// Scope:
    /// - `Scope::Tier(Some(TIER_ID))` compiles ngrams across annotation boundaries
    /// - `Scope::Annotation(Some(TIER_ID))` compiles ngrams across annotation boundaries
    /// - `Scope::File` compiles ngrams across annotation and tier boundaries and combines the result
    /// 
    /// Returns `HashMap<ngram, count>`.
    pub fn ngram(&self, size: usize, regex_remove: Option<&Regex>, scope: Scope) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();
        match scope {
            Scope::Annotation(tier_id) => {
                if let Some(t_id) = tier_id {
                    match self.get_tier(&t_id) {
                        Some(t) => return t.ngram(size, regex_remove, false),
                        None => return HashMap::new()
                    };
                } else {
                    return HashMap::new()
                }
            },
            Scope::Tier(tier_id) => {
                if let Some(t_id) = tier_id {
                    match self.get_tier(&t_id) {
                        Some(t) => return t.ngram(size, regex_remove, true),
                        None => return HashMap::new()
                    };
                } else {
                    return HashMap::new()
                }
            },
            Scope::File => {
                self.tiers.iter()
                    .for_each(|t| ngrams.extend(t.ngram(size, regex_remove, true)))
            },
        }

        ngrams
    }

    /// Returns total number of annotations in EAF.
    pub fn a_len(&self) -> usize {
        self.tiers.iter()
            .map(|t| t.len())
            .sum()
    }

    /// Average annotation length,
    /// i.e. average number of tokens (`char`s)
    /// in each annotation.
    pub fn a_avr_len(&self) -> f64 {
        let avr: Vec<f64> = self.tiers.iter()
            .map(|t| t.avr_annot_len())
            .collect();
        match avr.len() {
            0 => 0.,
            _ => avr.iter().sum::<f64>() / avr.len() as f64
        }
    }
    
    /// Returns number of tiers in EAF.
    pub fn t_len(&self) -> usize {
        self.tiers.len()
    }

    /// Average tier length,
    /// i.e. average number of annotations
    /// in each tier.
    pub fn t_avr_len(&self) -> f64 {
        let t_len = self.t_len();
        match t_len {
            0 => 0.,
            _ => self.tiers.iter().map(|t| t.len()).sum::<usize>() as f64 / t_len as f64
        }
    }

    /// Total number of words/tokens.
    pub fn tkn_len(&self) -> usize {
        self.tiers.par_iter()
            .map(|t| t.annotations.iter().map(|a| a.len()).sum::<usize>())
            .sum::<usize>()
    }

    /// Average word/token length.
    pub fn tkn_avr_len(&self) -> f64 {
        let avr: Vec<f64> = self.tiers.par_iter()
            .map(|t| t.avr_token_len())
            .collect();
        match avr.len() {
            0 => 0.,
            _ => avr.iter().sum::<f64>() / avr.len() as f64
        }
    }

    /// Pushes a time slot to time order as last item.
    /// Ensures the time slots does not exist, but does not verify
    /// that specified time slot ID/value is greater than exisiting ones.
    pub fn add_timeslot(&mut self, id: &str, val: Option<i64>, index: bool) {
        // pub fn push_timeslot(&mut self, id: &str, val: Option<u64>, index: bool) {
        self.time_order.add(id, val);
        if index {
            self.index()
        } else {
            self.indexed = false
        }
    }

    /// Returns a copy of all annotations in ELAN-file or for specified tier.
    pub fn annotations(&self, tier_id: Option<&str>) -> Result<Vec<Annotation>, EafError> {
        // clone to avoid having to pass &mut self for index+derive...
        let mut eaf = self.to_owned();

        if !eaf.indexed {
            eaf.index()
        }; // needed for derive()
        if !eaf.derived {
            eaf.derive()?
        };

        if let Some(id) = tier_id {
            eaf.tiers.into_iter()
                .find(|t| t.tier_id == id)
                .map(|t| t.annotations)
                .ok_or(EafError::InvalidTierId(id.to_owned()))
            // or just Option -> None?
        } else {
            Ok(eaf.tiers.into_iter()
                .flat_map(|t| t.annotations)
                .collect())
        }
    }

    /// Verifies that:
    /// - time slot reference ID:s are valid for alignable annotations.
    /// - reference annotation ID:s are valid for referred annotations.
    /// - reference tier ID:s are valid for referred tiers.
    /// Does not raise errors, only print stats.
    fn _validate(&mut self, _verbose: bool) {
        // if !self.indexed {self.index()}

        // let mut t_orphans: Vec<(String, String)> = Vec::new(); // (tier ID, ref tier ID)
        // let mut a_orphans: Vec<String> = Vec::new();
        // let mut ts_orphans: Vec<String> = Vec::new();

        // self.tiers.iter()
        //     .for_each(|t| {
        //         if let Some(t_id) = &t.parent_ref {
        //             if !self.exists(t_id).0 {
        //                 t_orphans.push((t.tier_id.to_owned(), t_id.to_owned()));
        //             }
        //         }
        //     })
        unimplemented!()
    }

    /// Remaps time slots and annotation ID:s so that they start on 1 or, optionally,
    /// specified annotation ID and/or time slot ID.
    /// For use with e.g. `filter()`, where parts of the EAF have been filtered out.
    /// Resets ID counters for timeslots and annotations to start on 1.
    /// Relabels and remaps the following numerical identifiers:
    /// - annotation ID:s for all annotations.
    /// - references to annotation ID:s for referred annotations.
    /// - time slot ID:s.
    /// - references to time slot ID:s for aligned annotations.
    pub fn remap(&mut self, a_idx: Option<usize>, ts_idx: Option<usize>) -> Result<(), EafError> {
        if !self.indexed {
            self.index()
        } // does not work for merged tiers if contains duped annot id:s.

        // 1. Remap time slots and create lookup table for current time slot ID -> new time slot ID
        let ts_map = self.time_order.remap(ts_idx);

        // 2. Create lookup table for current annotation ID -> new annotation ID
        let start_a_id = a_idx.unwrap_or(0);
        let a_map: HashMap<String, String> = self.annotations(None)?
            .iter()
            .enumerate()
            .map(|(i, a)| (a.id(), format!("a{}", start_a_id + i + 1)))
            .collect();

        // 3. Remap annotation ID and reference annotation ID.
        for tier in self.tiers.iter_mut() {
            for annotation in tier.iter_mut() {
                let annotation_id = annotation.id();

                // Look up and set new annotation ID. Required for all annotations.
                let new_a_id = a_map.get(&annotation_id)
                    .ok_or(EafError::InvalidAnnotationId(annotation_id.to_owned()))?;
                annotation.set_id(&new_a_id);
                // println!("A ID  {} -> {}", annotation_id, new_a_id);

                // Look up and set new time slot references. Required for alignable annotations.
                if let Some((ts1, ts2)) = self.index.a2ts.get(&annotation_id) {
                    let new_ts1 = ts_map.get(ts1)
                        .ok_or(EafError::InvalidTimeSlotId(ts1.to_owned()))?;
                    let new_ts2 = ts_map.get(ts2)
                        .ok_or(EafError::InvalidTimeSlotId(ts2.to_owned()))?;
                    annotation.set_ts_ref(new_ts1, new_ts2);

                    // println!("A TS1 {} -> {}", ts1, new_ts1);
                    // println!("A TS2 {} -> {}", ts2, new_ts2);
                }

                // If it exists, look up and set reference annotation ID. Required for referred annotations.
                if let Some(new_a_ref) = annotation.ref_id().and_then(|r| a_map.get(&r)) {
                    // println!("A REF {:?} -> {}", annotation.ref_id(), new_ref_id);
                    annotation.set_ref_id(new_a_ref);
                }

                // If it exists, look up and set previous annotation ID.
                if let Some(new_a_prev) = annotation.previous().and_then(|r| a_map.get(&r)) {
                    // println!("A REF {:?} -> {}", annotation.previous(), new_ref_id);
                    annotation.set_previous(new_a_prev);
                }
            }
        }

        // Re-index + derive EAF with updated values.
        self.index();
        self.derive()?;

        Ok(())
    }

    // /// Test for generating timeslots + references
    // /// Annotation or timeslot references are assumed not to exist,
    // /// and will ignored if they do exist,
    // /// but annotations all must have timeslot values set
    // pub fn generate_references(&mut self) -> Result<(), EafError> {
    //     // 1. generate temporary references
    //     let mut a_index = 1_usize;
    //     let mut ts_index = 1_usize;
    //     let mut timeslots: Vec<TimeSlot> = Vec::new();
    //     for tier in self.tiers.iter_mut() {
    //         for annot in tier.annotations.iter_mut() {
    //             annot.set_id(&format!("a{a_index}"));
    //             a_index += 1;
    //             let (t1, t2) = annot.ts_val();
    //             let ts1 = TimeSlot::new(&format!("ts{ts_index}"), t1);
    //             let ts2 = TimeSlot::new(&format!("ts{ts_index}"), t2);
    //             annot.set_ts_ref(&ts1.time_slot_id, &ts2.time_slot_id);
    //             timeslots.push(ts1);
    //             timeslots.push(ts2);
    //             ts_index += 2;
    //         }
    //     }

    //     self.time_order = TimeOrder::new(&timeslots);

    //     // 2. Index file with new values (derive not necessary?)
    //     self.index();
    //     // self.derive()?;

    //     // 3. since we now have a valid index, it's possible to use remap
    //     self.remap(None, None)?;

    //     Ok(())
    // }

    /// Extracts a section, that retains content
    /// within `start`, `end` boundaries in milliseconds.
    /// All annotations within that time span will be intact.
    ///
    /// - `media_dir`: Path to the directory containing linked media files.
    /// - `ffmpeg_path`: Custom path to ffmpeg. If ffmpeg is already in `PATH`,
    /// set this to `None`.
    /// - `process_media`: If set to `true`, the original media files will be
    /// processed according to the specified time span and linked. Requires ffmpeg.
    /// The original media files will remain untouched on disk.
    pub fn extract(
        &self,
        start: i64,
        end: i64,
        media_dir: Option<&Path>, // not needed...? just use linked media paths?
        ffmpeg_path: Option<&Path>,
        process_media: bool,
    ) -> Result<Self, EafError> {
        let mut eaf = self.to_owned();

        // 1. Filter time order.
        //    Time slots without time values between min/max time slots
        //    with a time value will be preserved. However, the range
        //    can only start and end with a time slot with a time value set.
        let time_order = eaf.time_order.filter(start, end)
            .ok_or(EafError::InvalidTimeSpan((start, end)))?;
        eaf.time_order = time_order.to_owned();

        // 2. Make sure annotations have derived timestamps etc.
        if !eaf.derived {
            eaf.derive()?
        }

        // Owned `Index` to avoid borrow errors...
        let index = eaf.index.to_owned();

        // 3. Iterate over tiers and annotations...
        for tier in eaf.tiers.iter_mut() {
            let annots: Vec<Annotation> = tier
                .iter()
                .filter(|a| {
                    // ...then retrieve time slot ID. Need to check if each annotation
                    // is a ref annotation by trying to retrieve `main_annotation` reference...
                    let ts_ids = match &a.main() {
                        // Ref annotation
                        Some(id) => index.a2ts.get(id).to_owned(),
                        // Alignable annotation
                        None => index.a2ts.get(&a.id()).to_owned(),
                    };

                    // Do the actual filtering based on whether filtered `ts_id` Vec
                    // contains the time slot references/ID:s in question.
                    // This means that only annotations fully contained within
                    // the time span will be preserved.
                    if let Some((ts_id1, ts_id2)) = ts_ids {
                        time_order.contains_id(&ts_id1) && time_order.contains_id(&ts_id2)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            tier.annotations = annots;
        }

        // Generates new media files from time span and sets these as new media url:s.
        if process_media {
            for mdsc in eaf.header.media_descriptor.iter_mut() {
                mdsc.timespan(start, end, media_dir, ffmpeg_path)?;
            }
        }

        eaf.indexed = false;

        // 4. remap/update identifiers so that:
        //    - annotation ID:s start on "a1",
        //      including remapping ref annotation ID:s.
        //    - time slot ID:s start on "ts1",
        //      including remapping time slot references.
        //    - re-indexes eaf
        eaf.remap(None, None)?;

        Ok(eaf)
    }

    /// Attempts to add an annotation as last item in tier with specified tier ID,
    /// together with corresponding time slot in time order.
    /// If time values are not set (or are intentionally `None`) in the annotation,
    /// time slots with empty time slot values will be created, but note that
    /// time slots with no time value can never be the the final time slot.
    /// (Re-)index is optional. This is for cases where annotations are added in batch,
    /// in which case it may be better to index only once when done.
    pub fn add_annotation(
        &mut self,
        annotation: &Annotation,
        tier_id: &str,
        index: bool,
    ) -> Result<(), EafError> {
        // Derive if not done.
        if !self.derived {
            self.derive()?
        }

        // Check if annotation with same ID already exists.
        if matches!(&self.exists(&annotation.id()), (_, true)) {
            return Err(EafError::AnnotationIDExists(annotation.id()));
        }

        // Ensure referred annotation ID exists if ref annotation
        if let Some(ref_id) = annotation.ref_id() {
            if matches!(&self.exists(&ref_id), (_, false)) {
                return Err(EafError::InvalidAnnotationId(annotation.id()));
            }
        } else {
            // Add time slots if alignable annotation.
            let (ts_id1, ts_id2) = annotation.ts_ref()
                .ok_or(EafError::MissingTimeslotRef(annotation.id()))?;
            let (ts_val1, ts_val2) = annotation.ts_val();

            // Add time slots to time order. Only adds if it does not exist.
            self.add_timeslot(&ts_id1, ts_val1, false);
            self.add_timeslot(&ts_id2, ts_val2, false);
        }

        self.get_tier_mut(tier_id)
            .ok_or(EafError::InvalidTierId(tier_id.to_owned()))?
            .add(annotation);

        // Index or set `indexed` to false if not.
        if index {
            self.index()
        } else {
            self.indexed = false
        }

        Ok(())
    }

    // /// Add an annotation along the time line to the specified tier,
    // /// including between, before, or after existing annotations.
    // pub fn add_annotation2(
    //     &mut self,
    //     annotation: &Annotation,
    //     tier_id: &str,
    //     index: bool,
    // ) -> Result<(), EafError> {
    //     Ok(())
    // }

    /// Returns reference to annotion with specified annotation ID if it exits.
    pub fn get_annotation(&self, id: &str) -> Option<&Annotation> {
        let (t_idx, a_idx) = self.index.a2idx.get(id)?;
        self.tiers.get(*t_idx)?.annotations.get(*a_idx)
    }

    /// Returns a mutable reference to annotion with specified annotation ID if it exits.
    pub fn get_annotation_mut(&mut self, id: &str) -> Option<&mut Annotation> {
        let (t_idx, a_idx) = self.index.a2idx.get(id)?;
        self.tiers.get_mut(*t_idx)?.annotations.get_mut(*a_idx)
    }

    /// Returns a reference to main annotation ID for specified ref annotation ID.
    pub fn main_annotation(&self, id: &str) -> Option<&Annotation> {
        match &self.index.a2ref.get(id) {
            Some(i) => self.main_annotation(i), // no mut version due to borrow issue here...
            None => self.get_annotation(id),
        }
    }

    /// Returns a mutable reference to main annotation ID for specified ref annotation ID.
    pub fn main_annotation_mut(&mut self, id: &str) -> Option<&mut Annotation> {
        let main_id = self.main_annotation(id)?.id(); // not mutable...
        self.get_annotation_mut(&main_id)
    }

    /// Returns first annotation if time slot values are set.
    pub fn first_annotation(&self) -> Option<&Annotation> {
        // TODO could alternatively use smallest time slot value in time order and find that
        // TODO  reference in annotation ts_ref?
        
        let mut first_annots = self.tiers.iter()
            .filter_map(|t| t.first())
            .collect::<Vec<_>>();
        // If time slot values are not set, use max int value
        // to ensure it's not first. (creates issues if none are set...)
        first_annots.sort_by_key(|a| a.ts_val().0.unwrap_or(i64::MAX));

        first_annots.first().cloned()
    }

    /// Returns last annotation if time slot values are set.
    pub fn last_annotation(&self) -> Option<&Annotation> {
        // TODO could alternatively use largest time slot value in time order and find that
        // TODO  reference in annotation ts_ref?
        
        let mut last_annots = self.tiers.iter()
            .filter_map(|t| t.last())
            .collect::<Vec<_>>();
        // If time slot values are not set, use max int value
        // to ensure it's not first. (creates issues if none are set...)
        last_annots.sort_by_key(|a| a.ts_val().0.unwrap_or(i64::MAX));

        last_annots.last().cloned()
    }

    /// Returns a reference to main tier for specified ref tier ID.
    pub fn main_tier(&self, id: &str) -> Option<&Tier> {
        match &self.index.t2ref.get(id) {
            Some(i) => self.main_tier(i),
            None => self.get_tier(id),
        }
    }

    /// Returns mutable reference to main tier for specified ref tier ID.
    pub fn main_tier_mut(&mut self, id: &str) -> Option<&mut Tier> {
        let main_id = self.main_tier(id)?.tier_id.to_owned(); // not mutable...
        self.get_tier_mut(&main_id)
    }

    /// Returns a reference to the parent tier if
    /// specified tier ID is a referred tier.
    /// Returns `None` if tier ID is a main tier,
    /// or if either tier ID or the parent tier ID
    /// does not exist.
    pub fn parent_tier(&self, id: &str) -> Option<&Tier> {
        match &self.get_tier(id)?.parent_ref {
            Some(ref_id) => self.get_tier(ref_id),
            None => None,
        }
    }

    /// Returns references to all child tiers if present.
    pub fn main_tiers(&self) -> Vec<&Tier> {
        self.tiers.iter()
            .filter(|t| t.parent_ref == None)
            .collect()
    }

    /// Returns references to all child tiers if present.
    pub fn child_tiers(&self, id: &str) -> Vec<&Tier> {
        self.tiers.iter()
            .filter(|t| t.parent_ref.as_deref() == Some(id))
            .collect()
    }

    /// Checks if tier with specified tier ID is tokenized.
    /// `recursive` checks if any parent is tokenized and returns `true`
    /// for the first tokenized parent found.
    /// Returns false if not tokenized or if tier ID does not exist.
    pub fn is_tokenized(&self, tier_id: &str, recursive: bool) -> bool {
        if recursive {
            let tier = match self.get_tier(tier_id) {
                Some(t) => t,
                None => return false,
            };

            // can only return true immediately if
            // tier with ID `tier_id` is tokenized
            let is_tkn = tier.is_tokenized();
            if is_tkn {
                true
            } else {
                // false, so need to check if parents are tokenized
                match &tier.parent_ref {
                    Some(id) => self.is_tokenized(id, recursive),
                    None => is_tkn,
                }
            }
        } else {
            self.get_tier(tier_id)
                .map(|t| t.is_tokenized())
                .unwrap_or(false)
        }
    }

    /// Adds a tier as the final item.
    /// If no tier is specified, an empty, default tier is appended - `stereotype`
    /// will be ignored in this case if set.
    pub fn add_tier(
        &mut self,
        tier: Option<Tier>,
        stereotype: Option<&StereoType>,
    ) -> Result<(), EafError> {
        match tier {
            Some(t) => {
                let ext_time_order = TimeOrder::from_hashmap(t.lookup_timeslots());
                self.time_order.join(&ext_time_order); // TODO should remap, dedup if necessary as well
                                                       // println!("{:?}", stereotype);
                let lt = match stereotype {
                    Some(s) => LinguisticType::new(&t.linguistic_type_ref, Some(s)),
                    None => LinguisticType::default(), // "default-lt" for a main, alignable tier
                };

                if !self.linguistic_types.contains(&lt) {
                    self.add_linguistic_type(&lt, true)
                }

                self.tiers.push(t);
            }
            None => self.tiers.push(Tier::default()),
        }

        self.index();
        self.derive()?;

        Ok(())
    }

    pub fn add_linguistic_type(&mut self, ling_type: &LinguisticType, add_constraint: bool) {
        if add_constraint {
            match &ling_type.constraints {
                Some(s) => {
                    let c = Constraint::from_string(s);
                    // let c = Constraint::from(s.to_owned()); // From trait doesn't work?
                    if !self.constraints.contains(&c) {
                        self.add_constraint(&c)
                    }
                }
                None => {}
            }
        }
        self.linguistic_types.push(ling_type.to_owned())
    }

    pub fn add_constraint(&mut self, constraint: &Constraint) {
        self.constraints.push(constraint.to_owned())
    }

    /// Returns a list of all tier IDs.
    pub fn tier_ids(&self) -> Vec<String> {
        if self.indexed {
            self.index.t2a.keys()
                .cloned()
                .collect()
        } else {
            self.tiers.iter()
                .map(|t| t.tier_id.to_owned())
                .collect()
        }
    }

    /// Returns specified tier.
    pub fn get_tier(&self, id: &str) -> Option<&Tier> {
        if self.indexed {
            let t_idx = self.index.t2idx.get(id)?;
            self.tiers.get(*t_idx)
        } else {
            self.tiers.iter().find(|t| t.tier_id == id)
        }
    }

    /// Returns mutable tier.
    pub fn get_tier_mut(&mut self, id: &str) -> Option<&mut Tier> {
        if self.indexed {
            let t_idx = self.index.t2idx.get(id)?;
            self.tiers.get_mut(*t_idx)
        } else {
            self.tiers.iter_mut()
                .find(|t| t.tier_id == id)
        }
    }

    /// Change tier ID for existing tier,
    /// and ref tier ID for tiers referring to this tier ID.
    pub fn change_tier_id(&mut self, from: &str, to: &str) -> Result<(), EafError> {
        // self.tiers.iter_mut()
        // .find(|t| t.tier_id == from)
        // .map(|t| t.tier_id = to.to_owned())
        // .ok_or(EafError::InvalidTierId(to.to_owned()))?;
        // 2. change ref_tier for referred tiers
        for tier in self.tiers.iter_mut() {
            match tier.is_ref() {
                // 1. change ref_tier for referred tiers
                true => {
                    if tier.parent_ref.as_deref() == Some(from) {
                        tier.parent_ref = Some(to.to_owned())
                    }
                },
                // 1. change name of tier
                false => {
                    if &tier.tier_id == from {
                        tier.tier_id = to.to_owned()
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks if specified ID exists as either tier ID or annotation ID.
    /// Returns `(bool, bool)` for `(tier exists, annotation exists)`.
    pub fn exists(&self, id: &str) -> (bool, bool) {
        (
            // use Index to check if `id` exists.
            self.index.t2a.keys().any(|t| t == id), // tier id
            self.index.a2t.keys().any(|a| a == id), // annotation id
        )
    }
}
