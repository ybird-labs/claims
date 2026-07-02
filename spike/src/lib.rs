//! Throwaway spike for the Claims Engine domain model of 2026-07-02
//! (`design/CLAIMS_ENGINE_DOMAIN_MODEL.md`).
//!
//! Not production code: deliberately isolated from the root workspace and
//! exempt from `AGENTS.md` module rules, though it keeps the L0/L1
//! separation loosely, since proving that boundary is part of its job.

pub mod canon;
pub mod export;
pub mod graphdb;
pub mod identity;
pub mod l0;
pub mod l1;
pub mod projection;
pub mod registry;
pub mod snapshot;
