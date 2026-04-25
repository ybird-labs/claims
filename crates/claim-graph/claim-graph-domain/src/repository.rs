use crate::{ClaimGraph, ClaimGraphError, ClaimGraphId};
use async_trait::async_trait;

#[async_trait]
pub trait ClaimGraphRepository: Send + Sync {
    async fn find_by_id(&self, id: &ClaimGraphId) -> Result<Option<ClaimGraph>, ClaimGraphError>;
    async fn save(&self, claim_graph: &ClaimGraph) -> Result<(), ClaimGraphError>;
    async fn next_identity(&self) -> Result<ClaimGraphId, ClaimGraphError>;
}
