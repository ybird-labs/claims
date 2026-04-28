use super::error::ClaimRepositoryResult;
use crate::domain::{Claim, ClaimId, ClaimIri};

pub trait ClaimRepository {
    fn get_claim(&self, claim_id: &ClaimId) -> ClaimRepositoryResult<Option<Claim>>;

    fn get_claim_by_iri(&self, claim_iri: &ClaimIri) -> ClaimRepositoryResult<Option<Claim>>;

    fn insert_claim(&self, claim: Claim) -> ClaimRepositoryResult<()>;
}
