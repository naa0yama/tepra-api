//! tepra-prefixed semantic conventions for app-specific telemetry.
//!
//! Mirrors the layout of `opentelemetry_semantic_conventions::{metric,
//! attribute}` to provide a single source of truth for `tepra.*` names
//! across all signals (metrics today, tracing/logs in the future).
//! Use these constants instead of string literals to avoid typos and drift.

/// Metric name constants for `tepra.*` instruments.
pub mod metric {
    /// End-to-end command execution latency.
    pub const RUN_DURATION: &str = "tepra.run.duration";
    /// Total greeting calls attributed by resolved gender.
    pub const GREETING_COUNT: &str = "tepra.greeting.count";
    /// Greeting calls that resulted in an error.
    pub const GREETING_ERRORS: &str = "tepra.greeting.errors";
    /// Total iterations executed in the count demo.
    pub const ITERATION_COUNT: &str = "tepra.iteration.count";
    /// Per-iteration sleep delay in the count demo.
    pub const ITERATION_DURATION: &str = "tepra.iteration.duration";
    /// Iterations currently executing (`UpDownCounter` demo).
    pub const ITERATION_IN_FLIGHT: &str = "tepra.iteration.in_flight";
}

/// Attribute key constants for `tepra.*` dimensions.
pub mod attribute {
    /// CLI command name attribute key.
    pub const COMMAND: &str = "tepra.command";
    /// Resolved gender attribute key.
    pub const GENDER: &str = "tepra.gender";
}
