use super::DomainError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AssertedContent {
    canonical_dataset: CanonicalRdfDataset,
}

impl AssertedContent {
    pub fn new(canonical_dataset: CanonicalRdfDataset) -> Self {
        Self { canonical_dataset }
    }

    pub fn canonical_dataset(&self) -> &CanonicalRdfDataset {
        &self.canonical_dataset
    }
}

/// Canonical RDF dataset representation for accepted asserted content.
///
/// This type stores output from the RDF canonicalization pipeline. It does not
/// parse, schema-validate, or canonicalize input by itself. Callers must provide
/// already-canonicalized N-Quads for the selected encoding profile.
///
/// The provided representation is preserved exactly because it participates in
/// claim-value fingerprinting.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CanonicalRdfDataset {
    encoding: CanonicalRdfContentEncoding,
    nquads: String,
}

impl CanonicalRdfDataset {
    pub fn from_canonical_nquads(
        encoding: CanonicalRdfContentEncoding,
        nquads: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let nquads = nquads.into();

        if nquads.trim().is_empty() {
            return Err(DomainError::EmptyAssertedContent);
        }

        Ok(Self { encoding, nquads })
    }

    pub fn encoding(&self) -> CanonicalRdfContentEncoding {
        self.encoding
    }

    pub fn as_str(&self) -> &str {
        &self.nquads
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.nquads.as_bytes()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CanonicalRdfContentEncoding {
    ClaimsRdfc10CanonicalNQuadsUtf8V1,
}

#[cfg(test)]
mod tests {
    use super::{CanonicalRdfContentEncoding, CanonicalRdfDataset};

    #[test]
    fn canonical_dataset_rejects_empty_canonical_nquads() {
        let err = CanonicalRdfDataset::from_canonical_nquads(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            "",
        )
        .unwrap_err();

        assert_eq!(err.to_string(), "asserted content must not be empty");
    }

    #[test]
    fn canonical_dataset_rejects_whitespace_only_canonical_nquads() {
        let err = CanonicalRdfDataset::from_canonical_nquads(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            " \n\t ",
        )
        .unwrap_err();

        assert_eq!(err.to_string(), "asserted content must not be empty");
    }

    #[test]
    fn canonical_dataset_preserves_canonical_nquads_exactly() {
        let nquads = "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n";

        let dataset = CanonicalRdfDataset::from_canonical_nquads(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            nquads,
        )
        .unwrap();

        assert_eq!(dataset.as_str(), nquads);
        assert_eq!(dataset.as_bytes(), nquads.as_bytes());
    }

    #[test]
    fn canonical_dataset_records_encoding_profile() {
        let dataset = CanonicalRdfDataset::from_canonical_nquads(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
        )
        .unwrap();

        assert_eq!(
            dataset.encoding(),
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1
        );
    }
}
