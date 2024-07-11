use crate::bytes::hkx_header::HkxHeaderError;
use crate::bytes::deserializer::BytesDeError;
use crate::bytes::sections::class_name_section::ClassNamesSectionError;
use crate::bytes::sections::section_header::SectionHeaderError;
use std::ffi::FromBytesUntilNulError;

/// All Error [`Result`] of hkx_serde crate
pub type Result<T, E = HkxError> = core::result::Result<T, E>;

/// All Error of hkx_serde crate
#[derive(Debug, thiserror::Error)]
pub enum HkxError {
    /// Binary data deserialization
    #[error(transparent)]
    BytesDeError(#[from] BytesDeError),

    /// Unknown havok class {0}
    #[error("Unknown havok class {0}")]
    UnknownHavokClass(String),

    #[error(transparent)]
    HkxHeaderError(#[from] HkxHeaderError),

    #[error(transparent)]
    SectionHeaderError(#[from] SectionHeaderError),

    #[error(transparent)]
    ClassNamesSectionHeaderError(#[from] ClassNamesSectionError),

    /// Cstr must be non terminated string.
    #[error(transparent)]
    FromBytesUntilNulError(#[from] FromBytesUntilNulError),

    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
