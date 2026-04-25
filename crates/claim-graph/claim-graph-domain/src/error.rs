use crate::ClaimGraphId;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ClaimGraphError {
    #[error("validation failed: {reason}")]
    Validation { reason: String },

    #[error("business rule violated: {rule}")]
    BusinessRule { rule: String },

    #[error("not found: {0}")]
    NotFound(ClaimGraphId),

    #[error("infrastructure failure: {0}")]
    Infrastructure(String),
}

impl ClaimGraphError {
    pub fn infrastructure<E: std::error::Error>(err: E) -> Self {
        Self::Infrastructure(err.to_string())
    }
}
