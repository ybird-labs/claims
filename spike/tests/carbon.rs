//! Carbon-project stress-test proofs C1–C6, one test per proof, over the
//! committed fixtures generated from ybird-labs/carbon-project (see
//! `tools/gen_carbon_fixtures.py` and the README's carbon section).
//!
//! The world under test is real registration-review data: two sites —
//! `01.0035.00035` (CZ, 5 plots) and `02.4368.00441` (SK, 2 plots) — each
//! judged against four numbered requirements (SL-003 land tenure, SL-006
//! homogeneity, SL-007 project start date, SL-009 historic activity), with
//! real outcomes spanning satisfied / not_satisfied / unclear. The engine
//! is exercised unmodified; every expectation is computed through the
//! pipeline (admissions, projections, SPARQL), never hardcoded as IRI or
//! fingerprint literals.

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

/// C1 — real entities admit through the unchanged L0 floor.
///
/// Source-system facts about real sites become claims whose subject IRIs
/// are minted from the stable farm keys (entity identity is claim content,
/// design §8), verified by finding the site IRI inside the canonical
/// N-Quads. Resubmitting the same bytes must be idempotent (§7): the same
/// ClaimIRI comes back, nothing is newly admitted, and both submissions are
/// witnessed as separate audit records (§11) — the ingest agent appears
/// there, never in content.
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

/// C2 — a derivation claim walks back to its input ClaimIRIs.
///
/// The real review derives each site's project start date from its earliest
/// active soil-sampling record and records that caveat in prose notes. Here
/// the derivation is structured content: a claim carrying the value, the
/// rule IRI, and `derived_from` input ClaimIRIs (provenance is content,
/// §9). The test walks the L0 projection from each derivation claim's named
/// graph back through `derived_from`, and requires the walked set to equal
/// the ClaimIRIs actually admitted as inputs — every one resolvable in L0.
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

/// C3 — tri-state, multi-evidence judgments are ordinary claims.
///
/// Real review verdicts don't fit the engine's binary validation
/// vocabulary: `unclear` (reviewed, evidence insufficient) is a first-class
/// outcome, and one verdict cites many evidence records. Both are user
/// space. The test checks each judgment claim: admitted through the normal
/// path, declaring the user-space judgment schema, citing ≥2 evidence
/// ClaimIRIs in its content (projected exactly as recorded), and validated
/// as conformant by the *unmodified* L1 against that user-space schema.
/// The real data must span all three outcomes, while the engine-published
/// validation-result vocabulary must still read exactly
/// `["conforms", "violations"]` — proving nothing engine-side was widened.
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

/// C4 — one SPARQL query composes trust through the claim chain.
///
/// Trust composition (§13) generalized to chains: accept the judge's
/// satisfied SL-003 (land tenure) judgments, follow their `evidence` links,
/// and read the cited claims' asserted plot registration ids. The single
/// query crosses three hops and both projection layers — judgment content
/// in named graphs, the witnessed `declaresSchema` join in the default
/// graph, evidence content in named graphs again. Its result must equal
/// the programmatically computed (evidence ClaimIRI, registration id)
/// pairs, be non-empty, and reach plot evidence from both real sites.
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

/// C5 — the normative rule version is content, so a bump is a new claim.
///
/// The credit-class version governs what a verdict *means* without changing
/// its payload shape; per the identity rule (§8) it lives in asserted
/// content, not in the engine's schema-version machinery. The test
/// re-issues site 1's SL-003 judgment byte-identical except for
/// `credit_class_version` ("1.5.2" → hypothetical "1.6.0") and requires:
/// a different ClaimIRI (content addressing sees the version), both claims
/// coexisting in L0 (no mutation, no supersession), and SPARQL filtering
/// judgments by version — each version query returning exactly its own
/// claim set.
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

/// C6 — an order-independent snapshot over the carbon claims.
///
/// A registration submission is a bounded evidence set: everything the
/// review rests on, frozen. Snapshot identity is canonical membership alone
/// (§14), so snapshotting all carbon claims (evidence, derivations,
/// judgments, L1 verdicts) in forward and reverse insertion order must
/// yield the same fingerprint and the same SnapshotIRI.
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

/// Guard for the anti-cheat posture of every proof above: nothing in the
/// pipeline depends on clocks, randomness, or iteration order. Two worlds
/// built independently from the same committed fixtures must admit exactly
/// the same claim set — and therefore the same snapshot IRI — making every
/// C-proof's computed expectation stable run-to-run without a single
/// hardcoded IRI or fingerprint literal.
#[test]
fn world_building_is_deterministic_run_to_run() {
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
