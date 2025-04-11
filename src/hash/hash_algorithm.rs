use std::error::Error;
use std::{fmt, io};
use std::fmt::{Debug, Formatter};
use std::path::Path;

type Hash = Vec<u8>;

#[derive(Debug)]
pub enum HashAlgorithmError<P: AsRef<Path> = String> {
    ParseError {
        error: Box<dyn Error + 'static>,
        hash: String,
    },
    FileError {
        error: Box<io::Error>,
        path: P
    }
}

impl fmt::Display for HashAlgorithmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithmError::ParseError { error, hash, .. } => {
                write!(f, "error parsing hash `{}`: {}", hash, error)
            },
            HashAlgorithmError::FileError { error, path } => {
                write!(f, "error reading file '{}': {}", path, error)
            }
        }
    }
}

impl Error for HashAlgorithmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HashAlgorithmError::ParseError { error, .. } => Some(error.as_ref()),
            HashAlgorithmError::FileError { error, .. } => Some(error.as_ref()),
        }
    }
}

pub trait HashAlgorithm {
    fn hash(&self, file: &Path) -> Result<Hash, HashAlgorithmError>;

    fn compare(&self, left: &Hash, right: &Hash) -> bool;

    fn serialize(&self, hash: &Hash) -> Result<String, HashAlgorithmError> {
        Ok(hex::encode(hash))
    }
    fn deserialize(&self, hash: &str) -> Result<Hash, HashAlgorithmError> {
        match hex::decode(hash) {
            Ok(h) => Ok(h),
            Err(e) => Err(HashAlgorithmError::ParseError {
                error: Box::new(e),
                hash: hash.to_string(),
            }),
        }
    }
}
