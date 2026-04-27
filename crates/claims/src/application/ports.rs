use super::error::ApplicationResult;
use crate::domain::{Claim, ClaimId, ClaimIri};

pub trait ClaimRepository {
    fn get_claim(&self, claim_id: &ClaimId) -> ApplicationResult<Option<Claim>>;

    fn get_claim_by_iri(&self, claim_iri: &ClaimIri) -> ApplicationResult<Option<Claim>>;

    fn insert_claim(&mut self, claim: Claim) -> ApplicationResult<()>;
}
