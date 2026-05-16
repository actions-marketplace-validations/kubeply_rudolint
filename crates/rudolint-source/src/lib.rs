//! Source text, spans, comments, and edit ranges.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
