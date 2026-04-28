use std::{collections::BTreeMap, sync::RwLock};

use crate::{
    application::{ClaimRepository, ClaimRepositoryError, ClaimRepositoryResult},
    domain::{Claim, ClaimId, ClaimIri},
};

#[derive(Debug, Default)]
pub struct State {
    claims_by_id: BTreeMap<ClaimId, Claim>,
    claim_ids_by_iri: BTreeMap<ClaimIri, ClaimId>,
}

#[derive(Debug, Default)]
pub struct InMemoryClaimRepository {
    state: RwLock<State>,
}

impl InMemoryClaimRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ClaimRepository for InMemoryClaimRepository {
    fn get_claim(&self, claim_id: &ClaimId) -> ClaimRepositoryResult<Option<Claim>> {
        let state = self.state.read().unwrap();

        Ok(state.claims_by_id.get(claim_id).cloned())
    }

    fn get_claim_by_iri(&self, claim_iri: &ClaimIri) -> ClaimRepositoryResult<Option<Claim>> {
        let state = self.state.read().unwrap();
        let Some(claim_id) = state.claim_ids_by_iri.get(claim_iri) else {
            return Ok(None);
        };

        Ok(state.claims_by_id.get(claim_id).cloned())
    }

    fn insert_claim(&self, claim: Claim) -> ClaimRepositoryResult<()> {
        let mut state = self.state.write().unwrap();
        let claim_id = claim.id().clone();
        let claim_iri = claim.iri().clone();

        if state.claims_by_id.contains_key(&claim_id) {
            return Err(ClaimRepositoryError::DuplicateId(
                claim_id.as_str().to_string(),
            ));
        }

        if state.claim_ids_by_iri.contains_key(&claim_iri) {
            return Err(ClaimRepositoryError::DuplicateIri(
                claim_iri.as_str().to_string(),
            ));
        }

        state.claim_ids_by_iri.insert(claim_iri, claim_id.clone());
        state.claims_by_id.insert(claim_id, claim);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{ClaimRepository, ClaimRepositoryError},
        domain::{
            AssertedAt, AssertedContent, AssertionProvenance, AssertorIri, CanonicalNQuads,
            CanonicalRdfContentEncoding, CanonicalRdfDataset, Claim, ClaimFingerprint, ClaimId,
            ClaimIri, ClaimValue, DateTimeUtc, Sha256Digest,
        },
    };

    use super::InMemoryClaimRepository;

    #[test]
    fn inserts_and_gets_claim_by_id() {
        let repository = InMemoryClaimRepository::new();
        let claim = claim("claim-1", "https://example.com/claims/1", [1; 32]);

        repository.insert_claim(claim.clone()).unwrap();

        assert_eq!(repository.get_claim(claim.id()).unwrap(), Some(claim));
    }

    #[test]
    fn inserts_and_gets_claim_by_iri() {
        let repository = InMemoryClaimRepository::new();
        let claim = claim("claim-1", "https://example.com/claims/1", [1; 32]);

        repository.insert_claim(claim.clone()).unwrap();

        assert_eq!(
            repository.get_claim_by_iri(claim.iri()).unwrap(),
            Some(claim)
        );
    }

    #[test]
    fn unknown_claim_id_returns_none() {
        let repository = InMemoryClaimRepository::new();

        assert_eq!(
            repository.get_claim(&ClaimId::new("missing")).unwrap(),
            None
        );
    }

    #[test]
    fn unknown_claim_iri_returns_none() {
        let repository = InMemoryClaimRepository::new();

        assert_eq!(
            repository
                .get_claim_by_iri(&ClaimIri::new("https://example.com/claims/missing").unwrap())
                .unwrap(),
            None
        );
    }

    #[test]
    fn duplicate_claim_id_is_rejected() {
        let repository = InMemoryClaimRepository::new();
        let first = claim("claim-1", "https://example.com/claims/1", [1; 32]);
        let duplicate_id = claim("claim-1", "https://example.com/claims/2", [2; 32]);

        repository.insert_claim(first).unwrap();

        let err = repository.insert_claim(duplicate_id).unwrap_err();

        assert_eq!(
            err,
            ClaimRepositoryError::DuplicateId("claim-1".to_string())
        );
    }

    #[test]
    fn duplicate_claim_iri_is_rejected() {
        let repository = InMemoryClaimRepository::new();
        let first = claim("claim-1", "https://example.com/claims/1", [1; 32]);
        let duplicate_iri = claim("claim-2", "https://example.com/claims/1", [2; 32]);

        repository.insert_claim(first).unwrap();

        let err = repository.insert_claim(duplicate_iri).unwrap_err();

        assert_eq!(
            err,
            ClaimRepositoryError::DuplicateIri("https://example.com/claims/1".to_string())
        );
    }

    fn claim(id: &str, iri: &str, digest: [u8; 32]) -> Claim {
        Claim::new(
            ClaimId::new(id),
            ClaimValue::new(
                ClaimIri::new(iri).unwrap(),
                asserted_content(),
                assertion_provenance(),
            ),
            ClaimFingerprint::claim_value_rdfc10_canonical_nquads_utf8_sha256_v1(
                Sha256Digest::new(digest),
            ),
        )
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
