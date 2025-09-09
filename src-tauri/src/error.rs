use serde_json;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Database(rusqlite::Error),
    Json(serde_json::Error),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Database(err) => write!(f, "Database error: {}", err),
            Error::Json(err) => write!(f, "JSON error: {}", err),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Database(err) => Some(err),
            Error::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::Database(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_display() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = Error::Io(io_err);
        assert_eq!(err.to_string(), "IO error: file not found");

        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err = Error::Json(json_err);
        assert!(err.to_string().starts_with("JSON error: "));

        let err = Error::Other("Something went wrong".to_string());
        assert_eq!(err.to_string(), "Error: Something went wrong");
    }

    #[test]
    fn test_error_conversions() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));

        let db_err = rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error {
                code: rusqlite::ffi::ErrorCode::ConstraintViolation,
                extended_code: 1555,
            },
            Some("UNIQUE constraint failed: users.email".to_string()),
        );
        let err: Error = db_err.into();
        assert!(matches!(err, Error::Database(_)));

        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: Error = json_err.into();
        assert!(matches!(err, Error::Json(_)));
    }
}
