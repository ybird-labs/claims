use async_trait::async_trait;
use claim_graph_domain::{ClaimGraph, ClaimGraphError, ClaimGraphId, ClaimGraphRepository};

pub struct NoopClaimGraphRepository;

impl NoopClaimGraphRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopClaimGraphRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ClaimGraphRepository for NoopClaimGraphRepository {
    async fn find_by_id(&self, _id: &ClaimGraphId) -> Result<Option<ClaimGraph>, ClaimGraphError> {
        Ok(None)
    }

    async fn save(&self, _claim_graph: &ClaimGraph) -> Result<(), ClaimGraphError> {
        Ok(())
    }

    async fn next_identity(&self) -> Result<ClaimGraphId, ClaimGraphError> {
        Ok(ClaimGraphId::new())
    }
}
