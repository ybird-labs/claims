use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    EmptyAssertedContent,
    InvalidAbsoluteIri,
    InvalidCanonicalInstant,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAssertedContent => f.write_str("asserted content must not be empty"),
            Self::InvalidAbsoluteIri => f.write_str("value must be a valid absolute IRI"),
            Self::InvalidCanonicalInstant => {
                f.write_str("timestamp must be a valid canonical UTC instant")
            }
        }
    }
}

impl std::error::Error for DomainError {}
