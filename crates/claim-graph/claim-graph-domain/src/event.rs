use crate::ClaimGraphId;
use chrono::{DateTime, Utc};
use shared_kernel::DomainEvent;

#[derive(Debug, Clone)]
pub struct ClaimGraphCreated {
    pub claim_graph_id: ClaimGraphId,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for ClaimGraphCreated {
    fn event_type(&self) -> &'static str {
        "ClaimGraphCreated"
    }

    fn aggregate_id(&self) -> &str {
        self.claim_graph_id.as_str()
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn version(&self) -> u64 {
        1
    }
}
