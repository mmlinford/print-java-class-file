use std::fmt::{Display, Formatter};

use crate::primitives::U2;

pub const MIN_MAJOR_VERSION: U2 = 45;
pub const MAX_MAJOR_VERSION: U2 = 61;

#[derive(Clone, Copy)]
pub enum JavaVersion {
    V1_0_2Or1_1,
    V1_2,
    V1_3,
    V1_4,
    V5_0,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
}

impl Display for JavaVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use JavaVersion::*;
        let as_str = match self {
            V1_0_2Or1_1 => "1.0.2 or 1.1",
            V1_2 => "1.2",
            V1_3 => "1.3",
            V1_4 => "1.4",
            V5_0 => "5.0",
            V6 => "6",
            V7 => "7",
            V8 => "8",
            V9 => "9",
            V10 => "10",
            V11 => "11",
            V12 => "12",
            V13 => "13",
            V14 => "14",
            V15 => "15",
            V16 => "16",
            V17 => "17",
        };

        f.write_str(as_str)
    }
}

impl TryFrom<U2> for JavaVersion {
    type Error = TryJavaVersionFromU2Error;

    fn try_from(value: U2) -> Result<Self, Self::Error> {
        use JavaVersion::*;
        match value {
            45 => Ok(V1_0_2Or1_1),
            46 => Ok(V1_2),
            47 => Ok(V1_3),
            48 => Ok(V1_4),
            49 => Ok(V5_0),
            50 => Ok(V6),
            51 => Ok(V7),
            52 => Ok(V8),
            53 => Ok(V9),
            54 => Ok(V10),
            55 => Ok(V11),
            56 => Ok(V12),
            57 => Ok(V13),
            58 => Ok(V14),
            59 => Ok(V15),
            60 => Ok(V16),
            61 => Ok(V17),
            ..MIN_MAJOR_VERSION | MAX_MAJOR_VERSION.. => Err(TryJavaVersionFromU2Error(value)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unrecognized or unsupported major version: {0}")]
pub struct TryJavaVersionFromU2Error(U2);
