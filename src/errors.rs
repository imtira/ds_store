use std::{array, error, fmt, io, result, str, string};

pub type Result<T> = result::Result<T, DSStoreError>;

#[derive(Debug)]
pub enum DSStoreError {
    HeaderTooSmall,
    BadMagic,
    OffsetsDontMatch,
    BadToc,
    BadString,
    TooLittleData(usize),
    Io(io::Error),
}

impl From<io::Error> for DSStoreError {
    fn from(error: io::Error) -> Self {
        DSStoreError::Io(error)
    }
}

impl From<array::TryFromSliceError> for DSStoreError {
    fn from(_error: array::TryFromSliceError) -> Self {
        DSStoreError::HeaderTooSmall
    }
}

impl From<str::Utf8Error> for DSStoreError {
    fn from(_error: str::Utf8Error) -> Self {
        DSStoreError::BadToc
    }
}

impl From<string::FromUtf16Error> for DSStoreError {
    fn from(_error: string::FromUtf16Error) -> Self {
        DSStoreError::BadString
    }
}

impl fmt::Display for DSStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DSStoreError::*;
        write!(
            f,
            "{}",
            match self {
                HeaderTooSmall => String::from("header size is less than 36b"),
                BadMagic => String::from("invalid magic"),
                OffsetsDontMatch => String::from("parity offets don't match"),
                BadToc => String::from("failed parsing toc key, bad offset?"),
                BadString => String::from("failed to convert utf16 to utf8"),
                TooLittleData(r) => format!("requested more data than available ({})", r),
                Io(err) => err.to_string(),
            }
        )
    }
}

impl error::Error for DSStoreError {}
