use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

use super::DomainError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AssertedAt(DateTimeUtc);

impl AssertedAt {
    pub fn new(value: DateTimeUtc) -> Self {
        Self(value)
    }

    pub fn get(self) -> DateTimeUtc {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DateTimeUtc(OffsetDateTime);

impl DateTimeUtc {
    pub fn new(value: OffsetDateTime) -> Self {
        Self(value.to_offset(UtcOffset::UTC))
    }

    pub fn parse_rfc3339(value: &str) -> Result<Self, DomainError> {
        let parsed = OffsetDateTime::parse(value, &Rfc3339)
            .map_err(|_| DomainError::InvalidCanonicalInstant)?;

        Ok(Self::new(parsed))
    }

    pub fn get(self) -> OffsetDateTime {
        self.0
    }

    pub fn format_rfc3339(self) -> Result<String, DomainError> {
        self.0
            .format(&Rfc3339)
            .map_err(|_| DomainError::InvalidCanonicalInstant)
    }
}

#[cfg(test)]
mod tests {
    use super::DateTimeUtc;

    #[test]
    fn datetime_utc_normalizes_equivalent_offsets() {
        let utc = DateTimeUtc::parse_rfc3339("2026-04-25T10:00:00Z").unwrap();
        let offset = DateTimeUtc::parse_rfc3339("2026-04-25T06:00:00-04:00").unwrap();

        assert_eq!(utc, offset);
    }

    #[test]
    fn datetime_utc_rejects_invalid_timestamp() {
        let err = DateTimeUtc::parse_rfc3339("not-a-timestamp").unwrap_err();

        assert_eq!(
            err.to_string(),
            "timestamp must be a valid canonical UTC instant"
        );
    }
}
