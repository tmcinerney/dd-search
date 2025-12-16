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
    use serial_test::serial;

    // Helper to set up a clean environment state and run a test
    // This ensures proper isolation and cleanup
    fn with_env<F, R>(vars: &[(&str, Option<&str>)], test_fn: F) -> R
    where
        F: FnOnce() -> R,
    {
        // Save original values
        let originals: Vec<_> = vars
            .iter()
            .map(|(key, _)| (*key, std::env::var(key).ok()))
            .collect();

        // Set up test environment
        for (key, value) in vars {
            // SAFETY: We're in a serial test, so no race conditions
            unsafe {
                if let Some(val) = value {
                    std::env::set_var(key, val);
                } else {
                    std::env::remove_var(key);
                }
            }
        }

        // Run the test
        let result = test_fn();

        // Restore original values
        for (key, original) in originals {
            // SAFETY: We're in a serial test, so no race conditions
            unsafe {
                if let Some(val) = original {
                    std::env::set_var(key, val);
                } else {
                    std::env::remove_var(key);
                }
            }
        }

        result
    }

    #[test]
    #[serial]
    fn test_load_config_success() {
        with_env(
            &[
                ("DD_API_KEY", Some("test-api-key")),
                ("DD_APP_KEY", Some("test-app-key")),
            ],
            || {
                let result = load_config();
                assert!(
                    result.is_ok(),
                    "load_config should succeed with valid credentials"
                );
            },
        );
    }

    #[test]
    #[serial]
    fn test_load_config_missing_api_key() {
        with_env(
            &[
                ("DD_API_KEY", None), // Remove DD_API_KEY
                ("DD_APP_KEY", Some("test-app-key")),
            ],
            || {
                let result = load_config();
                assert!(
                    result.is_err(),
                    "load_config should fail when DD_API_KEY is missing"
                );

                if let Err(AppError::Config(msg)) = result {
                    assert!(
                        msg.contains("DD_API_KEY"),
                        "Error message should mention DD_API_KEY, got: {}",
                        msg
                    );
                    assert!(
                        msg.contains("not set"),
                        "Error message should mention 'not set', got: {}",
                        msg
                    );
                } else {
                    panic!("Expected Config error, got: {:?}", result);
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_load_config_missing_app_key() {
        with_env(
            &[
                ("DD_API_KEY", Some("test-api-key")),
                ("DD_APP_KEY", None), // Remove DD_APP_KEY
            ],
            || {
                let result = load_config();
                assert!(
                    result.is_err(),
                    "load_config should fail when DD_APP_KEY is missing"
                );

                if let Err(AppError::Config(msg)) = result {
                    assert!(
                        msg.contains("DD_APP_KEY"),
                        "Error message should mention DD_APP_KEY, got: {}",
                        msg
                    );
                } else {
                    panic!("Expected Config error");
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_load_config_empty_api_key() {
        with_env(
            &[
                ("DD_API_KEY", Some("")), // Set DD_API_KEY to empty string
                ("DD_APP_KEY", Some("test-app-key")),
            ],
            || {
                let result = load_config();
                assert!(
                    result.is_err(),
                    "load_config should fail when DD_API_KEY is empty"
                );

                if let Err(AppError::Config(msg)) = result {
                    assert!(
                        msg.contains("DD_API_KEY"),
                        "Error message should mention DD_API_KEY, got: {}",
                        msg
                    );
                    assert!(
                        msg.contains("empty"),
                        "Error message should mention 'empty', got: {}",
                        msg
                    );
                } else {
                    panic!("Expected Config error, got: {:?}", result);
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_load_config_empty_app_key() {
        with_env(
            &[
                ("DD_API_KEY", Some("test-api-key")),
                ("DD_APP_KEY", Some("")), // Set DD_APP_KEY to empty string
            ],
            || {
                let result = load_config();
                assert!(
                    result.is_err(),
                    "load_config should fail when DD_APP_KEY is empty"
                );

                if let Err(AppError::Config(msg)) = result {
                    assert!(
                        msg.contains("DD_APP_KEY"),
                        "Error message should mention DD_APP_KEY, got: {}",
                        msg
                    );
                    assert!(
                        msg.contains("empty"),
                        "Error message should mention 'empty', got: {}",
                        msg
                    );
                } else {
                    panic!("Expected Config error");
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_load_config_with_site() {
        with_env(
            &[
                ("DD_API_KEY", Some("test-api-key")),
                ("DD_APP_KEY", Some("test-app-key")),
                ("DD_SITE", Some("datadoghq.eu")),
            ],
            || {
                let result = load_config();
                // Should succeed even with DD_SITE set (it's optional)
                assert!(
                    result.is_ok(),
                    "load_config should succeed with DD_SITE set"
                );
            },
        );
    }
}
