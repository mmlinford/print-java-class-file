use std::num::TryFromIntError;
use crate::constant_pool::TryConstantKindFromU1Error;
use crate::java_version::TryJavaVersionFromU2Error;
use crate::modified_utf8::TryModifiedUtf8StringFromByteVecError;
use crate::primitives::{U1, U2, U4};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open input file: {0}")]
    FileOpen(std::io::Error),

    #[error("Failed to read primitive field: {field_name}")]
    ReadPrimitive {
        field_name: String,
        source: std::io::Error,
    },

    #[error("Magic number at beginning of file was incorrect: {0}")]
    IncorrectMagicNumber(U4),

    #[error("Major version {0} is not supported")]
    UnsupportedMajorVersion(U2),

    #[error("Invalid constant_pool_count: {0}")]
    InvalidConstantPoolCount(U2),

    #[error("Unsupported CpInfo tag: {0}")]
    UnsupportedCpInfoTag(U1),

    #[error("Failed to read buffer: {buffer_name}")]
    ReadBuffer {
        buffer_name: String,
        source: std::io::Error,
    },

    #[error("Attribute length was larger than is supported: attribute_name: {attribute_name} attribute_length: {attribute_length}")]
    AttributeLengthTooLarge {
        attribute_name: String,
        attribute_length: U4,
        source: TryFromIntError,
    },

    #[error(transparent)]
    TryJavaVersionFromU2(#[from] TryJavaVersionFromU2Error),

    #[error(transparent)]
    TryConstantKindFromU1(#[from] TryConstantKindFromU1Error),

    #[error(transparent)]
    InvalidUtf8Constant(#[from] TryModifiedUtf8StringFromByteVecError),
}
