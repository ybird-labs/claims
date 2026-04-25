use claim_graph_domain::ClaimGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClaimGraphApplicationError {
    #[error("domain error: {0}")]
    Domain(#[from] ClaimGraphError),

    #[error("not found")]
    NotFound,

    #[error("infrastructure failure")]
    Infrastructure {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl ClaimGraphApplicationError {
    pub fn infrastructure<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self::Infrastructure {
            source: Box::new(err),
        }
    }
}
