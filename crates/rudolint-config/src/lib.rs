use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use rudolint_diagnostics::Severity;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub ignore: BTreeSet<String>,
    #[serde(default)]
    pub severity: BTreeMap<String, Severity>,
    #[serde(default)]
    pub trusted_registries: Vec<String>,
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let Some(path) = path else {
            return Ok(Self::default());
        };
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yaml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn ignores(&self, code: &str) -> bool {
        self.ignore.contains(code)
    }

    pub fn severity_override(&self, code: &str) -> Option<Severity> {
        self.severity.get(code).copied()
    }
}
