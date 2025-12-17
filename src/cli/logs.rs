//! Logs domain command actions.

use clap::Subcommand;

use super::shared::{Pagination, TimeRange};

/// Available actions for the logs domain.
#[derive(Subcommand, Debug)]
pub enum LogsAction {
    /// Search logs using Datadog query syntax
    #[command(long_about = "Search logs using Datadog's native query syntax.

Query Syntax:
  • Attributes: @http.status_code:500, @user.id:123
  • Tags: service:api, env:production
  • Wildcards: service:web*, *error*
  • Ranges: @http.status_code:[200 TO 299]
  • Boolean: service:api AND status:error
  • Negation: -env:development, NOT status:ok
  • Grouping: (service:api OR service:web) AND status:error

Output Format:
  Each line contains a JSON object with log attributes and metadata.
  Pipe to jq for filtering: ddog logs search \"...\" | jq '.attributes.message'

Examples:
  # Search for errors
  ddog logs search \"service:api AND status:error\"

  # Search with time range
  ddog logs search \"@http.status_code:500\" --from now-15m --limit 50

  # Search specific indexes
  ddog logs search \"env:production\" --indexes main,web

  # Complex query with filtering
  ddog logs search \"service:api\" | jq 'select(.attributes.duration > 1000)'

Documentation:
  https://docs.datadoghq.com/logs/explorer/search_syntax/")]
    Search {
        /// Datadog query string (e.g., "service:api AND @http.status_code:500")
        #[arg(long_help = "Datadog query string using Datadog's log search syntax.

Supports:
  • Attributes: @http.status_code:500
  • Tags: service:api, env:prod
  • Wildcards: service:web*, *error*
  • Boolean operators: AND, OR, NOT
  • Negation: -env:development
  • Ranges: @http.status_code:[200 TO 299]

Examples:
  \"service:api AND status:error\"
  \"@http.status_code:500\"
  \"(service:api OR service:web) AND env:prod\"
  \"service:* -env:development\"")]
        query: String,

        #[command(flatten)]
        time_range: TimeRange,

        #[command(flatten)]
        pagination: Pagination,

        /// Log indexes to search (comma-separated, default: all)
        #[arg(
            short,
            long,
            value_delimiter = ',',
            default_value = "*",
            long_help = "Log indexes to search. Specify multiple indexes separated by commas.

Examples:
  --indexes main           # Search only the 'main' index
  --indexes main,web       # Search both 'main' and 'web' indexes
  --indexes \"*\"            # Search all indexes (default)"
        )]
        indexes: Vec<String>,
    },
}
