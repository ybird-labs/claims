use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DomainError {
    #[error("asserted content must not be empty")]
    EmptyAssertedContent,

    #[error("value must be a valid absolute IRI")]
    InvalidAbsoluteIri,

    #[error("invalid RFC 3339 instant format")]
    InvalidRfc3339InstantFormat,

    #[error("failed to format instant as RFC 3339")]
    Rfc3339InstantFormatFailed,
}
