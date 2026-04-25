use chrono::{DateTime, Utc};

pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn version(&self) -> u64;
}
