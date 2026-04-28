use crate::domain::{Claim, ClaimId, ClaimIri};

use super::{ApplicationError, ApplicationResult, ClaimRepository, ClaimRepositoryError};

#[derive(Debug, Default)]
pub struct ClaimService<R> {
    repository: R,
}

impl<R> ClaimService<R>
where
    R: ClaimRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn insert_claim(&mut self, claim: Claim) -> ApplicationResult<()> {
        self.repository
            .insert_claim(claim)
            .map_err(|error| match error {
                ClaimRepositoryError::DuplicateId(claim_id) => {
                    ApplicationError::ClaimAlreadyExists(claim_id)
                }
                ClaimRepositoryError::DuplicateIri(claim_iri) => {
                    ApplicationError::ClaimIriAlreadyExists(claim_iri)
                }
                error => ApplicationError::ClaimRepository(error),
            })
    }

    pub fn get_claim(&self, claim_id: &ClaimId) -> ApplicationResult<Option<Claim>> {
        Ok(self.repository.get_claim(claim_id)?)
    }

    pub fn get_claim_by_iri(&self, claim_iri: &ClaimIri) -> ApplicationResult<Option<Claim>> {
        Ok(self.repository.get_claim_by_iri(claim_iri)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        AssertedAt, AssertedContent, AssertionProvenance, AssertorIri, CanonicalNQuads,
        CanonicalRdfContentEncoding, CanonicalRdfDataset, Claim, ClaimFingerprint, ClaimId,
        ClaimIri, ClaimValue, DateTimeUtc, Sha256Digest,
    };

    use crate::application::{
        ApplicationError, ClaimRepository, ClaimRepositoryError, ClaimRepositoryResult,
        ClaimService,
    };

    #[test]
    fn insert_claim_maps_late_duplicate_id_to_claim_already_exists() {
        let candidate = claim("claim-1", "https://example.com/claims/1", [1; 32]);
        let repository = StubClaimRepository {
            insert_error: Some(ClaimRepositoryError::DuplicateId("claim-1".to_string())),
        };
        let mut service = ClaimService::new(repository);

        let err = service.insert_claim(candidate).unwrap_err();

        assert_eq!(
            err,
            ApplicationError::ClaimAlreadyExists("claim-1".to_string())
        );
    }

    #[test]
    fn insert_claim_maps_late_duplicate_iri_to_claim_iri_already_exists() {
        let candidate = claim("claim-1", "https://example.com/claims/1", [1; 32]);
        let repository = StubClaimRepository {
            insert_error: Some(ClaimRepositoryError::DuplicateIri(
                "https://example.com/claims/1".to_string(),
            )),
        };
        let mut service = ClaimService::new(repository);

        let err = service.insert_claim(candidate).unwrap_err();

        assert_eq!(
            err,
            ApplicationError::ClaimIriAlreadyExists("https://example.com/claims/1".to_string())
        );
    }

    #[derive(Default)]
    struct StubClaimRepository {
        insert_error: Option<ClaimRepositoryError>,
    }

    impl ClaimRepository for StubClaimRepository {
        fn get_claim(&self, _claim_id: &ClaimId) -> ClaimRepositoryResult<Option<Claim>> {
            Ok(None)
        }

        fn get_claim_by_iri(&self, _claim_iri: &ClaimIri) -> ClaimRepositoryResult<Option<Claim>> {
            Ok(None)
        }

        fn insert_claim(&mut self, _claim: Claim) -> ClaimRepositoryResult<()> {
            match self.insert_error.clone() {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }
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
