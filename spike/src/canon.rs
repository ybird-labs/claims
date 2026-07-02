//! JSON-LD to canonical N-Quads (RDFC-1.0).
//!
//! Submitted material must be self-contained JSON-LD: the parser uses
//! sophia's `NoLoader`, so remote context references fail instead of being
//! fetched (design doc §10: contexts are pinned, never resolved remotely).

use sophia::api::prelude::*;
use sophia::inmem::dataset::LightDataset;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CanonError {
    #[error("material is not parseable self-contained JSON-LD: {0}")]
    Parse(String),
    #[error("RDFC-1.0 canonicalization failed: {0}")]
    Canonicalize(String),
}

/// Parses self-contained JSON-LD text into an RDF dataset and returns its
/// RDFC-1.0 canonical N-Quads representation (design doc §4, §20.1).
pub fn jsonld_to_canonical_nquads(material: &str) -> Result<String, CanonError> {
    let dataset: LightDataset = sophia::jsonld::parser::parse_str(material)
        .collect_quads()
        .map_err(|e| CanonError::Parse(e.to_string()))?;

    canonical_nquads(&dataset)
}

/// RDFC-1.0 canonical N-Quads for an in-memory dataset.
pub fn canonical_nquads(dataset: &LightDataset) -> Result<String, CanonError> {
    let mut out = Vec::<u8>::new();
    sophia::c14n::rdfc10::normalize(dataset, &mut out)
        .map_err(|e| CanonError::Canonicalize(e.to_string()))?;

    String::from_utf8(out).map_err(|e| CanonError::Canonicalize(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equivalent_documents_canonicalize_identically() {
        let a = r#"{"@context":{"p":"https://x.example/p"},"p":"v","@id":"https://x.example/s"}"#;
        let b = r#"{"@id":"https://x.example/s","p":"v","@context":{"p":"https://x.example/p"}}"#;

        assert_eq!(
            jsonld_to_canonical_nquads(a).unwrap(),
            jsonld_to_canonical_nquads(b).unwrap()
        );
    }

    #[test]
    fn blank_nodes_get_canonical_labels() {
        let doc = r#"{"@context":{"p":"https://x.example/p"},"p":"v"}"#;

        let nquads = jsonld_to_canonical_nquads(doc).unwrap();

        assert!(nquads.contains("_:c14n0"), "got: {nquads}");
    }

    #[test]
    fn contentless_document_yields_empty_nquads() {
        assert_eq!(jsonld_to_canonical_nquads("{}").unwrap(), "");
    }

    #[test]
    fn malformed_json_is_rejected() {
        assert!(matches!(
            jsonld_to_canonical_nquads("not json"),
            Err(CanonError::Parse(_))
        ));
    }
}
