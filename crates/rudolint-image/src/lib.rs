//! Container image references, registries, tags, and digests.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageReference {
    pub raw: String,
}
