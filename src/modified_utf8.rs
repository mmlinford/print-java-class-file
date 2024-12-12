use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

pub struct ModifiedUtf8String(String);

impl Display for ModifiedUtf8String {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<Vec<u8>> for ModifiedUtf8String {
    type Error = TryModifiedUtf8StringFromByteVecError;

    // TODO: check for the other restrictions on Modified UTF-8
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let inner =
            String::from_utf8(value).map_err(TryModifiedUtf8StringFromByteVecError::FromUtf8)?;
        Ok(ModifiedUtf8String(inner))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to read byte buffer as valid Modified UTF-8")]
pub enum TryModifiedUtf8StringFromByteVecError {
    FromUtf8(#[from] FromUtf8Error),
}
