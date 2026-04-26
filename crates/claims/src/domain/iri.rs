use std::fmt;
use std::str::FromStr;

use oxiri::Iri;

use super::DomainError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AbsoluteIri(String);

impl AbsoluteIri {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        Iri::parse(value.as_str()).map_err(|_| DomainError::InvalidAbsoluteIri)?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AbsoluteIri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AbsoluteIri {
    type Err = DomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

macro_rules! domain_iri {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(AbsoluteIri);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
                Ok(Self(AbsoluteIri::new(value)?))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn as_absolute_iri(&self) -> &AbsoluteIri {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

// Domain IRIs
domain_iri!(AssertorIri);
domain_iri!(ClaimIri);
domain_iri!(ClaimTypeIri);
domain_iri!(SnapshotIri);

#[cfg(test)]
mod tests {
    use super::{AbsoluteIri, ClaimIri};

    #[test]
    fn absolute_iri_accepts_valid_absolute_iri() {
        let iri = AbsoluteIri::new("https://example.com/claims/1").unwrap();

        assert_eq!(iri.as_str(), "https://example.com/claims/1");
    }

    #[test]
    fn absolute_iri_rejects_relative_reference() {
        let err = AbsoluteIri::new("claims/1").unwrap_err();

        assert_eq!(err.to_string(), "value must be a valid absolute IRI");
    }

    #[test]
    fn claim_iri_keeps_domain_specific_type() {
        let iri = ClaimIri::new("https://example.com/claims/1").unwrap();

        assert_eq!(iri.as_str(), "https://example.com/claims/1");
    }
}
