//! Pure Claims Engine domain model.
//!
//! Domain code owns business language and invariants only.
//!
//! Do not import infrastructure, API, storage, projection, HTTP, DB, RDF parser
//! implementation, triplestore implementation, external SDK, DTO, or async IO
//! concerns here.
//!
//! Repository, storage, event-publishing, canonicalization, projection, and
//! external-adapter ports belong in `crate::application` by default.

mod claim;
mod content;
mod error;
mod fingerprint;
mod id;
mod iri;
mod provenance;
mod time;

pub use claim::{Claim, ClaimValue};
pub use content::{
    AssertedContent, CanonicalNQuads, CanonicalRdfContentEncoding, CanonicalRdfDataset,
};
pub use error::DomainError;
pub use fingerprint::{
    ClaimFingerprint, ClaimFingerprintSuite, Sha256Digest, SnapshotFingerprint,
    SubmittedMaterialFingerprint,
};
pub use id::{ClaimId, SnapshotId, SubmissionId};
pub use iri::{AbsoluteIri, AssertorIri, ClaimIri, ClaimTypeIri, SnapshotIri};
pub use provenance::AssertionProvenance;
pub use time::{AssertedAt, DateTimeUtc};
