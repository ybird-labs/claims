use crate::{ClaimGraphError, ClaimGraphId};
use shared_kernel::DomainEvent;

#[derive(Debug)]
pub struct ClaimGraph {
    id: ClaimGraphId,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

impl ClaimGraph {
    pub fn create(id: ClaimGraphId) -> Result<Self, ClaimGraphError> {
        Ok(Self {
            id,
            uncommitted_events: Vec::new(),
        })
    }

    pub fn id(&self) -> &ClaimGraphId {
        &self.id
    }

    pub fn uncommitted_events(&self) -> &[Box<dyn DomainEvent>] {
        &self.uncommitted_events
    }

    pub fn clear_events(&mut self) {
        self.uncommitted_events.clear();
    }
}
