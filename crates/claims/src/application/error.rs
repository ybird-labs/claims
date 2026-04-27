use thiserror::Error;

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ApplicationError {
    #[error("claim repository operation failed: {0}")]
    ClaimRepositoryFailed(String),
}
