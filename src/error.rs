//! Application error types and exit codes.
//!
//! Provides a unified error type for the application with appropriate
//! exit codes for different failure modes.

use std::io;
use thiserror::Error;

/// Application error type covering all failure modes.
///
/// Each variant maps to a specific exit code for scripting compatibility.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl AppError {
    /// Returns the exit code for this error type.
    ///
    /// Exit codes:
    /// - 2: Authentication failure (401/403)
    /// - 3: API error
    /// - 4: Invalid query syntax
    /// - 5: Configuration error
    /// - 6: IO error
    /// - 7: Serialization error
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::Auth(_) => 2,
            AppError::Api(_) => 3,
            AppError::InvalidQuery(_) => 4,
            AppError::Config(_) => 5,
            AppError::Io(_) => 6,
            AppError::Serialization(_) => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_error_exit_code() {
        let error = AppError::Auth("test".to_string());
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn test_api_error_exit_code() {
        let error = AppError::Api("test".to_string());
        assert_eq!(error.exit_code(), 3);
    }

    #[test]
    fn test_invalid_query_error_exit_code() {
        let error = AppError::InvalidQuery("test".to_string());
        assert_eq!(error.exit_code(), 4);
    }

    #[test]
    fn test_config_error_exit_code() {
        let error = AppError::Config("test".to_string());
        assert_eq!(error.exit_code(), 5);
    }

    #[test]
    fn test_io_error_exit_code() {
        let error = AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        assert_eq!(error.exit_code(), 6);
    }

    #[test]
    fn test_serialization_error_exit_code() {
        let invalid_json = "invalid json";
        let error: AppError = serde_json::from_str::<serde_json::Value>(invalid_json)
            .unwrap_err()
            .into();
        assert_eq!(error.exit_code(), 7);
    }

    #[test]
    fn test_error_display() {
        let auth_error = AppError::Auth("invalid credentials".to_string());
        assert!(format!("{}", auth_error).contains("Authentication failed"));
        assert!(format!("{}", auth_error).contains("invalid credentials"));

        let api_error = AppError::Api("connection failed".to_string());
        assert!(format!("{}", api_error).contains("API error"));
        assert!(format!("{}", api_error).contains("connection failed"));
    }
}
