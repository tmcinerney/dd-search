//! Spans domain command actions.

use clap::Subcommand;

use super::shared::{Pagination, TimeRange};

/// Available actions for the spans domain.
#[derive(Subcommand, Debug)]
pub enum SpansAction {
    /// Search APM spans using Datadog query syntax
    #[command(long_about = "Search APM spans using Datadog's native query syntax.

Query Syntax:
  • Service: service:web, service:api
  • Environment: env:production, env:staging
  • Operation: operation_name:http.request
  • Resource: resource_name:/api/users
  • Duration: @duration:>1s, @duration:[100ms TO 500ms]
  • Tags: custom.tag:value
  • Status: error:true
  • Boolean: service:web AND env:prod
  • Negation: -env:development
  • Wildcards: service:web*, resource_name:/api/*

Output Format:
  Each line contains a JSON object with span attributes and trace metadata.
  Pipe to jq for filtering: ddog spans search \"...\" | jq '.duration'

Examples:
  # Find slow spans
  ddog spans search \"service:api @duration:>1s\"

  # Search by service and environment
  ddog spans search \"service:web env:prod\" --from now-1h

  # Find errors in a specific service
  ddog spans search \"service:api error:true\" --limit 50

  # Complex query with resource filtering
  ddog spans search \"service:web resource_name:/api/users/*\" | jq 'select(.duration > 1000000000)'

Documentation:
  https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/")]
    Search {
        /// Datadog query string (e.g., "service:web env:prod @duration:>1s")
        #[arg(long_help = "Datadog query string using Datadog's APM search syntax.

Supports:
  • Service filtering: service:api, service:web
  • Environment: env:prod, env:staging
  • Duration queries: @duration:>1s, @duration:[100ms TO 500ms]
  • Status: error:true, error:false
  • Tags: custom.tag:value
  • Boolean operators: AND, OR, NOT
  • Negation: -env:development
  • Wildcards: service:*, resource_name:/api/*

Examples:
  \"service:api env:prod\"
  \"@duration:>1s\"
  \"service:web AND error:true\"
  \"resource_name:/api/users/* @duration:[100ms TO 1s]\"")]
        query: String,

        #[command(flatten)]
        time_range: TimeRange,

        #[command(flatten)]
        pagination: Pagination,
    },
}
