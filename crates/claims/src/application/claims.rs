use crate::domain::{Claim, ClaimId, ClaimIri};

use super::{ApplicationResult, ClaimRepository};

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
        self.repository.insert_claim(claim)
    }

    pub fn get_claim(&self, claim_id: &ClaimId) -> ApplicationResult<Option<Claim>> {
        self.repository.get_claim(claim_id)
    }

    pub fn get_claim_by_iri(&self, claim_iri: &ClaimIri) -> ApplicationResult<Option<Claim>> {
        self.repository.get_claim_by_iri(claim_iri)
    }
}
