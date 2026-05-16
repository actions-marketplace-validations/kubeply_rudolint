//! Autofix edit planning and patch generation.

use rudolint_source::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub span: SourceSpan,
    pub replacement: String,
}
