//! Main CLI argument definitions.

use clap::{Parser, Subcommand};

use super::logs::LogsAction;
use super::metrics::MetricsAction;
use super::spans::SpansAction;

/// Main CLI application structure.
#[derive(Parser, Debug)]
#[command(name = "ddog")]
#[command(about = "Query Datadog logs, APM spans, and metrics from the command line")]
#[command(
    long_about = "Query Datadog logs, APM spans, and metrics from the command line.

Environment Variables (Required):
  DD_API_KEY     Your Datadog API key
  DD_APP_KEY     Your Datadog application key
  DD_SITE        Datadog site (optional, default: datadoghq.com)

Output Format:
  All commands output newline-delimited JSON (NDJSON), one record per line.
  Perfect for piping to jq, grep, or other line-oriented tools.

Examples:
  # Search logs for errors
  ddog logs search \"service:api AND status:error\"

  # Query metrics with jq
  ddog metrics query \"avg:system.cpu.user{*}\" | jq .value

  # Search spans with time range
  ddog spans search \"service:web\" --from now-1h --to now

Documentation:
  https://github.com/tmcinerney/ddog"
)]
#[command(version)]
pub struct Cli {
    /// Enable verbose/debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub domain: Domain,
}

/// Available domains for querying Datadog.
#[derive(Subcommand, Debug)]
pub enum Domain {
    /// Logs domain - search and analyze logs
    Logs {
        #[command(subcommand)]
        action: LogsAction,
    },

    /// Spans domain - search and analyze APM traces
    Spans {
        #[command(subcommand)]
        action: SpansAction,
    },

    /// Metrics domain - query and list metrics
    Metrics {
        #[command(subcommand)]
        action: MetricsAction,
    },
}
