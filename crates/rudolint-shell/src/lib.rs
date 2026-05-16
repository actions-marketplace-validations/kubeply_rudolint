//! Shell command parsing and analysis for `RUN` instructions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellProgram {
    pub source: String,
}
