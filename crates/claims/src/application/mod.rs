//! Application layer.
//!
//! Use cases, commands, queries, and side-effecting ports belong here.
//! Infrastructure implements the ports; domain stays pure.
//!
mod claim_service;
mod error;
mod ports;

pub use claim_service::ClaimService;
pub use error::{
    ApplicationError, ApplicationResult, ClaimRepositoryError, ClaimRepositoryOperation,
    ClaimRepositoryResult,
};
pub use ports::ClaimRepository;
