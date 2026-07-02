//! Content addressing: fingerprints and IRIs derived from canonical values.
//!
//! Provisional choices for design doc §20.2 (recorded in the spike README):
//! SHA-256 digests, hex encoding, `https://claims.example/` namespace.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

/// Versioned fingerprint suite for claim values (design doc §7).
pub const CLAIM_FINGERPRINT_SUITE: &str = "claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v1";

/// Versioned fingerprint suite for snapshot membership (design doc §14).
pub const SNAPSHOT_FINGERPRINT_SUITE: &str =
    "claims-spike/snapshot-membership/sorted-iris-utf8/sha256/v1";

/// Versioned fingerprint suite for exact submitted material bytes (design doc §10).
pub const SUBMITTED_MATERIAL_FINGERPRINT_SUITE: &str =
    "claims-spike/submitted-material/raw-bytes/sha256/v1";

/// Fingerprint over a canonical claim value: declared schema references
/// (canonical sorted duplicate-free set) plus canonical content (design doc
/// §3, §7). The preimage excludes the fingerprint, the ClaimIRI, and all
/// audit metadata (§18 invariants 2 and 7).
pub fn claim_fingerprint(declared_schemas: &BTreeSet<String>, canonical_nquads: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CLAIM_FINGERPRINT_SUITE.as_bytes());
    hasher.update(b"\n");
    for schema_iri in declared_schemas {
        hasher.update(b"schema ");
        hasher.update(schema_iri.as_bytes());
        hasher.update(b"\n");
    }
    hasher.update(b"---\n");
    hasher.update(canonical_nquads.as_bytes());
    hex::encode(hasher.finalize())
}

/// ClaimIRI derived from the claim fingerprint (design doc §7).
pub fn claim_iri(fingerprint_hex: &str) -> String {
    format!("https://claims.example/claim/sha256/{fingerprint_hex}")
}

/// Fingerprint over canonical sorted snapshot membership (design doc §14).
pub fn snapshot_fingerprint(membership: &BTreeSet<String>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(SNAPSHOT_FINGERPRINT_SUITE.as_bytes());
    hasher.update(b"\n");
    for claim_iri in membership {
        hasher.update(claim_iri.as_bytes());
        hasher.update(b"\n");
    }
    hex::encode(hasher.finalize())
}

/// SnapshotIRI derived from the snapshot fingerprint (design doc §14, §20.7
/// provisional choice: same derivation scheme as ClaimIRIs).
pub fn snapshot_iri(fingerprint_hex: &str) -> String {
    format!("https://claims.example/snapshot/sha256/{fingerprint_hex}")
}

/// Fingerprint over exact submitted material bytes (design doc §10, §11).
pub fn submitted_material_fingerprint(material: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(material);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schemas(iris: &[&str]) -> BTreeSet<String> {
        iris.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn claim_fingerprint_is_deterministic() {
        let a = claim_fingerprint(&schemas(&["https://x.example/s"]), "<a> <b> <c> .\n");
        let b = claim_fingerprint(&schemas(&["https://x.example/s"]), "<a> <b> <c> .\n");
        assert_eq!(a, b);
    }

    #[test]
    fn claim_fingerprint_covers_schema_declaration() {
        let a = claim_fingerprint(&schemas(&["https://x.example/s1"]), "<a> <b> <c> .\n");
        let b = claim_fingerprint(&schemas(&["https://x.example/s2"]), "<a> <b> <c> .\n");
        assert_ne!(a, b);
    }

    #[test]
    fn claim_fingerprint_covers_content() {
        let a = claim_fingerprint(&schemas(&["https://x.example/s"]), "<a> <b> <c> .\n");
        let b = claim_fingerprint(&schemas(&["https://x.example/s"]), "<a> <b> <d> .\n");
        assert_ne!(a, b);
    }

    #[test]
    fn snapshot_fingerprint_ignores_insertion_order() {
        let m1: BTreeSet<String> = ["https://c.example/2", "https://c.example/1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let m2: BTreeSet<String> = ["https://c.example/1", "https://c.example/2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(snapshot_fingerprint(&m1), snapshot_fingerprint(&m2));
    }
}
