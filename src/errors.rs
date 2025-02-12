//! Various errors that may arise when parsing, processing, and generating EAF-files.

use std::fmt;

use mp4iter::Mp4Error;

#[derive(Debug)]
/// Various errors for parsing, processing and building EAF-files.
pub enum EafError {
    // General

    /// No tiers in Eaf, no annotations in tier etc.
    NoData,
    /// General type mismatch, e.g. ref annotation in main annotation context
    TypeMismatch,

    // Annotation errors

    /// Annotation ID already exists (e.g. when adding new annotations).
    AnnotationIdExists(String),
    /// Invalid annotation ID.
    AnnotationIdInvalid(String),
    /// Annotation has no ID set.
    AnnotationIdMissing,
    /// Annotation ID is not set.
    AnnotationIdNotSet,
    /// Referred annotation has no ref ID set.
    AnnotationRefMissing,
    /// Missing main annotation for referred annotation.
    /// Not part of EAF specification.
    /// Value: `(ANNOTATION_ID, Option<REF_ANNOTATION>)`
    AnnotationMainMissing((String, Option<String>)),
    /// Encounterd referred annotation, expected main annotation.
    AnnotationMainExpected(String),
    /// Annotation type mismatch (when e.g. trying to add referred annotations to main tier)
    AnnotationTypeMismatch,
    /// Annotation timespan overlaps with another annotation in the same tier
    AnnotationOverlap,

    /// Missing file name (when e.g. trying to extract section from media file path)
    FileNameMissing(String),
    /// Missing file extension (when e.g. trying to extract section from media file path)
    FileExtensionMissing(String),
    /// Error reading media path into URI (prefixed `file://`).
    /// `Url::from_file_path` returns `Result<Url, ()>`
    UrlError(String),
    /// Missing EAF header
    HeaderMissing,

    // IO

    /// IO errors.
    IOError(std::io::Error),
    Mp4Error(Mp4Error),

    // Tier errors

    /// Number of annotations in ref tier
    /// exceed those in parent tier,
    /// or is not equal in number,
    /// depending on context.
    /// Value: `(parent_tier_id, ref_tier_id)`
    TierAlignmentError((String, String)),
    /// Invalid tier ID.
    TierIdInvalid(String),
    /// Unexpected tokenized tier.
    TierIsTokenized(String),
    /// Expected referred tier.
    /// Value: Tier ID for tested tier.
    TierRefExpected(String),
    /// Tiers incompatible for e.g. merging
    /// if one is a referred tier and the other is not.
    /// Value: `(tier_id_1, tier_id_2)`
    TierTypeMismatch((String, String)),
    /// Missing tier ID for annotation.
    /// For optional non-EAF fields,
    /// that are set on `Eaf::derive()`.
    TierIdMissing(String),
    /// Missing time order.
    TimeOrderMissing,
    /// Tier ID is not set.
    TierIdNotSet,
    /// Parent tier ID is not set.
    /// Only applies to referred tiers.
    TierRefIdNotSet,
    /// The main tier does not have any annotations,
    /// whereas the referred tier does, meaning there
    /// are no "main" annotations to refer to.
    TierRefEmptyParent,
    /// Referred tier has no parent.
    /// Value: Tier ID for referred tier.
    TierRefMissingParent(String),
    /// Encounterd referred tier, expected main tier.
    TierMainExpected(String),

    // Time slot errors

    /// `TimeOrder` contains duplicate time slot IDs.
    TimeslotIDDuplicated,
    /// Invalid time slot ID.
    TimeslotIdInvalid(String),
    /// Missing specific time slot reference for annotation ID.
    TimeslotRefMissing(String),
    /// Failed to sort time slots due non-standard time slot naming.
    TimeslotSortingError,
    /// Missing one or more time slot reference/s.
    TimeslotRefsMissing,
    /// Missing start time slot reference/s.
    TimeslotRef1Missing,
    /// Missing end time slot reference/s.
    TimeslotRef2Missing,
    /// Missing time slot value for annotation ID (not part of EAF specification).
    TimeslotValMissing(String),
    /// Error when filtering media, time slots etc on time.
    TimeSpanInvalid((i64, i64)),
    /// Timeslot ID already exists (e.g. when adding new timeslots).
    TimeSlotIdExists(String),

    // Other errors

    /// Error parsing integer from string.
    ParseIntError(std::num::ParseIntError),
    /// Error parsing float from string.
    ParseFloatError(std::num::ParseFloatError),
    /// Quick-xml error.
    QuickXMLError(quick_xml::Error),
    /// Quick-xml deserialization error.
    QuickXMLDeError(quick_xml::DeError),
    QuickXMLSeError(quick_xml::SeError),
    /// Invalid path.
    PathInvalid(String),
    /// Error decoding string as UTF-8.
    Utf8Error(std::str::Utf8Error),
    /// Value is too small to be used in this context.
    /// E.g. negative time slot values.
    ValueTooSmall(i64),
    /// Value is too large to be used in this context.
    /// E.g. time slot value exceeds media duration.
    ValueTooLarge(i64),
    /// Missing namespace location
    XmlNameSpaceMissing,
    /// Missing no namespace location
    XmlNoNameSpaceMissing,
}

impl std::error::Error for EafError {}
impl fmt::Display for EafError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EafError::NoData => write!(f, "Input is empty or contains no relevant data"),
            EafError::TypeMismatch => write!(f, "Objects have different types"),
            EafError::HeaderMissing => write!(f, "Missing EAF header"),
            EafError::IOError(err) => write!(f, "IO error: {}", err),
            EafError::Mp4Error(err) => write!(f, "MP4 error: {}", err),
            EafError::TierAlignmentError((t1, t2)) => write!(f,
                "Annotations in referred tier '{t2}' exceed or is not equal to those in parent tier '{t1}'"),
            EafError::TierRefExpected(e) => {
                write!(f, "Expected referred tier. '{}' is a main tier.", e)
            }
            EafError::XmlNameSpaceMissing => write!(f, "Missing XML namespace locatation"),
            EafError::XmlNoNameSpaceMissing => write!(f, "Missing XML no namespace locatation"),
            EafError::TierTypeMismatch((id1, id2)) => write!(f, "The tiers '{id1}' and '{id2}' do not have compatible type."),
            EafError::Utf8Error(err) => write!(f, "Error parsing bytes to string: {}", err),
            EafError::QuickXMLError(err) => write!(f, "QuickXML error parsing EAF: {}", err),
            EafError::QuickXMLDeError(err) => write!(f, "QuickXML error deserialising EAF: {}", err),
            EafError::QuickXMLSeError(err) => write!(f, "QuickXML error serialising EAF: {}", err),
            EafError::ParseIntError(err) => write!(f, "Error parsing string to integer: {}", err),
            EafError::ParseFloatError(err) => write!(f, "Error parsing string to float: {}", err),
            EafError::TierIsTokenized(tier_id) => write!(f, "'{}' is a tokenized tier", tier_id),
            EafError::TierIdInvalid(tier_id) => write!(f, "No such tier '{}'", tier_id),
            EafError::AnnotationIdInvalid(annotation_id) => write!(f, "No such annotation '{}'", annotation_id),
            EafError::TimeslotIdInvalid(time_slot_id) => write!(f, "No such time slot '{}'", time_slot_id),
            EafError::TimeslotRefMissing(annotation_id) => write!(f, "No time slot reference for annotation/s with ID {}.", annotation_id),
            EafError::TimeslotSortingError => write!(f, "Failed to sort time slots. Possibly due to non-standard naming."),
            EafError::TimeslotIDDuplicated => write!(f, "Time slot ID must be unique."),
            EafError::TimeslotRefsMissing => write!(f, "Missing one or more time slot references."),
            EafError::TimeslotRef1Missing => write!(f, "Missing start time slot reference."),
            EafError::TimeslotRef2Missing => write!(f, "Missing end time slot reference."),
            EafError::TimeslotValMissing(annotation_val) => write!(f, "No time slot value for annotation/s with ID {}.", annotation_val),
            EafError::TimeSpanInvalid((start, end)) => write!(f, "Invalid time span {}ms-{}ms", start, end),
            EafError::TierIdMissing(annotation_id) => write!(f, "Tier ID not set for annotation with ID '{}'", annotation_id),
            EafError::AnnotationIdMissing => write!(f, "Annotation ID not set"),
            EafError::TimeOrderMissing => write!(f, "Missing time order"),
            EafError::AnnotationIdNotSet => write!(f, "Annotation ID not set"),
            EafError::TierIdNotSet => write!(f, "Tier ID not set"),
            EafError::TierRefMissingParent(id) => write!(f,
                "Referred tier parent ref ID is not valid or tier is missing"),
            EafError::TierRefIdNotSet => write!(f, "Parent tier ID not set"),
            EafError::TierRefEmptyParent => write!(f, "Parent tier can not be empty"),
            EafError::AnnotationRefMissing => write!(f, "Annotation ref ID not set for referred annotation"),
            EafError::AnnotationMainMissing((annotation_id, ref_annotation)) => write!(
                f, "Missing main annotation for ID '{}'. No main annotation with ID '{}'",
                annotation_id,
                ref_annotation.as_deref().unwrap_or_else(|| "NONE")),
            EafError::TimeSlotIdExists(id) => write!(f, "Timeslot with ID '{}' already exists", id),
            EafError::AnnotationIdExists(id) => write!(f, "Annotation with ID '{}' already exists", id),
            EafError::TierMainExpected(tier_id) => write!(f, "Expected main tier. '{}' is referred.", tier_id),
            EafError::AnnotationMainExpected(annotation_id) => write!(f,
                "Expected annotation on main tier. '{}' is referred", annotation_id),
            EafError::AnnotationTypeMismatch => write!(f, "Annotation types do not match or annotation type is incompatible with tier type"),
            EafError::AnnotationOverlap => write!(f, "Annotation timespans overlap in the same tier"),
            EafError::FileNameMissing(path) => write!(f, "No file name in path '{}'", path),
            EafError::FileExtensionMissing(path) => write!(f, "No file extion in path '{}'", path),
            EafError::UrlError(path) => write!(f, "Failed to convert path to UNC for {}", path),
            EafError::PathInvalid(path) => write!(f, "No such file '{}'", path),
            EafError::ValueTooSmall(num) => write!(f, "Value '{}' is too small in this context.", num),
            EafError::ValueTooLarge(num) => write!(f, "Value '{}' is too large in this context.", num),
        }
    }
}

/// Converts std::str::Utf8Error to EafError
impl From<std::str::Utf8Error> for EafError {
    fn from(err: std::str::Utf8Error) -> EafError {
        EafError::Utf8Error(err)
    }
}

/// Converts EafError to std::io::Error
impl From<EafError> for std::io::Error {
    fn from(err: EafError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning EafErrors in main()
    }
}

/// Converts mp4iter::Mp4Error to EafError
impl From<mp4iter::Mp4Error> for EafError {
    fn from(err: mp4iter::Mp4Error) -> EafError {
        EafError::Mp4Error(err)
    }
}

/// Converts std::io::Error to EafError
impl From<std::io::Error> for EafError {
    fn from(err: std::io::Error) -> EafError {
        EafError::IOError(err)
    }
}

/// Converts quick_xml::Error to EafError
impl From<quick_xml::Error> for EafError {
    fn from(err: quick_xml::Error) -> EafError {
        EafError::QuickXMLError(err)
    }
}

/// Converts quick_xml::DeError to EafError
impl From<quick_xml::DeError> for EafError {
    fn from(err: quick_xml::DeError) -> EafError {
        EafError::QuickXMLDeError(err)
    }
}

/// Converts quick_xml::SeError to EafError
impl From<quick_xml::SeError> for EafError {
    fn from(err: quick_xml::SeError) -> EafError {
        EafError::QuickXMLSeError(err)
    }
}

/// Converts std::num::ParseIntError to EafError
impl From<std::num::ParseIntError> for EafError {
    fn from(err: std::num::ParseIntError) -> EafError {
        EafError::ParseIntError(err)
    }
}

/// Converts std::num::ParseFloatError to EafError
impl From<std::num::ParseFloatError> for EafError {
    fn from(err: std::num::ParseFloatError) -> EafError {
        EafError::ParseFloatError(err)
    }
}
