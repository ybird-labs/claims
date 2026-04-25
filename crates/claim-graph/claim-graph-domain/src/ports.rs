use crate::ClaimGraphError;
use async_trait::async_trait;
use shared_kernel::DomainEvent;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, events: &[Box<dyn DomainEvent>]) -> Result<(), ClaimGraphError>;
}
