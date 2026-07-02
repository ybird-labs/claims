//! End-to-end integration test mirroring the demo binary's proofs.

use claims_spike::graphdb::{trust_composition_query, trusted_total_query, SparqlProjection};
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
fn sparql_projection_answers_trust_and_content_queries() {
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
    validator
        .validate(
            &mut store,
            &registry,
            &good.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:00:00Z",
        )
        .unwrap();
    validator
        .validate(
            &mut store,
            &registry,
            &bad.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:05:00Z",
        )
        .unwrap();

    let graphdb = SparqlProjection::load(&store).unwrap();

    // SPARQL trust composition agrees with the programmatic query.
    let projection = l0_projection(&store).unwrap();
    let programmatic = conforming_claims(&projection, CARBON_SCHEMA_IRI, VALIDATOR_IRI);
    let sparql_trusted = graphdb
        .select_iris(
            &trust_composition_query(CARBON_SCHEMA_IRI, VALIDATOR_IRI),
            "claim",
        )
        .unwrap();
    assert_eq!(sparql_trusted, programmatic);
    assert!(sparql_trusted.contains(&good.claim_iri));
    assert!(!sparql_trusted.contains(&bad.claim_iri));

    // An untrusted validator yields an empty SPARQL result.
    let untrusted = graphdb
        .select_iris(
            &trust_composition_query(CARBON_SCHEMA_IRI, "https://validators.example/nobody"),
            "claim",
        )
        .unwrap();
    assert!(untrusted.is_empty());

    // One query composes trust with claim content across layers.
    let tons_property = format!("{CARBON_SCHEMA_IRI}/tons_co2");
    let total = graphdb
        .select_integer(
            &trusted_total_query(CARBON_SCHEMA_IRI, VALIDATOR_IRI, &tons_property),
            "total",
        )
        .unwrap();
    assert_eq!(total, 5);
}

/// Build the full demo scenario: both carbon claims admitted and judged.
fn judged_scenario() -> (L0Store, String, String) {
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
    validator
        .validate(
            &mut store,
            &registry,
            &good.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:00:00Z",
        )
        .unwrap();
    validator
        .validate(
            &mut store,
            &registry,
            &bad.claim_iri,
            CARBON_SCHEMA_IRI,
            "2026-07-01T11:05:00Z",
        )
        .unwrap();

    (store, good.claim_iri, bad.claim_iri)
}

#[test]
fn graph_export_contains_the_full_story() {
    let (store, good_iri, bad_iri) = judged_scenario();

    let export = claims_spike::export::graph_export(&store).unwrap();
    let nodes = export["nodes"].as_array().unwrap();
    let edges = export["edges"].as_array().unwrap();

    let node = |id: &str| nodes.iter().find(|n| n["id"] == id);

    // Both carbon claim nodes, with statuses derived from verdict content.
    assert_eq!(node(&good_iri).unwrap()["kind"], "claim");
    assert_eq!(node(&good_iri).unwrap()["status"], "conforms");
    assert_eq!(node(&bad_iri).unwrap()["kind"], "claim");
    assert_eq!(node(&bad_iri).unwrap()["status"], "violations");

    // Both validation claim nodes.
    let verdicts: Vec<_> = nodes.iter().filter(|n| n["kind"] == "verdict").collect();
    assert_eq!(verdicts.len(), 2);

    // The verifier entities.
    assert_eq!(
        node("https://verifiers.example/alice").unwrap()["kind"],
        "entity"
    );
    assert_eq!(
        node("https://verifiers.example/mallory").unwrap()["kind"],
        "entity"
    );

    // declaresSchema metadata edges for all four claims.
    let declares: Vec<_> = edges
        .iter()
        .filter(|e| e["label"] == "declaresSchema" && e["kind"] == "metadata")
        .collect();
    assert_eq!(declares.len(), 4);

    // Verdict edges linking judgments to their target claims.
    let targets: Vec<_> = edges
        .iter()
        .filter(|e| e["label"] == "target_claim")
        .map(|e| e["target"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&good_iri));
    assert!(targets.contains(&bad_iri));
}

#[test]
fn graph_html_is_self_contained() {
    let (store, _, _) = judged_scenario();

    let html = claims_spike::export::graph_html(&store).unwrap();

    assert!(html.contains("const GRAPH = {"));
    for forbidden in [
        "<script src",
        "<link ",
        "fetch(",
        "import(",
        "XMLHttpRequest",
    ] {
        assert!(
            !html.contains(forbidden),
            "graph.html must be self-contained, found: {forbidden}"
        );
    }
}

#[test]
fn projection_nquads_round_trips() {
    use sophia::api::prelude::*;

    let (store, _, _) = judged_scenario();

    let out_dir = std::env::temp_dir().join(format!("claims-spike-test-{}", std::process::id()));
    let (nquads_path, html_path) = claims_spike::export::write_artifacts(&store, &out_dir).unwrap();

    let written = std::fs::read_to_string(&nquads_path).unwrap();
    let reparsed: sophia::inmem::dataset::LightDataset =
        sophia::turtle::parser::nq::parse_str(&written)
            .collect_quads()
            .unwrap();
    let projection = l0_projection(&store).unwrap();
    assert_eq!(
        reparsed.quads().count(),
        projection.quads().count(),
        "written N-Quads must round-trip to the same quad count"
    );

    assert!(html_path.exists());
    std::fs::remove_dir_all(&out_dir).ok();
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
