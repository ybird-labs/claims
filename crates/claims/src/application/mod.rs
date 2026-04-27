//! Application layer.
//!
//! Use cases, commands, queries, and side-effecting ports belong here.
//! Infrastructure implements the ports; domain stays pure.
//!
mod claims;
mod error;
mod ports;

pub use claims::ClaimService;
pub use error::{ApplicationError, ApplicationResult};
pub use ports::ClaimRepository;
