//! Demo binary: narrates and asserts the six proofs of the 2026-07-02
//! domain model, using the carbon-verification example. Exits non-zero if
//! any assertion fails.

use claims_spike::graphdb::{trust_composition_query, trusted_total_query, SparqlProjection};
use claims_spike::l0::L0Store;
use claims_spike::l1::{Validator, VALIDATION_RESULT_SCHEMA_IRI};
use claims_spike::projection::{conforming_claims, l0_projection};
use claims_spike::registry::{SchemaRegistry, SchemaVersion};
use claims_spike::snapshot::Snapshot;

const CARBON_SCHEMA_IRI: &str = "https://claims.example/schema/carbon-storage-verification/1.0.0";
const TONS_CO2_PROPERTY_IRI: &str =
    "https://claims.example/schema/carbon-storage-verification/1.0.0/tons_co2";
const VALIDATOR_IRI: &str = "https://validators.example/service/l1";

const MATERIAL_GOOD_A: &str = include_str!("../fixtures/material_good_a.jsonld");
const MATERIAL_GOOD_B: &str = include_str!("../fixtures/material_good_b.jsonld");
const MATERIAL_BAD: &str = include_str!("../fixtures/material_bad.jsonld");

fn proof(number: u8, title: &str) {
    println!("\n=== PROOF {number}: {title} ===");
}

fn check(label: &str, condition: bool) {
    assert!(condition, "FAILED: {label}");
    println!("  ok: {label}");
}

fn main() {
    println!("Claims Engine spike — design/CLAIMS_ENGINE_DOMAIN_MODEL.md (2026-07-02)");
    println!("Scenario: a verifier verified that tons of CO2 were stored at a site.");

    // Setup: register schema versions (user-defined carbon claim type +
    // engine-published validation-result type).
    let mut registry = SchemaRegistry::new();
    registry
        .register(SchemaVersion {
            iri: CARBON_SCHEMA_IRI.to_string(),
            context: serde_json::from_str(include_str!(
                "../fixtures/carbon-storage-verification.context.jsonld"
            ))
            .expect("carbon context fixture"),
            json_schema: serde_json::from_str(include_str!(
                "../fixtures/carbon-storage-verification.schema.json"
            ))
            .expect("carbon schema fixture"),
        })
        .expect("register carbon schema");
    registry
        .register(SchemaVersion {
            iri: VALIDATION_RESULT_SCHEMA_IRI.to_string(),
            context: serde_json::from_str(include_str!(
                "../fixtures/validation-result.context.jsonld"
            ))
            .expect("validation-result context fixture"),
            json_schema: serde_json::from_str(include_str!(
                "../fixtures/validation-result.schema.json"
            ))
            .expect("validation-result schema fixture"),
        })
        .expect("register validation-result schema");

    let mut store = L0Store::new();

    proof(1, "content addressing and idempotent admission");
    let admission_a = store
        .admit(
            MATERIAL_GOOD_A.as_bytes(),
            "https://people.example/bob",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T08:00:00Z",
        )
        .expect("good material A admits");
    let admission_b = store
        .admit(
            MATERIAL_GOOD_B.as_bytes(),
            "https://people.example/carol",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T09:30:00Z",
        )
        .expect("good material B admits");
    println!("  claim: {}", admission_a.claim_iri);
    check(
        "differently-ordered JSON documents yield the same ClaimIRI",
        admission_a.claim_iri == admission_b.claim_iri,
    );
    check(
        "first submission newly admits, second is idempotent",
        admission_a.newly_admitted && !admission_b.newly_admitted,
    );
    let submissions = store.submissions_for(&admission_a.claim_iri);
    check(
        "both submissions are witnessed as audit records",
        submissions.len() == 2,
    );
    check(
        "submitted material fingerprints differ (bytes differ, claim does not)",
        submissions[0].material_fingerprint != submissions[1].material_fingerprint,
    );
    let good_claim_iri = admission_a.claim_iri;

    proof(2, "L0 enforces only the base-schema floor");
    let no_declaration = store.admit(
        MATERIAL_GOOD_A.as_bytes(),
        "https://people.example/bob",
        &[],
        "2026-07-01T10:00:00Z",
    );
    check(
        "missing schema declaration is rejected",
        no_declaration.is_err(),
    );
    let empty_content = store.admit(
        b"{}",
        "https://people.example/bob",
        &[CARBON_SCHEMA_IRI],
        "2026-07-01T10:00:00Z",
    );
    check("empty content is rejected", empty_content.is_err());
    let bad_admission = store
        .admit(
            MATERIAL_BAD.as_bytes(),
            "https://people.example/mallory",
            &[CARBON_SCHEMA_IRI],
            "2026-07-01T10:15:00Z",
        )
        .expect("schema-invalid claim still admits at L0");
    println!(
        "  admitted schema-invalid claim: {}",
        bad_admission.claim_iri
    );
    check(
        "a claim that will fail its declared schema is ADMITTED",
        store.claim(&bad_admission.claim_iri).is_some(),
    );
    let bad_claim_iri = bad_admission.claim_iri;

    proof(
        3,
        "L1 validation is a judgment recorded as validation claims",
    );
    let validator = Validator::new(VALIDATOR_IRI);
    let good_outcome = validator
        .validate(
            &mut store,
            &registry,
            &good_claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:00:00Z",
        )
        .expect("validation of good claim runs");
    check(
        "good claim conforms to its declared schema",
        good_outcome.conforms && good_outcome.violations == 0,
    );
    let bad_outcome = validator
        .validate(
            &mut store,
            &registry,
            &bad_claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:05:00Z",
        )
        .expect("validation of bad claim runs");
    println!(
        "  bad claim violations: {} (missing accreditation_id, tons_co2 < 1)",
        bad_outcome.violations
    );
    check(
        "bad claim fails validation, yet remains admitted in L0",
        !bad_outcome.conforms
            && bad_outcome.violations > 0
            && store.claim(&bad_claim_iri).is_some(),
    );
    let good_verdict = store
        .claim(&good_outcome.validation_claim_iri)
        .expect("validation claim admitted");
    check(
        "outcomes are ordinary claims admitted through the normal path",
        good_verdict
            .declared_schemas()
            .contains(VALIDATION_RESULT_SCHEMA_IRI)
            && store
                .submissions_for(good_verdict.iri())
                .iter()
                .all(|record| record.submitter == VALIDATOR_IRI),
    );
    check(
        "the failing verdict is also a claim in L0",
        store.claim(&bad_outcome.validation_claim_iri).is_some(),
    );

    proof(4, "trust composition over the L0 projection");
    let projection = l0_projection(&store).expect("projection builds");
    let trusted = conforming_claims(&projection, CARBON_SCHEMA_IRI, VALIDATOR_IRI);
    println!("  trusted claims: {trusted:?}");
    check(
        "the conforming claim is returned",
        trusted.contains(&good_claim_iri),
    );
    check(
        "the failing claim is excluded",
        !trusted.contains(&bad_claim_iri),
    );
    check(
        "exactly one claim survives trust composition",
        trusted.len() == 1,
    );

    proof(5, "deterministic snapshot over conforming claims");
    let snapshot_forward = Snapshot::create(trusted.iter().cloned()).expect("snapshot builds");
    let snapshot_reverse =
        Snapshot::create(trusted.iter().rev().cloned()).expect("snapshot builds");
    println!("  snapshot: {}", snapshot_forward.iri());
    check(
        "membership order does not affect the snapshot fingerprint",
        snapshot_forward.fingerprint() == snapshot_reverse.fingerprint()
            && snapshot_forward.iri() == snapshot_reverse.iri(),
    );
    check(
        "snapshot membership is exactly the trusted claim set",
        snapshot_forward.membership().iter().eq(trusted.iter()),
    );

    proof(6, "SPARQL over the projection in an in-memory graph store");
    let graphdb = SparqlProjection::load(&store).expect("projection loads into oxigraph");
    let sparql_trusted = graphdb
        .select_iris(
            &trust_composition_query(CARBON_SCHEMA_IRI, VALIDATOR_IRI),
            "claim",
        )
        .expect("SPARQL trust composition runs");
    check(
        "SPARQL trust composition equals the programmatic query result",
        sparql_trusted == trusted,
    );
    let total_tons = graphdb
        .select_integer(
            &trusted_total_query(CARBON_SCHEMA_IRI, VALIDATOR_IRI, TONS_CO2_PROPERTY_IRI),
            "total",
        )
        .expect("SPARQL aggregate runs");
    println!("  total verified tons of CO2 across trusted claims: {total_tons}");
    check(
        "one SPARQL query composes trust with claim content (5 verified tons)",
        total_tons == 5,
    );

    println!("\nALL PROOFS PASSED");
}
