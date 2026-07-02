//! L0: the raw claim record.
//!
//! Admits, content-addresses, and stores immutable claims. Enforces only the
//! base claim schema floor (design doc §6.1, §12): content parses as
//! well-formed self-contained JSON-LD, canonical content is non-empty, and at
//! least one well-formed schema version IRI is declared. Conformance to
//! declared schemas is NOT checked here — that is L1's judgment.

use std::collections::{BTreeMap, BTreeSet};

use crate::canon;
use crate::identity;

/// Accepted immutable claim: the claim value (declared schema references +
/// canonical content) plus its derived fingerprint and ClaimIRI (design doc §3).
///
/// Fields are private with no mutators: immutability after durable admission
/// (§18 invariant 1) holds by construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    iri: String,
    declared_schemas: BTreeSet<String>,
    canonical_nquads: String,
    fingerprint: String,
}

impl Claim {
    pub fn iri(&self) -> &str {
        &self.iri
    }

    pub fn declared_schemas(&self) -> &BTreeSet<String> {
        &self.declared_schemas
    }

    pub fn canonical_nquads(&self) -> &str {
        &self.canonical_nquads
    }

    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

/// Engine-witnessed ingestion audit record (design doc §11). Lives outside
/// the claim value; never part of claim identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubmissionRecord {
    pub submission_id: String,
    pub submitter: String,
    pub accepted_at: String,
    pub material: Vec<u8>,
    pub material_fingerprint: String,
    pub claim_iri: String,
}

/// Outcome of durable admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Admission {
    pub claim_iri: String,
    /// `false` when the claim value already existed (idempotent resubmission,
    /// design doc §7): same claim, new submission record.
    pub newly_admitted: bool,
    pub submission_id: String,
}

/// Base-schema floor violations (design doc §6.1). Note the absence of any
/// "does not conform to its declared schema" variant: that is not L0's job.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdmissionError {
    #[error("claim must declare at least one schema version")]
    NoSchemaDeclaration,
    #[error("declared schema reference is not a well-formed absolute IRI: {0}")]
    InvalidSchemaDeclaration(String),
    #[error("submitted material is not valid UTF-8")]
    MaterialNotUtf8,
    #[error(transparent)]
    Malformed(#[from] canon::CanonError),
    #[error("claim content must not be empty")]
    EmptyClaimContent,
}

/// In-memory authoritative claim record plus witnessed submission log.
#[derive(Debug, Default)]
pub struct L0Store {
    claims: BTreeMap<String, Claim>,
    submissions: Vec<SubmissionRecord>,
}

impl L0Store {
    pub fn new() -> Self {
        Self::default()
    }

    /// Durable admission (design doc §12): enforce the base-schema floor,
    /// canonicalize, content-address, store. `accepted_at` is the engine's
    /// witnessed clock reading, supplied by the caller in this spike.
    pub fn admit(
        &mut self,
        material: &[u8],
        submitter: &str,
        declared_schemas: &[&str],
        accepted_at: &str,
    ) -> Result<Admission, AdmissionError> {
        if declared_schemas.is_empty() {
            return Err(AdmissionError::NoSchemaDeclaration);
        }

        let mut declared = BTreeSet::new();
        for schema_iri in declared_schemas {
            if sophia::iri::Iri::new(*schema_iri).is_err() {
                return Err(AdmissionError::InvalidSchemaDeclaration(
                    (*schema_iri).to_string(),
                ));
            }
            declared.insert((*schema_iri).to_string());
        }

        let text = std::str::from_utf8(material).map_err(|_| AdmissionError::MaterialNotUtf8)?;
        let canonical_nquads = canon::jsonld_to_canonical_nquads(text)?;

        if canonical_nquads.is_empty() {
            return Err(AdmissionError::EmptyClaimContent);
        }

        let fingerprint = identity::claim_fingerprint(&declared, &canonical_nquads);
        let claim_iri = identity::claim_iri(&fingerprint);

        let newly_admitted = !self.claims.contains_key(&claim_iri);
        if newly_admitted {
            self.claims.insert(
                claim_iri.clone(),
                Claim {
                    iri: claim_iri.clone(),
                    declared_schemas: declared,
                    canonical_nquads,
                    fingerprint,
                },
            );
        }

        let submission_id = format!("sub-{:04}", self.submissions.len() + 1);
        self.submissions.push(SubmissionRecord {
            submission_id: submission_id.clone(),
            submitter: submitter.to_string(),
            accepted_at: accepted_at.to_string(),
            material: material.to_vec(),
            material_fingerprint: identity::submitted_material_fingerprint(material),
            claim_iri: claim_iri.clone(),
        });

        Ok(Admission {
            claim_iri,
            newly_admitted,
            submission_id,
        })
    }

    pub fn claim(&self, claim_iri: &str) -> Option<&Claim> {
        self.claims.get(claim_iri)
    }

    pub fn claims(&self) -> impl Iterator<Item = &Claim> {
        self.claims.values()
    }

    pub fn submissions_for(&self, claim_iri: &str) -> Vec<&SubmissionRecord> {
        self.submissions
            .iter()
            .filter(|record| record.claim_iri == claim_iri)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA: &str = "https://claims.example/schema/test/1.0.0";

    fn material(value: &str) -> String {
        format!(
            r#"{{"@context":{{"p":"https://x.example/p"}},"@id":"https://x.example/s","p":"{value}"}}"#
        )
    }

    #[test]
    fn rejects_missing_schema_declaration() {
        let mut store = L0Store::new();

        let err = store
            .admit(material("v").as_bytes(), "bob", &[], "2026-07-01T00:00:00Z")
            .unwrap_err();

        assert_eq!(err, AdmissionError::NoSchemaDeclaration);
    }

    #[test]
    fn rejects_relative_schema_declaration() {
        let mut store = L0Store::new();

        let err = store
            .admit(
                material("v").as_bytes(),
                "bob",
                &["not-an-absolute-iri"],
                "2026-07-01T00:00:00Z",
            )
            .unwrap_err();

        assert_eq!(
            err,
            AdmissionError::InvalidSchemaDeclaration("not-an-absolute-iri".to_string())
        );
    }

    #[test]
    fn rejects_empty_content() {
        let mut store = L0Store::new();

        let err = store
            .admit(b"{}", "bob", &[SCHEMA], "2026-07-01T00:00:00Z")
            .unwrap_err();

        assert_eq!(err, AdmissionError::EmptyClaimContent);
    }

    #[test]
    fn admission_is_idempotent_and_records_every_submission() {
        let mut store = L0Store::new();
        let bytes = material("v");

        let first = store
            .admit(bytes.as_bytes(), "bob", &[SCHEMA], "2026-07-01T00:00:00Z")
            .unwrap();
        let second = store
            .admit(bytes.as_bytes(), "carol", &[SCHEMA], "2026-07-01T01:00:00Z")
            .unwrap();

        assert_eq!(first.claim_iri, second.claim_iri);
        assert!(first.newly_admitted);
        assert!(!second.newly_admitted);
        assert_eq!(store.submissions_for(&first.claim_iri).len(), 2);
        assert_eq!(store.claims().count(), 1);
    }

    #[test]
    fn different_schema_declaration_is_a_different_claim() {
        let mut store = L0Store::new();
        let bytes = material("v");

        let first = store
            .admit(bytes.as_bytes(), "bob", &[SCHEMA], "2026-07-01T00:00:00Z")
            .unwrap();
        let second = store
            .admit(
                bytes.as_bytes(),
                "bob",
                &["https://claims.example/schema/other/1.0.0"],
                "2026-07-01T00:00:00Z",
            )
            .unwrap();

        assert_ne!(first.claim_iri, second.claim_iri);
    }
}
