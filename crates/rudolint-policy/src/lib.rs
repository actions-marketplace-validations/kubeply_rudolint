//! Rule selection, profiles, severity overrides, and compatibility policy.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyMode {
    Default,
    Compat,
}
