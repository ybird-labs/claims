use super::{AssertedContent, AssertionProvenance, ClaimFingerprint, ClaimId, ClaimIri};

/// Immutable claim value committed to by a [`ClaimFingerprint`].
///
/// `ClaimValue` contains the domain fields that participate in claim-value
/// fingerprinting: the canonical Claim IRI, canonical asserted content, and
/// assertion provenance. It intentionally excludes the fingerprint itself and
/// local engine metadata such as storage identity or submitted material.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClaimValue {
    iri: ClaimIri,
    asserted_content: AssertedContent,
    provenance: AssertionProvenance,
}

impl ClaimValue {
    pub fn new(
        iri: ClaimIri,
        asserted_content: AssertedContent,
        provenance: AssertionProvenance,
    ) -> Self {
        Self {
            iri,
            asserted_content,
            provenance,
        }
    }

    pub fn iri(&self) -> &ClaimIri {
        &self.iri
    }

    pub fn asserted_content(&self) -> &AssertedContent {
        &self.asserted_content
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        &self.provenance
    }
}

/// Accepted durable claim with local identity, immutable value, and fingerprint.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Claim {
    id: ClaimId,
    value: ClaimValue,
    fingerprint: ClaimFingerprint,
}

impl Claim {
    pub fn new(id: ClaimId, value: ClaimValue, fingerprint: ClaimFingerprint) -> Self {
        Self {
            id,
            value,
            fingerprint,
        }
    }

    pub fn id(&self) -> &ClaimId {
        &self.id
    }

    pub fn value(&self) -> &ClaimValue {
        &self.value
    }

    pub fn iri(&self) -> &ClaimIri {
        self.value.iri()
    }

    pub fn asserted_content(&self) -> &AssertedContent {
        self.value.asserted_content()
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        self.value.provenance()
    }

    pub fn fingerprint(&self) -> &ClaimFingerprint {
        &self.fingerprint
    }
}

#[cfg(test)]
mod tests {
    use super::{Claim, ClaimValue};
    use crate::domain::{
        AssertedAt, AssertedContent, AssertionProvenance, AssertorIri, CanonicalNQuads,
        CanonicalRdfContentEncoding, CanonicalRdfDataset, ClaimFingerprint, ClaimId, ClaimIri,
        DateTimeUtc, Sha256Digest,
    };

    #[test]
    fn claim_value_preserves_committed_fields() {
        let iri = ClaimIri::new("https://example.com/claims/1").unwrap();
        let content = asserted_content();
        let provenance = assertion_provenance();

        let value = ClaimValue::new(iri.clone(), content.clone(), provenance.clone());

        assert_eq!(value.iri(), &iri);
        assert_eq!(value.asserted_content(), &content);
        assert_eq!(value.provenance(), &provenance);
    }

    #[test]
    fn claim_preserves_identity_value_and_fingerprint() {
        let id = ClaimId::new("claim-1");
        let value = ClaimValue::new(
            ClaimIri::new("https://example.com/claims/1").unwrap(),
            asserted_content(),
            assertion_provenance(),
        );
        let fingerprint = ClaimFingerprint::claim_value_rdfc10_canonical_nquads_utf8_sha256_v1(
            Sha256Digest::new([7; 32]),
        );

        let claim = Claim::new(id.clone(), value.clone(), fingerprint);

        assert_eq!(claim.id(), &id);
        assert_eq!(claim.value(), &value);
        assert_eq!(claim.iri(), value.iri());
        assert_eq!(claim.asserted_content(), value.asserted_content());
        assert_eq!(claim.provenance(), value.provenance());
        assert_eq!(claim.fingerprint(), &fingerprint);
    }

    fn asserted_content() -> AssertedContent {
        let nquads = CanonicalNQuads::from_canonicalized(
            "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
        )
        .unwrap();

        AssertedContent::new(CanonicalRdfDataset::new(
            CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
            nquads,
        ))
    }

    fn assertion_provenance() -> AssertionProvenance {
        AssertionProvenance::new(
            AssertorIri::new("https://example.com/assertors/alice").unwrap(),
            AssertedAt::new(DateTimeUtc::parse_rfc3339("2026-04-25T10:00:00Z").unwrap()),
        )
    }
}
