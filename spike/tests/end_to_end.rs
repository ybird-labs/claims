//! End-to-end integration test mirroring the demo binary's five proofs.

use claims_spike::l0::L0Store;
use claims_spike::l1::{Validator, VALIDATION_RESULT_SCHEMA_IRI};
use claims_spike::projection::{conforming_claims, l0_projection};
use claims_spike::registry::{SchemaRegistry, SchemaVersion};
use claims_spike::snapshot::Snapshot;

const CARBON_SCHEMA_IRI: &str = "https://claims.example/schema/carbon-storage-verification/1.0.0";
const VALIDATOR_IRI: &str = "https://validators.example/service/l1";

const MATERIAL_GOOD_A: &str = include_str!("../fixtures/material_good_a.jsonld");
const MATERIAL_GOOD_B: &str = include_str!("../fixtures/material_good_b.jsonld");
const MATERIAL_BAD: &str = include_str!("../fixtures/material_bad.jsonld");

fn registry() -> SchemaRegistry {
    let mut registry = SchemaRegistry::new();
    registry
        .register(SchemaVersion {
            iri: CARBON_SCHEMA_IRI.to_string(),
            context: serde_json::from_str(include_str!(
                "../fixtures/carbon-storage-verification.context.jsonld"
            ))
            .unwrap(),
            json_schema: serde_json::from_str(include_str!(
                "../fixtures/carbon-storage-verification.schema.json"
            ))
            .unwrap(),
        })
        .unwrap();
    registry
        .register(SchemaVersion {
            iri: VALIDATION_RESULT_SCHEMA_IRI.to_string(),
            context: serde_json::from_str(include_str!(
                "../fixtures/validation-result.context.jsonld"
            ))
            .unwrap(),
            json_schema: serde_json::from_str(include_str!(
                "../fixtures/validation-result.schema.json"
            ))
            .unwrap(),
        })
        .unwrap();
    registry
}

#[test]
fn differently_ordered_documents_are_the_same_claim() {
    let mut store = L0Store::new();

    let a = store
        .admit(
            MATERIAL_GOOD_A.as_bytes(),
            "https://people.example/bob",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T08:00:00Z",
        )
        .unwrap();
    let b = store
        .admit(
            MATERIAL_GOOD_B.as_bytes(),
            "https://people.example/carol",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T09:30:00Z",
        )
        .unwrap();

    assert_eq!(a.claim_iri, b.claim_iri);
    assert!(a.newly_admitted);
    assert!(!b.newly_admitted);

    let submissions = store.submissions_for(&a.claim_iri);
    assert_eq!(submissions.len(), 2);
    assert_ne!(
        submissions[0].material_fingerprint,
        submissions[1].material_fingerprint
    );
}

#[test]
fn l0_admits_schema_invalid_claims_but_enforces_the_floor() {
    let mut store = L0Store::new();

    assert!(store
        .admit(
            MATERIAL_GOOD_A.as_bytes(),
            "https://people.example/bob",
            &[],
            "2026-07-01T10:00:00Z",
        )
        .is_err());
    assert!(store
        .admit(
            b"{}",
            "https://people.example/bob",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T10:00:00Z",
        )
        .is_err());

    let bad = store
        .admit(
            MATERIAL_BAD.as_bytes(),
            "https://people.example/mallory",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T10:15:00Z",
        )
        .unwrap();
    assert!(store.claim(&bad.claim_iri).is_some());
}

#[test]
fn validation_judgments_become_claims_and_compose_into_trust() {
    let registry = registry();
    let mut store = L0Store::new();
    let validator = Validator::new(VALIDATOR_IRI);

    let good = store
        .admit(
            MATERIAL_GOOD_A.as_bytes(),
            "https://people.example/bob",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T08:00:00Z",
        )
        .unwrap();
    let bad = store
        .admit(
            MATERIAL_BAD.as_bytes(),
            "https://people.example/mallory",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T10:15:00Z",
        )
        .unwrap();

    let good_outcome = validator
        .validate(
            &mut store,
            &registry,
            &good.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:00:00Z",
        )
        .unwrap();
    let bad_outcome = validator
        .validate(
            &mut store,
            &registry,
            &bad.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:05:00Z",
        )
        .unwrap();

    assert!(good_outcome.conforms);
    assert!(!bad_outcome.conforms);
    assert!(bad_outcome.violations > 0);

    // Judgments are ordinary claims in L0.
    let verdict = store.claim(&good_outcome.validation_claim_iri).unwrap();
    assert!(verdict
        .declared_schemas()
        .contains(VALIDATION_RESULT_SCHEMA_IRI));
    // The failing claim remains admitted.
    assert!(store.claim(&bad.claim_iri).is_some());

    // Trust composition returns exactly the conforming claim.
    let projection = l0_projection(&store).unwrap();
    let trusted = conforming_claims(&projection, CARBON_SCHEMA_IRI, VALIDATOR_IRI);
    assert!(trusted.contains(&good.claim_iri));
    assert!(!trusted.contains(&bad.claim_iri));
    assert_eq!(trusted.len(), 1);

    // A validator nobody trusts yields nothing.
    let untrusted = conforming_claims(
        &projection,
        CARBON_SCHEMA_IRI,
        "https://validators.example/nobody",
    );
    assert!(untrusted.is_empty());

    // Snapshot over the trusted set is deterministic.
    let forward = Snapshot::create(trusted.iter().cloned()).unwrap();
    let reverse = Snapshot::create(trusted.iter().rev().cloned()).unwrap();
    assert_eq!(forward, reverse);
}

#[test]
fn validating_an_unknown_claim_fails() {
    let registry = registry();
    let mut store = L0Store::new();
    let validator = Validator::new(VALIDATOR_IRI);

    let err = validator.validate(
        &mut store,
        &registry,
        "https://claims.example/claim/sha256/does-not-exist",
        CARBON_SCHEMA_IRI,
        "2026-07-01T11:00:00Z",
    );

    assert!(err.is_err());
}
