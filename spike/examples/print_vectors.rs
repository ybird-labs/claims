// SPDX-License-Identifier: MPL-2.0

//! Prints test-vector values for design/CLAIMS_PROTOCOL.md.
//! Run: cd spike && cargo run --example print_vectors

use std::collections::BTreeSet;

use claims_spike::{canon, identity};

const SCHEMA: &str = "https://claims.example/schema/carbon-storage-verification/1.0.0";

/// Protocol-fixed, origin-free reference form (decision 2026-09-12):
/// claims:claim:rdfc10-sha256:<64 lowercase hex>. Supersedes the spike's
/// per-instance `https://claims.example/...` namespace for spec vectors.
/// Preimage layouts are unchanged: the minted object's own IRI never
/// enters its preimage, but declared schema IRIs and authored content
/// (including any cited ClaimIRIs) remain hashed — vectors whose inputs
/// cite engine-minted IRIs (snapshots, the §8 example) change form.
fn claim_iri(fingerprint_hex: &str) -> String {
    format!("claims:claim:rdfc10-sha256:{fingerprint_hex}")
}

/// claims:snapshot:rdfc10-sha256:<64 lowercase hex> (decision 2026-09-12).
fn snapshot_iri(fingerprint_hex: &str) -> String {
    format!("claims:snapshot:rdfc10-sha256:{fingerprint_hex}")
}

fn schemas(iris: &[&str]) -> BTreeSet<String> {
    iris.iter().map(|s| s.to_string()).collect()
}

fn vector(label: &str, material_path: &str, declared: &[&str]) {
    let material = std::fs::read(material_path).unwrap();
    let text = std::str::from_utf8(&material).unwrap();
    let nquads = canon::jsonld_to_canonical_nquads(text).unwrap();
    let declared = schemas(declared);
    let fp = identity::claim_fingerprint(&declared, &nquads);
    let iri = claim_iri(&fp);
    let mat_fp = identity::submitted_material_fingerprint(&material);
    println!("=== {label} ===");
    println!("material file: {material_path}");
    println!("material bytes (len): {}", material.len());
    println!("submitted-material fingerprint: {mat_fp}");
    println!("declared schemas: {declared:?}");
    println!("canonical n-quads:\n{nquads}");
    println!("canonical n-quads byte length: {}", nquads.len());
    println!("claim fingerprint: {fp}");
    println!("claim iri: {iri}");
    println!();
}

fn main() {
    // Positive vectors: the same-claim pair.
    vector(
        "V-CLAIM-1a (material_good_a)",
        "fixtures/material_good_a.jsonld",
        &[SCHEMA],
    );
    vector(
        "V-CLAIM-1b (material_good_b, same claim)",
        "fixtures/material_good_b.jsonld",
        &[SCHEMA],
    );

    // Minimal claim: one triple, one schema.
    let minimal = r#"{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":"v"}"#;
    let nquads = canon::jsonld_to_canonical_nquads(minimal).unwrap();
    let declared = schemas(&["https://x.example/schema/1"]);
    let fp = identity::claim_fingerprint(&declared, &nquads);
    println!("=== V-CLAIM-2 (minimal) ===");
    println!("material (exact bytes): {minimal}");
    println!(
        "submitted-material fingerprint: {}",
        identity::submitted_material_fingerprint(minimal.as_bytes())
    );
    println!("canonical n-quads:\n{nquads}");
    println!("claim fingerprint: {fp}");
    println!("claim iri: {}", claim_iri(&fp));
    println!();

    // Minimal claim with two schemas declared in reverse order -> sorted set.
    let declared2 = schemas(&["https://x.example/schema/2", "https://x.example/schema/1"]);
    let fp2 = identity::claim_fingerprint(&declared2, &nquads);
    println!("=== V-CLAIM-3 (minimal, two schemas, order-insensitive) ===");
    println!("declared (as submitted): [schema/2, schema/1]");
    println!("canonical declared order: {declared2:?}");
    println!("claim fingerprint: {fp2}");
    println!("claim iri: {}", claim_iri(&fp2));
    println!();

    // Snapshot vector over the two distinct claim IRIs above, membership
    // built from the new-form (origin-free) ClaimIRIs.
    let iri_min = claim_iri(&fp);
    let iri_two = claim_iri(&fp2);
    let membership: BTreeSet<String> = [iri_two.clone(), iri_min.clone()].into_iter().collect();
    let sfp = identity::snapshot_fingerprint(&membership);
    println!("=== V-SNAP-1 (two members, insertion order reversed) ===");
    println!("members (as supplied): [{iri_two}, {iri_min}]");
    println!("canonical sorted membership: {membership:?}");
    println!("snapshot fingerprint: {sfp}");
    println!("snapshot iri: {}", snapshot_iri(&sfp));
    println!();

    // §8 validation-claim example: a verdict over V-CLAIM-2 judged from its
    // authored form, with vocabulary terms under the protocol-fixed
    // claims:vocab:validation-result:1.0.0# namespace and target_claim in
    // the new-form ClaimIRI. Run through the real canonicalizer.
    let target = claim_iri(&fp);
    let judged_material = identity::submitted_material_fingerprint(minimal.as_bytes());
    let validation_material = format!(
        concat!(
            r#"{{"@context":{{"vr":"claims:vocab:validation-result:1.0.0#","#,
            r#""xsd":"http://www.w3.org/2001/XMLSchema#"}},"#,
            r#""@type":"vr:ValidationResult","#,
            r#""vr:validator":{{"@id":"https://validators.example/l1"}},"#,
            r#""vr:target_claim":{{"@id":"{target}"}},"#,
            r#""vr:schema_version":{{"@id":"https://x.example/schema/1"}},"#,
            r#""vr:outcome":"conforms","#,
            r#""vr:violations":{{"@value":"0","@type":"xsd:integer"}},"#,
            r#""vr:judged_material":"{judged}","#,
            r#""vr:validated_at":{{"@value":"2026-09-10T12:00:00Z","@type":"xsd:dateTime"}}}}"#,
        ),
        target = target,
        judged = judged_material,
    );
    let vnquads = canon::jsonld_to_canonical_nquads(&validation_material).unwrap();
    let vdeclared = schemas(&["claims:vocab:validation-result:1.0.0"]);
    let vfp = identity::claim_fingerprint(&vdeclared, &vnquads);
    println!("=== §8 validation-claim example (verdict over V-CLAIM-2, authored form) ===");
    println!("declared schemas: {vdeclared:?}");
    println!("canonical n-quads:\n{vnquads}");
    println!("claim fingerprint: {vfp}");
    println!("claim iri: {}", claim_iri(&vfp));
    println!();

    // Negative vectors.
    println!("=== N-1 (not UTF-8) ===");
    let bad_utf8: &[u8] = &[0x7b, 0xff, 0x7d];
    println!("material bytes: 7b ff 7d");
    println!(
        "utf8 check: {:?}",
        std::str::from_utf8(bad_utf8).map(|_| ()).err().unwrap()
    );
    println!();

    println!("=== N-2 (malformed JSON-LD) ===");
    println!("material: not json");
    println!(
        "result: {:?}",
        canon::jsonld_to_canonical_nquads("not json").err().unwrap()
    );
    println!();

    println!("=== N-3 (empty content) ===");
    println!("material: {{}}");
    let empty = canon::jsonld_to_canonical_nquads("{}").unwrap();
    println!("canonical n-quads: {empty:?} (empty -> reject)");
    println!();

    println!("=== N-4 (remote context, must not fetch) ===");
    let remote = r#"{"@context":"https://schema.org","name":"x"}"#;
    println!("material: {remote}");
    println!(
        "result: {:?}",
        canon::jsonld_to_canonical_nquads(remote).err()
    );
    println!();

    println!("=== N-5 (named graph in content) ===");
    let named_graph = r#"{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/g","@graph":[{"@id":"https://x.example/s","p":"v"}]}"#;
    println!("material: {named_graph}");
    match canon::jsonld_to_canonical_nquads(named_graph) {
        Ok(nq) => println!("canonical n-quads (note graph term -> reject at floor):\n{nq}"),
        Err(e) => println!("result: {e:?}"),
    }
    println!();

    println!("=== N-6 (relative schema IRI) ===");
    println!("declared: \"not-an-absolute-iri\"");
    println!(
        "iri well-formedness: {:?}",
        sophia::iri::Iri::new("not-an-absolute-iri")
            .map(|_| ())
            .err()
    );
}
