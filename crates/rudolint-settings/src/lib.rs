//! Resolved settings after config discovery and CLI overrides.

use rudolint_config::Config;

#[derive(Debug, Clone, Default)]
pub struct Settings {
    pub config: Config,
}
