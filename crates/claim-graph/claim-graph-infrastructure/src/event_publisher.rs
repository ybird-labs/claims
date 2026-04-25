use async_trait::async_trait;
use claim_graph_domain::ClaimGraphError;
use claim_graph_domain::EventPublisher;
use shared_kernel::DomainEvent;

pub struct NoopEventPublisher;

impl NoopEventPublisher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for NoopEventPublisher {
    async fn publish(&self, _events: &[Box<dyn DomainEvent>]) -> Result<(), ClaimGraphError> {
        Ok(())
    }
}
