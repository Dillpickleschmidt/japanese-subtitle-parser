use std::error::Error as StdError;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Database(rusqlite::Error),
    NotFound(String),
    InvalidInput(String),
    Other(String),
}

// Implement the Display trait for the Error enum to format error messages consistently.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Database(err) => write!(f, "Database error: {}", err),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// Implement the Error trait for the Error enum to allow it to be used with the standard library's error
// handling mechanisms.
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Database(err) => Some(err),
            _ => None,
        }
    }
}

// Implement the From trait for the Error enum to allow automatic conversion from standard library errors
// or third-party library errors into your custom Error type.
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

// Define a type alias for the Result type that uses the custom Error type as the error variant to be used
// throughout the application.
pub type Result<T> = result::Result<T, Error>;

// Helper functions - These functions provide a convenient way to create specific error variants.
impl Error {
    pub fn not_found(message: &str) -> Self {
        Error::NotFound(message.to_string())
    }

    pub fn invalid_input(message: &str) -> Self {
        Error::InvalidInput(message.to_string())
    }

    pub fn other(message: &str) -> Self {
        Error::Other(message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_display() {
        let err = Error::NotFound("User 123".to_string());
        assert_eq!(err.to_string(), "Not found: User 123");

        let err = Error::InvalidInput("Invalid email".to_string());
        assert_eq!(err.to_string(), "Invalid input: Invalid email");

        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = Error::Io(io_err);
        assert_eq!(err.to_string(), "IO error: file not found");
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
    }

    #[test]
    fn test_error_helper_functions() {
        let err = Error::not_found("User 123");
        assert_eq!(err.to_string(), "Not found: User 123");

        let err = Error::invalid_input("Email must not be empty");
        assert_eq!(err.to_string(), "Invalid input: Email must not be empty");

        let err = Error::other("Unexpected error occurred");
        assert_eq!(err.to_string(), "Error: Unexpected error occurred");
    }
}
