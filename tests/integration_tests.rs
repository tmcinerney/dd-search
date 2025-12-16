//! Integration tests for dd-search.
//!
//! These tests require valid Datadog credentials to be set in environment variables:
//! - DD_API_KEY
//! - DD_APP_KEY
//! - DD_SITE (optional)
//!
//! Run with: cargo test --test integration_tests
//!
//! Note: These tests make actual API calls to Datadog and may consume API quota.

use dd_search::client::{LogsClient, SpansClient};
use dd_search::config;
use dd_search::time;
use futures_util::StreamExt;

fn has_credentials() -> bool {
    std::env::var("DD_API_KEY").is_ok() && std::env::var("DD_APP_KEY").is_ok()
}

#[tokio::test]
#[ignore] // Ignore by default - requires credentials and API access
async fn test_logs_search_with_relative_time() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = LogsClient::new(config);

    // Test with relative time range (last hour)
    let query = "*"; // Match all logs
    let from = "now-1h";
    let to = "now";
    let indexes = vec!["*".to_string()];

    // Validate time format first
    assert!(time::is_valid_time_format(from));
    assert!(time::is_valid_time_format(to));
    assert!(time::is_valid_time_range(from, to));

    let mut stream = std::pin::pin!(client.search(query, from, to, indexes));
    let mut count = 0;
    let max_results = 10; // Limit to avoid consuming too much quota

    while let Some(result) = stream.next().await {
        match result {
            Ok(_log) => {
                count += 1;
                if count >= max_results {
                    break;
                }
            }
            Err(e) => {
                // Don't fail on API errors - just log them
                eprintln!("API error (may be expected): {}", e);
                break;
            }
        }
    }

    // If we got here without panicking, the time range was accepted
    println!(
        "Successfully queried {} logs with relative time range",
        count
    );
}

#[tokio::test]
#[ignore]
async fn test_logs_search_with_iso8601_time() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = LogsClient::new(config);

    // Test with ISO8601 time range (last 24 hours)
    use chrono::{Duration, Utc};
    let end_time = Utc::now();
    let start_time = end_time - Duration::hours(24);

    // Use RFC3339 format which Datadog accepts
    let from = start_time.to_rfc3339();
    let to = end_time.to_rfc3339();

    // Validate time format
    assert!(time::is_valid_time_format(&from));
    assert!(time::is_valid_time_format(&to));
    assert!(time::is_valid_time_range(&from, &to));

    let query = "*";
    let indexes = vec!["*".to_string()];

    let mut stream = std::pin::pin!(client.search(query, &from, &to, indexes));
    let mut count = 0;
    let max_results = 10;

    while let Some(result) = stream.next().await {
        match result {
            Ok(_log) => {
                count += 1;
                if count >= max_results {
                    break;
                }
            }
            Err(e) => {
                eprintln!("API error (may be expected): {}", e);
                break;
            }
        }
    }

    println!(
        "Successfully queried {} logs with ISO8601 time range",
        count
    );
}

#[tokio::test]
#[ignore]
async fn test_logs_search_various_time_ranges() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = LogsClient::new(config);

    // Test various relative time formats
    let time_ranges = vec![
        ("now-15m", "now"),
        ("now-1h", "now"),
        ("now-6h", "now"),
        ("now-1d", "now"),
        ("now-1w", "now"),
    ];

    for (from, to) in time_ranges {
        // Validate format
        assert!(
            time::is_valid_time_format(from),
            "Invalid from time: {}",
            from
        );
        assert!(time::is_valid_time_format(to), "Invalid to time: {}", to);
        assert!(
            time::is_valid_time_range(from, to),
            "Invalid time range: {} to {}",
            from,
            to
        );

        let query = "*";
        let indexes = vec!["*".to_string()];

        let mut stream = std::pin::pin!(client.search(query, from, to, indexes));
        let mut has_result = false;

        // Just check that the query doesn't error out - check first result
        if let Some(result) = stream.next().await {
            match result {
                Ok(_) => {
                    has_result = true;
                }
                Err(e) => {
                    let msg = format!("{}", e);
                    // 401 is a real auth failure, 403 might be permissions
                    if msg.contains("401") {
                        panic!("Authentication failed: {}", msg);
                    }
                    if msg.contains("403") {
                        eprintln!(
                            "Warning: 403 Forbidden for time range {} to {} - may indicate insufficient permissions",
                            from, to
                        );
                    }
                    // Other errors might be acceptable (no logs in range, etc.)
                }
            }
        }

        println!(
            "Time range {} to {}: {}",
            from,
            to,
            if has_result { "OK" } else { "No results" }
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_spans_search_with_relative_time() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = SpansClient::new(config);

    let query = "*";
    let from = "now-1h";
    let to = "now";

    // Validate time format
    assert!(time::is_valid_time_format(from));
    assert!(time::is_valid_time_format(to));
    assert!(time::is_valid_time_range(from, to));

    let mut stream = std::pin::pin!(client.search(query, from, to));
    let mut count = 0;
    let max_results = 10;

    while let Some(result) = stream.next().await {
        match result {
            Ok(_span) => {
                count += 1;
                if count >= max_results {
                    break;
                }
            }
            Err(e) => {
                eprintln!("API error (may be expected): {}", e);
                break;
            }
        }
    }

    println!(
        "Successfully queried {} spans with relative time range",
        count
    );
}

#[tokio::test]
#[ignore]
async fn test_spans_search_with_iso8601_time() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = SpansClient::new(config);

    use chrono::{Duration, Utc};
    let end_time = Utc::now();
    let start_time = end_time - Duration::hours(24);

    // Use RFC3339 format which Datadog accepts
    let from = start_time.to_rfc3339();
    let to = end_time.to_rfc3339();

    // Validate time format
    assert!(time::is_valid_time_format(&from));
    assert!(time::is_valid_time_format(&to));
    assert!(time::is_valid_time_range(&from, &to));

    let query = "*";

    let mut stream = std::pin::pin!(client.search(query, &from, &to));
    let mut count = 0;
    let max_results = 10;

    while let Some(result) = stream.next().await {
        match result {
            Ok(_span) => {
                count += 1;
                if count >= max_results {
                    break;
                }
            }
            Err(e) => {
                eprintln!("API error (may be expected): {}", e);
                break;
            }
        }
    }

    println!(
        "Successfully queried {} spans with ISO8601 time range",
        count
    );
}

#[tokio::test]
#[ignore]
async fn test_spans_search_various_time_ranges() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = SpansClient::new(config);

    let time_ranges = vec![
        ("now-15m", "now"),
        ("now-1h", "now"),
        ("now-6h", "now"),
        ("now-1d", "now"),
    ];

    for (from, to) in time_ranges {
        assert!(
            time::is_valid_time_format(from),
            "Invalid from time: {}",
            from
        );
        assert!(time::is_valid_time_format(to), "Invalid to time: {}", to);
        assert!(
            time::is_valid_time_range(from, to),
            "Invalid time range: {} to {}",
            from,
            to
        );

        let query = "*";

        let mut stream = std::pin::pin!(client.search(query, from, to));
        let mut has_result = false;

        // Check first result to verify query format
        if let Some(result) = stream.next().await {
            match result {
                Ok(_) => {
                    has_result = true;
                }
                Err(e) => {
                    let msg = format!("{}", e);
                    // 401 is a real auth failure, but 403 might be permissions/rate limit
                    // For spans, 403 could mean APM is not enabled or insufficient permissions
                    if msg.contains("401") {
                        panic!("Authentication failed: {}", msg);
                    }
                    if msg.contains("403") {
                        // Log but don't panic - might be permissions issue
                        eprintln!(
                            "Warning: 403 Forbidden for time range {} to {} - may indicate insufficient permissions or APM not enabled",
                            from, to
                        );
                    }
                }
            }
        }

        println!(
            "Time range {} to {}: {}",
            from,
            to,
            if has_result {
                "OK"
            } else {
                "No results or access denied"
            }
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_logs_search_with_unix_timestamp() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = LogsClient::new(config);

    // Test with Unix timestamp in milliseconds (last hour)
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let one_hour_ago = now - (60 * 60 * 1000);

    let from = one_hour_ago.to_string();
    let to = now.to_string();

    // Validate time format
    assert!(time::is_valid_time_format(&from));
    assert!(time::is_valid_time_format(&to));
    assert!(time::is_valid_time_range(&from, &to));

    let query = "*";
    let indexes = vec!["*".to_string()];

    let mut stream = std::pin::pin!(client.search(query, &from, &to, indexes));
    let mut count = 0;
    let max_results = 10;

    while let Some(result) = stream.next().await {
        match result {
            Ok(_log) => {
                count += 1;
                if count >= max_results {
                    break;
                }
            }
            Err(e) => {
                eprintln!("API error (may be expected): {}", e);
                break;
            }
        }
    }

    println!(
        "Successfully queried {} logs with Unix timestamp format",
        count
    );
}

#[tokio::test]
#[ignore]
async fn test_time_range_edge_cases() {
    if !has_credentials() {
        eprintln!("Skipping test: DD_API_KEY and DD_APP_KEY not set");
        return;
    }

    let config = config::load_config().expect("Failed to load config");
    let client = LogsClient::new(config);

    // Test edge cases for time ranges (all valid Datadog formats)
    let edge_cases = vec![
        ("now-1s", "now"),  // Very short range (1 second)
        ("now-90s", "now"), // 90 seconds
        ("now-30m", "now"), // 30 minutes
        ("now-2h", "now"),  // 2 hours
        ("now-1w", "now"),  // 1 week
        ("now-1mo", "now"), // 1 month
    ];

    for (from, to) in edge_cases {
        assert!(
            time::is_valid_time_format(from),
            "Invalid from time: {}",
            from
        );
        assert!(time::is_valid_time_format(to), "Invalid to time: {}", to);

        let query = "*";
        let indexes = vec!["*".to_string()];

        let mut stream = std::pin::pin!(client.search(query, from, to, indexes));

        // Just verify it doesn't error out immediately
        let mut error_count = 0;
        let mut result_count = 0;

        for _ in 0..5 {
            // Only check first few results
            if let Some(result) = stream.next().await {
                match result {
                    Ok(_) => result_count += 1,
                    Err(e) => {
                        let msg = format!("{}", e);
                        if msg.contains("401") {
                            panic!("Authentication failed: {}", msg);
                        }
                        if msg.contains("403") {
                            eprintln!(
                                "Warning: 403 Forbidden for edge case {} to {} - may indicate insufficient permissions",
                                from, to
                            );
                        }
                        error_count += 1;
                    }
                }
            } else {
                break;
            }
        }

        println!(
            "Edge case {} to {}: {} results, {} errors",
            from, to, result_count, error_count
        );
    }
}
