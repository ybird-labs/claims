use thiserror::Error;

use crate::domain::{ClaimId, ClaimIri};

pub type ApplicationResult<T> = Result<T, ApplicationError>;

pub type ClaimRepositoryResult<T> = Result<T, ClaimRepositoryError>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ApplicationError {
    #[error("claim already exists: {0}")]
    ClaimAlreadyExists(ClaimId),

    #[error("claim IRI already exists: {0}")]
    ClaimIriAlreadyExists(ClaimIri),

    #[error("claim repository failed: {0}")]
    ClaimRepository(#[from] ClaimRepositoryError),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ClaimRepositoryError {
    #[error("claim insert failed duplicate ID: {0}")]
    DuplicateId(ClaimId),

    #[error("claim insert failed duplicate IRI: {0}")]
    DuplicateIri(ClaimIri),

    #[error("claim repository {operation} failed: {message}")]
    BackendFailed {
        operation: ClaimRepositoryOperation,
        message: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimRepositoryOperation {
    InsertClaim,
    GetClaim,
    GetClaimByIri,
}

impl std::fmt::Display for ClaimRepositoryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsertClaim => f.write_str("insert_claim"),
            Self::GetClaim => f.write_str("get_claim"),
            Self::GetClaimByIri => f.write_str("get_claim_by_iri"),
        }
    }
}
