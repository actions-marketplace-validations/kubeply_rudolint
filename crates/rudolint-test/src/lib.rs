//! Shared test-only helpers.

use std::path::{Path, PathBuf};

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rudolint-test should live under crates/rudolint-test")
        .to_path_buf()
}

pub fn fixture_path(path: impl AsRef<Path>) -> PathBuf {
    workspace_root().join("fixtures").join(path)
}
