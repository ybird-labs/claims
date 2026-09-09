// SPDX-License-Identifier: MPL-2.0

use super::{AssertionProvenance, ClaimContent, ClaimId, ClaimIri};

/// Provenance-bearing assertion of canonical claim content.
///
/// `Assertion` is the point where pure [`ClaimContent`] becomes something an
/// assertor said at a particular instant. It intentionally has no claim IRI or
/// local storage identity; those are added later by [`ClaimValue`] and [`Claim`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Assertion {
    content: ClaimContent,
    provenance: AssertionProvenance,
}

impl Assertion {
    pub fn new(content: ClaimContent, provenance: AssertionProvenance) -> Self {
        Self {
            content,
            provenance,
        }
    }

    pub fn content(&self) -> &ClaimContent {
        &self.content
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        &self.provenance
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClaimCandidate {
    assertion: Assertion,
}

impl ClaimCandidate {
    pub fn new(assertion: Assertion) -> Self {
        Self { assertion }
    }

    pub fn from_content(content: ClaimContent, provenance: AssertionProvenance) -> Self {
        Self::new(Assertion::new(content, provenance))
    }

    pub fn assertion(&self) -> &Assertion {
        &self.assertion
    }

    pub fn content(&self) -> &ClaimContent {
        self.assertion.content()
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        self.assertion.provenance()
    }

    /// Builds a claim value while keeping this candidate available.
    ///
    /// This clones the candidate's assertion. Use this when callers still need
    /// to inspect or reuse the candidate after deriving the claim value.
    pub fn to_claim_value(&self, iri: ClaimIri) -> ClaimValue {
        ClaimValue::new(iri, self.assertion.clone())
    }

    /// Converts this candidate into a claim value.
    ///
    /// This consumes the candidate and moves its assertion into the claim value
    /// without cloning. Use this for one-way admission flows where the candidate
    /// is no longer needed after conversion.
    pub fn into_claim_value(self, iri: ClaimIri) -> ClaimValue {
        ClaimValue::new(iri, self.assertion)
    }
}

/// Immutable claim value from which a claim fingerprint can be derived.
///
/// `ClaimValue` contains the domain fields that participate in claim-value
/// fingerprinting: the canonical Claim IRI and the provenance-bearing assertion
/// of canonical claim content. It intentionally excludes local engine metadata
/// such as storage identity or submitted material.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClaimValue {
    iri: ClaimIri,
    assertion: Assertion,
}

impl ClaimValue {
    pub fn new(iri: ClaimIri, assertion: Assertion) -> Self {
        Self { iri, assertion }
    }

    pub fn iri(&self) -> &ClaimIri {
        &self.iri
    }

    pub fn assertion(&self) -> &Assertion {
        &self.assertion
    }

    pub fn content(&self) -> &ClaimContent {
        self.assertion.content()
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        self.assertion.provenance()
    }
}

/// Accepted durable claim with local identity and immutable value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Claim {
    id: ClaimId,
    value: ClaimValue,
}

impl Claim {
    pub fn new(id: ClaimId, value: ClaimValue) -> Self {
        Self { id, value }
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

    pub fn assertion(&self) -> &Assertion {
        self.value.assertion()
    }

    pub fn content(&self) -> &ClaimContent {
        self.value.content()
    }

    pub fn provenance(&self) -> &AssertionProvenance {
        self.value.provenance()
    }
}

#[cfg(test)]
mod tests {
    use super::{Assertion, Claim, ClaimCandidate, ClaimValue};
    use crate::domain::{
        AssertedAt, AssertionProvenance, AssertorIri, CanonicalNQuads, CanonicalRdfContentEncoding,
        CanonicalRdfDataset, ClaimContent, ClaimId, ClaimIri, DateTimeUtc,
    };

    #[test]
    fn assertion_preserves_content_and_provenance() {
        let content = claim_content();
        let provenance = assertion_provenance();

        let assertion = Assertion::new(content.clone(), provenance.clone());

        assert_eq!(assertion.content(), &content);
        assert_eq!(assertion.provenance(), &provenance);
    }

    #[test]
    fn claim_candidate_preserves_assertion() {
        let assertion = assertion();

        let candidate = ClaimCandidate::new(assertion.clone());

        assert_eq!(candidate.assertion(), &assertion);
        assert_eq!(candidate.content(), assertion.content());
        assert_eq!(candidate.provenance(), assertion.provenance());
    }

    #[test]
    fn claim_candidate_from_content_builds_assertion() {
        let content = claim_content();
        let provenance = assertion_provenance();

        let candidate = ClaimCandidate::from_content(content.clone(), provenance.clone());

        assert_eq!(candidate.content(), &content);
        assert_eq!(candidate.provenance(), &provenance);
    }

    #[test]
    fn claim_candidate_to_claim_value() {
        let iri = ClaimIri::new("https://example.com/claims/1").unwrap();
        let assertion = assertion();

        let candidate = ClaimCandidate::new(assertion.clone());

        let value = candidate.to_claim_value(iri.clone());

        assert_eq!(value.iri(), &iri);
        assert_eq!(value.assertion(), &assertion);
    }

    #[test]
    fn claim_candidate_into_claim_value() {
        let iri = ClaimIri::new("https://example.com/claims/1").unwrap();
        let assertion = assertion();

        let candidate = ClaimCandidate::new(assertion.clone());

        let value = candidate.into_claim_value(iri.clone());

        assert_eq!(value.iri(), &iri);
        assert_eq!(value.assertion(), &assertion);
    }

    #[test]
    fn claim_value_preserves_committed_fields() {
        let iri = ClaimIri::new("https://example.com/claims/1").unwrap();
        let assertion = assertion();

        let value = ClaimValue::new(iri.clone(), assertion.clone());

        assert_eq!(value.iri(), &iri);
        assert_eq!(value.assertion(), &assertion);
        assert_eq!(value.content(), assertion.content());
        assert_eq!(value.provenance(), assertion.provenance());
    }

    #[test]
    fn claim_preserves_identity_and_value() {
        let id = ClaimId::new("claim-1");
        let value = ClaimValue::new(
            ClaimIri::new("https://example.com/claims/1").unwrap(),
            assertion(),
        );

        let claim = Claim::new(id.clone(), value.clone());

        assert_eq!(claim.id(), &id);
        assert_eq!(claim.value(), &value);
        assert_eq!(claim.iri(), value.iri());
        assert_eq!(claim.assertion(), value.assertion());
        assert_eq!(claim.content(), value.content());
        assert_eq!(claim.provenance(), value.provenance());
    }

    fn assertion() -> Assertion {
        Assertion::new(claim_content(), assertion_provenance())
    }

    fn claim_content() -> ClaimContent {
        let nquads = CanonicalNQuads::from_canonicalized(
            "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
        )
        .unwrap();

        ClaimContent::new(CanonicalRdfDataset::new(
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
