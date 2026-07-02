//! Artifact checks for the carbon demo: the N-Quads projection round-trips,
//! the interactive graph carries outcome-derived styling computed from claim
//! content, no artifact references a remote resource, and the workbench's
//! expected C4 result equals the programmatic expectation.
//!
//! The live-execution check (the workbench actually evaluating SPARQL in a
//! browser) is `tools/verify_sparql_workbench.cjs` — it cannot run under
//! `cargo test` because it needs a real Chromium.

use std::path::PathBuf;

use claims_spike::carbon::{
    build_world, expected_registration_pairs, judgment_evidence_query, workbench_samples,
    write_carbon_artifacts, CarbonWorld, SL_003,
};
use claims_spike::projection::l0_projection;
use sophia::api::prelude::*;

fn artifacts(world: &CarbonWorld) -> [PathBuf; 4] {
    let out_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("carbon-artifacts");
    write_carbon_artifacts(world, &out_dir).expect("artifacts write")
}

/// The graph data embedded in graph.html, parsed back out of the document.
fn embedded_graph_json(html: &str) -> serde_json::Value {
    let start = html.find("const GRAPH = ").expect("data marker") + "const GRAPH = ".len();
    let end = start + html[start..].find(";\n").expect("data terminator");
    serde_json::from_str(&html[start..end]).expect("embedded graph data parses")
}

#[test]
fn projection_nquads_round_trip_into_oxigraph_with_equal_quad_count() {
    let world = build_world();
    let [nquads_path, ..] = artifacts(&world);

    let written = std::fs::read(&nquads_path).expect("read projection.nq");
    let store = oxigraph::store::Store::new().expect("store");
    store
        .load_from_reader(oxigraph::io::RdfFormat::NQuads, written.as_slice())
        .expect("written N-Quads load into oxigraph");

    let projection = l0_projection(&world.store).expect("projection");
    assert_eq!(
        store.len().expect("store size"),
        projection.quads().count(),
        "every projected quad survives the file round-trip"
    );
}

#[test]
fn graph_html_tags_every_judgment_with_its_content_derived_outcome() {
    let world = build_world();
    let [_, graph_path, ..] = artifacts(&world);
    let html = std::fs::read_to_string(&graph_path).expect("read graph.html");
    let graph = embedded_graph_json(&html);
    let nodes = graph["nodes"].as_array().expect("nodes");

    let mut outcomes_seen = std::collections::BTreeSet::new();
    for site in &world.sites {
        for judgment in &site.judgments {
            let node = nodes
                .iter()
                .find(|n| n["id"] == judgment.claim_iri.as_str())
                .unwrap_or_else(|| panic!("node for {}", judgment.claim_iri));
            assert_eq!(
                node["outcome"],
                judgment.outcome.as_str(),
                "outcome read from the judgment's own claim content"
            );
            assert_eq!(node["schema"], "requirement-judgment");
            outcomes_seen.insert(judgment.outcome.clone());
        }
    }
    assert_eq!(outcomes_seen.len(), 3, "all three outcomes in the data");

    // Each outcome present in the data resolves to an outcome-derived style
    // hook in the document (CSS custom property `--outcome-<value>`).
    for outcome in &outcomes_seen {
        assert!(
            html.contains(&format!("--outcome-{outcome}")),
            "style hook for outcome {outcome}"
        );
    }
}

#[test]
fn no_artifact_references_a_remote_resource() {
    let world = build_world();
    let [_, graph_path, sparql_path, _] = artifacts(&world);

    // Load-bearing positions only: element URLs, CSS imports/urls, and
    // remote fetch/import call targets. IRIs appearing as data are fine.
    let forbidden = [
        "src=\"http",
        "src='http",
        "href=\"http",
        "href='http",
        "url(http",
        "@import",
        "fetch(\"http",
        "fetch('http",
        "import(\"http",
        "import('http",
        "XMLHttpRequest",
    ];
    for path in [&graph_path, &sparql_path] {
        let html = std::fs::read_to_string(path).expect("read artifact");
        for pattern in forbidden {
            assert!(
                !html.contains(pattern),
                "{} must be self-contained, found: {pattern}",
                path.display()
            );
        }
    }
}

#[test]
fn expected_c4_json_equals_the_programmatic_expectation() {
    let world = build_world();
    let [.., expected_path] = artifacts(&world);

    let written: Vec<[String; 2]> =
        serde_json::from_str(&std::fs::read_to_string(&expected_path).expect("read json"))
            .expect("expected_c4.json parses");
    let expected: Vec<[String; 2]> = expected_registration_pairs(&world, SL_003, "satisfied")
        .into_iter()
        .map(|(evidence, registration)| [evidence, registration])
        .collect();

    assert_eq!(written, expected);
    assert!(!expected.is_empty());
}

#[test]
fn workbench_embeds_the_engine_the_data_and_the_c4_query() {
    let world = build_world();
    let [_, _, sparql_path, _] = artifacts(&world);
    let html = std::fs::read_to_string(&sparql_path).expect("read sparql.html");

    // The preloaded first sample is the C4 chain query, JSON-escaped.
    let c4_query = judgment_evidence_query(
        SL_003,
        "satisfied",
        &world.sites[0].credit_class_version,
    );
    let c4_escaped = serde_json::to_string(&c4_query).expect("encodes");
    assert!(
        html.contains(c4_escaped.trim_matches('"')),
        "C4 chain query is embedded as a sample"
    );
    assert_eq!(
        workbench_samples(&world)[0].1,
        c4_query,
        "the C4 query is the preloaded default sample"
    );

    // Engine and data are embedded, not referenced.
    assert!(html.contains("initSync"), "vendored oxigraph web.js inlined");
    assert!(html.contains("WASM_BASE64"), "wasm embedded as base64");
    // A projected claim IRI appears inside the embedded N-Quads data.
    let a_claim_iri = &world.sites[0].derivation_iri;
    assert!(
        html.contains(a_claim_iri.as_str()),
        "projection data embedded in the page"
    );
}
