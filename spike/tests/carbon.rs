//! Carbon-project stress-test proofs C1–C6, one test per proof, over the
//! committed fixtures generated from ybird-labs/carbon-project.

use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

use claims_spike::carbon::{
    build_world, expected_registration_pairs, judgment_evidence_query,
    judgments_by_version_query, objects_in_graph, projection, reissue_judgment_with_version,
    CarbonWorld, DERIVATION_SCHEMA_IRI, JUDGMENT_SCHEMA_IRI,
};
use claims_spike::graphdb::SparqlProjection;
use claims_spike::l1::VALIDATION_RESULT_SCHEMA_IRI;
use claims_spike::snapshot::Snapshot;

const SL_003: &str = "C06-REGISTRATION-SL-003";

/// The world is deterministic and read-only for most proofs; build it once.
fn world() -> &'static Mutex<CarbonWorld> {
    static WORLD: OnceLock<Mutex<CarbonWorld>> = OnceLock::new();
    WORLD.get_or_init(|| Mutex::new(build_world()))
}

#[test]
fn c1_real_entities_admit_through_the_unchanged_floor() {
    let world = world().lock().unwrap();

    assert_eq!(world.sites.len(), 2, "two real sites");
    for site in &world.sites {
        let claim = world.store.claim(&site.site_record_iri).expect("site record");
        assert!(
            claim.canonical_nquads().contains(&format!("<{}>", site.site_iri)),
            "{}: subject IRI in canonical content",
            site.farm_key
        );
        assert!(
            site.site_iri.ends_with(&site.farm_key),
            "{}: entity IRI minted from the real farm key",
            site.farm_key
        );
        assert!(
            site.site_record_resubmission_idempotent,
            "{}: same bytes resubmitted -> same claim, not newly admitted",
            site.farm_key
        );
        assert_eq!(
            site.site_record_submission_count, 2,
            "{}: both submissions witnessed",
            site.farm_key
        );
    }
}

#[test]
fn c2_derivation_walks_back_to_its_inputs() {
    let world = world().lock().unwrap();
    let proj = projection(&world);

    for site in &world.sites {
        let walked = objects_in_graph(
            &proj,
            &site.derivation_iri,
            &format!("{DERIVATION_SCHEMA_IRI}/derived_from"),
        );
        assert_eq!(
            walked, site.derivation_inputs,
            "{}: derived_from cites exactly the admitted inputs",
            site.farm_key
        );
        assert!(walked.len() >= 2, "{}: at least two inputs", site.farm_key);
        for input in &walked {
            assert!(
                world.store.claim(input).is_some(),
                "{}: input {input} resolvable in L0",
                site.farm_key
            );
        }
    }
}

#[test]
fn c3_tri_state_multi_evidence_judgments_are_ordinary_claims() {
    let world = world().lock().unwrap();
    let proj = projection(&world);

    let mut outcomes = BTreeSet::new();
    for site in &world.sites {
        assert_eq!(site.judgments.len(), 4, "{}: four requirements", site.farm_key);
        for judgment in &site.judgments {
            let claim = world
                .store
                .claim(&judgment.claim_iri)
                .expect("judgment admitted as an ordinary claim");
            assert!(claim.declared_schemas().contains(JUDGMENT_SCHEMA_IRI));
            let cited = objects_in_graph(
                &proj,
                &judgment.claim_iri,
                &format!("{JUDGMENT_SCHEMA_IRI}/evidence"),
            );
            assert_eq!(cited, judgment.evidence_iris);
            assert!(cited.len() >= 2, "multi-evidence: ≥2 cited ClaimIRIs");
            assert!(
                judgment.l1_conforms,
                "unmodified L1 validates the judgment against its user-space schema"
            );
            assert!(world.store.claim(&judgment.validation_claim_iri).is_some());
            outcomes.insert(judgment.outcome.clone());
        }
    }

    let tri_state: BTreeSet<String> = ["satisfied", "not_satisfied", "unclear"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(outcomes, tri_state, "real data spans all three outcomes");

    // The engine's own validation vocabulary stays binary; tri-state is
    // user-space only.
    let engine_schema = &world
        .registry
        .get(VALIDATION_RESULT_SCHEMA_IRI)
        .expect("engine schema registered")
        .json_schema;
    assert_eq!(
        engine_schema["properties"]["outcome"]["enum"],
        serde_json::json!(["conforms", "violations"])
    );
}

#[test]
fn c4_sparql_composes_trust_through_the_claim_chain() {
    let world = world().lock().unwrap();
    let graphdb = SparqlProjection::load(&world.store).expect("projection loads");

    let expected = expected_registration_pairs(&world, SL_003, "satisfied");
    assert!(!expected.is_empty(), "expectation computed from the world");

    let via_sparql = graphdb
        .select_pairs(
            &judgment_evidence_query(SL_003, "satisfied", &world.sites[0].credit_class_version),
            "evidence",
            "registration",
        )
        .expect("chain query runs");

    assert_eq!(via_sparql, expected, "judgment -> evidence -> content chain");
    for site in &world.sites {
        assert!(
            site.plot_registrations
                .keys()
                .any(|iri| via_sparql.iter().any(|(evidence, _)| evidence == iri)),
            "{}: chain reaches this site's plot evidence",
            site.farm_key
        );
    }
}

#[test]
fn c5_rule_version_is_content_so_a_bump_is_a_new_claim() {
    let mut world = world().lock().unwrap();

    let original_iri = world.sites[0]
        .judgments
        .iter()
        .find(|j| j.requirement_id == SL_003)
        .expect("SL-003 judged")
        .claim_iri
        .clone();
    let version = world.sites[0].credit_class_version.clone();

    let reissued_iri = reissue_judgment_with_version(&mut world, 0, SL_003, "1.6.0");

    assert_ne!(reissued_iri, original_iri, "bumped version = new identity");
    assert!(world.store.claim(&original_iri).is_some(), "original coexists");
    assert!(world.store.claim(&reissued_iri).is_some(), "reissue coexists");

    let graphdb = SparqlProjection::load(&world.store).expect("projection reloads");
    let under_original: BTreeSet<String> = world
        .sites
        .iter()
        .flat_map(|site| &site.judgments)
        .filter(|j| j.requirement_id == SL_003)
        .map(|j| j.claim_iri.clone())
        .collect();
    let sparql_original = graphdb
        .select_iris(&judgments_by_version_query(SL_003, &version), "judgment_claim")
        .expect("filter by original version");
    let sparql_bumped = graphdb
        .select_iris(&judgments_by_version_query(SL_003, "1.6.0"), "judgment_claim")
        .expect("filter by bumped version");

    assert_eq!(sparql_original, under_original);
    assert_eq!(sparql_bumped, [reissued_iri].into_iter().collect());
}

#[test]
fn c6_snapshot_over_the_carbon_claims_is_order_independent() {
    let world = world().lock().unwrap();

    let membership: Vec<String> = world
        .store
        .claims()
        .map(|claim| claim.iri().to_string())
        .collect();
    assert!(membership.len() >= 20, "evidence + derivations + judgments + verdicts");

    let forward = Snapshot::create(membership.iter().cloned()).expect("snapshot");
    let reverse = Snapshot::create(membership.iter().rev().cloned()).expect("snapshot");

    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(forward.iri(), reverse.iri());
}

#[test]
fn world_building_is_deterministic_run_to_run() {
    // Two independently built worlds admit identical claim sets: all IRIs
    // are pipeline-computed (no clocks, no randomness), so snapshots agree.
    let world_a = build_world();
    let world_b = build_world();

    let iris_a: BTreeSet<String> = world_a.store.claims().map(|c| c.iri().to_string()).collect();
    let iris_b: BTreeSet<String> = world_b.store.claims().map(|c| c.iri().to_string()).collect();

    assert_eq!(iris_a, iris_b);
    assert_eq!(
        Snapshot::create(iris_a).unwrap().iri(),
        Snapshot::create(iris_b).unwrap().iri()
    );
}
