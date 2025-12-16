//! Configuration loading from environment variables.
//!
//! Validates that required Datadog credentials are set before creating
//! the API client configuration.

use datadog_api_client::datadog::Configuration;

use crate::error::AppError;

/// Loads and validates Datadog configuration from environment variables.
///
/// # Required Environment Variables
///
/// - `DD_API_KEY` - Datadog API key
/// - `DD_APP_KEY` - Datadog application key
///
/// # Optional Environment Variables
///
/// - `DD_SITE` - Datadog site (defaults to `datadoghq.com`)
///
/// # Errors
///
/// Returns `AppError::Config` if required environment variables are missing or empty.
pub fn load_config() -> Result<Configuration, AppError> {
    let api_key = std::env::var("DD_API_KEY")
        .map_err(|_| AppError::Config("DD_API_KEY environment variable not set".into()))?;

    let app_key = std::env::var("DD_APP_KEY")
        .map_err(|_| AppError::Config("DD_APP_KEY environment variable not set".into()))?;

    if api_key.is_empty() {
        return Err(AppError::Config("DD_API_KEY is empty".into()));
    }
    if app_key.is_empty() {
        return Err(AppError::Config("DD_APP_KEY is empty".into()));
    }

    // DD_SITE is optional - the SDK reads it automatically
    // Defaults to datadoghq.com if not set

    Ok(Configuration::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_env(key: &str, value: &str) {
        // SAFETY: Tests are isolated and we clean up after ourselves
        unsafe {
            std::env::set_var(key, value);
        }
    }

    fn remove_env(key: &str) {
        // SAFETY: Tests are isolated and we clean up after ourselves
        unsafe {
            std::env::remove_var(key);
        }
    }

    #[test]
    fn test_load_config_success() {
        // Ensure clean state
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        
        set_env("DD_API_KEY", "test-api-key");
        set_env("DD_APP_KEY", "test-app-key");

        let result = load_config();
        assert!(result.is_ok());

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
    }

    #[test]
    fn test_load_config_missing_api_key() {
        // Ensure clean state
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        
        // DD_API_KEY is not set
        set_env("DD_APP_KEY", "test-app-key");

        let result = load_config();
        assert!(result.is_err());
        if let Err(AppError::Config(msg)) = result {
            assert!(msg.contains("DD_API_KEY"), "Error message should mention DD_API_KEY, got: {}", msg);
        } else {
            panic!("Expected Config error");
        }

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
    }

    #[test]
    fn test_load_config_missing_app_key() {
        // Ensure clean state
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        
        set_env("DD_API_KEY", "test-api-key");
        // DD_APP_KEY is not set

        let result = load_config();
        assert!(result.is_err());
        if let Err(AppError::Config(msg)) = result {
            assert!(msg.contains("DD_APP_KEY"), "Error message should mention DD_APP_KEY, got: {}", msg);
        } else {
            panic!("Expected Config error");
        }

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
    }

    #[test]
    fn test_load_config_empty_api_key() {
        // Ensure clean state
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        
        set_env("DD_API_KEY", "");
        set_env("DD_APP_KEY", "test-app-key");

        let result = load_config();
        assert!(result.is_err());
        if let Err(AppError::Config(msg)) = result {
            assert!(msg.contains("DD_API_KEY"), "Error message should mention DD_API_KEY, got: {}", msg);
            assert!(msg.contains("empty"), "Error message should mention 'empty', got: {}", msg);
        } else {
            panic!("Expected Config error");
        }

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
    }

    #[test]
    fn test_load_config_empty_app_key() {
        // Ensure clean state
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        
        set_env("DD_API_KEY", "test-api-key");
        set_env("DD_APP_KEY", "");

        let result = load_config();
        assert!(result.is_err());
        if let Err(AppError::Config(msg)) = result {
            assert!(msg.contains("DD_APP_KEY"), "Error message should mention DD_APP_KEY, got: {}", msg);
            assert!(msg.contains("empty"), "Error message should mention 'empty', got: {}", msg);
        } else {
            panic!("Expected Config error");
        }

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
    }

    #[test]
    fn test_load_config_with_site() {
        set_env("DD_API_KEY", "test-api-key");
        set_env("DD_APP_KEY", "test-app-key");
        set_env("DD_SITE", "datadoghq.eu");

        let result = load_config();
        // Should succeed even with DD_SITE set (it's optional)
        assert!(result.is_ok());

        // Cleanup
        remove_env("DD_API_KEY");
        remove_env("DD_APP_KEY");
        remove_env("DD_SITE");
    }
}
