use core::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum Error {
    ShmName,
    Libc {
        name: &'static str,
        source: io::Error,
    },
}

impl Error {
    pub(crate) fn is_not_found(&self) -> bool {
        match self {
            Error::Libc { name: _, source } => matches!(source.kind(), io::ErrorKind::NotFound),
            _ => false,
        }
    }

    pub(crate) fn is_already_exists(&self) -> bool {
        match self {
            Error::Libc { name: _, source } => {
                matches!(source.kind(), io::ErrorKind::AlreadyExists)
            }
            _ => false,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShmName => write!(
                f,
                "shm name must at most {} bytes",
                crate::backend::Shm::MAX_LEN,
            ),
            Self::Libc { name, source: _ } => write!(f, "{name} error"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ShmName => None,
            Self::Libc { name: _, source } => Some(source),
        }
    }
}
