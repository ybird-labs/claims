//! Infrastructure adapters.
//!
//! Concrete persistence, RDF/canonicalization, projection writers, external
//! SDKs, and other side-effecting implementations belong here.

mod in_memory;

pub use in_memory::InMemoryClaimRepository;
