use claim_graph_domain::ClaimGraph;
use claim_graph_domain::ClaimGraphId;
use claim_graph_domain::ClaimGraphRepository;
use claim_graph_domain::EventPublisher;

pub struct CreateClaimGraphUseCase<R, E> {
    repo: R,
    events: E,
}

impl<R, E> CreateClaimGraphUseCase<R, E>
where
    R: ClaimGraphRepository,
    E: EventPublisher,
{
    pub fn new(repo: R, events: E) -> Self {
        Self { repo, events }
    }

    pub async fn execute(
        &self,
        _cmd: crate::CreateClaimGraphCommand,
    ) -> Result<ClaimGraphId, crate::ClaimGraphApplicationError> {
        let id = self.repo.next_identity().await?;
        let claim_graph = ClaimGraph::create(id.clone())?;
        self.repo.save(&claim_graph).await?;
        self.events
            .publish(claim_graph.uncommitted_events())
            .await?;
        Ok(id)
    }
}
