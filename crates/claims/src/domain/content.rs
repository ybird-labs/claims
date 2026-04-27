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

/// Canonical N-Quads representation produced by a trusted RDF canonicalization pipeline.
///
/// This is a pure domain value used for claim identity and fingerprinting. It
/// stores canonicalized N-Quads text but does not carry RDF parser or
/// canonicalizer implementation types.
///
/// `from_canonicalized` is a trusted-boundary constructor: it does not parse RDF,
/// validate N-Quads syntax, or prove RDF Dataset Canonicalization 1.0. It only
/// rejects empty or whitespace-only text. Callers must use it only with text
/// produced by a trusted canonicalization pipeline outside the domain.
///
/// The representation is preserved exactly because the RDF canonicalization
/// pipeline owns whitespace control and these bytes participate in claim-value
/// fingerprinting.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CanonicalNQuads {
    value: String,
}

impl CanonicalNQuads {
    pub fn from_canonicalized(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DomainError::EmptyAssertedContent);
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }
}

/// Canonical RDF dataset representation for accepted asserted content.
///
/// This type pairs canonical N-Quads with the selected encoding profile. It does
/// not parse, trim, schema-validate, or canonicalize input by itself.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CanonicalRdfDataset {
    encoding: CanonicalRdfContentEncoding,
    nquads: CanonicalNQuads,
}

impl CanonicalRdfDataset {
    pub fn new(encoding: CanonicalRdfContentEncoding, nquads: CanonicalNQuads) -> Self {
        Self { encoding, nquads }
    }

    pub fn encoding(&self) -> CanonicalRdfContentEncoding {
        self.encoding
    }

    pub fn nquads(&self) -> &CanonicalNQuads {
        &self.nquads
    }

    pub fn as_str(&self) -> &str {
        self.nquads.as_str()
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
    use super::{CanonicalNQuads, CanonicalRdfContentEncoding, CanonicalRdfDataset};

    #[test]
    fn canonical_nquads_rejects_empty_value() {
        let err = CanonicalNQuads::from_canonicalized("").unwrap_err();

        assert_eq!(err.to_string(), "asserted content must not be empty");
    }

    #[test]
    fn canonical_nquads_rejects_whitespace_only_value() {
        let err = CanonicalNQuads::from_canonicalized(" \n\t ").unwrap_err();

        assert_eq!(err.to_string(), "asserted content must not be empty");
    }

    #[test]
    fn canonical_nquads_preserves_value_exactly() {
        let value = "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n";

        let nquads = CanonicalNQuads::from_canonicalized(value).unwrap();

        assert_eq!(nquads.as_str(), value);
        assert_eq!(nquads.as_bytes(), value.as_bytes());
    }

    #[test]
    fn canonical_dataset_exposes_canonical_nquads() {
        let nquads = CanonicalNQuads::from_canonicalized(
            "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
        )
        .unwrap();
        let dataset = CanonicalRdfDataset::new(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            nquads.clone(),
        );

        assert_eq!(dataset.nquads(), &nquads);
        assert_eq!(dataset.as_str(), nquads.as_str());
        assert_eq!(dataset.as_bytes(), nquads.as_bytes());
    }

    #[test]
    fn canonical_dataset_records_encoding_profile() {
        let nquads = CanonicalNQuads::from_canonicalized(
            "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
        )
        .unwrap();
        let dataset = CanonicalRdfDataset::new(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            nquads,
        );

        assert_eq!(
            dataset.encoding(),
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1
        );
    }
}
