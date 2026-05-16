//! BuildKit frontend, mount, entitlement, and Buildx semantic analysis.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontend {
    pub image: String,
}
