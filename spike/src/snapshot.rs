//! Snapshots: stable named selections of immutable claim references
//! (design doc §14).

use std::collections::BTreeSet;

use crate::identity;

/// Immutable snapshot: canonical SnapshotIRI, non-empty duplicate-free
/// unordered membership set of ClaimIRIs, derivable fingerprint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    iri: String,
    membership: BTreeSet<String>,
    fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SnapshotError {
    #[error("snapshot membership must not be empty")]
    EmptyMembership,
}

impl Snapshot {
    /// Membership is a set: duplicates collapse, order is meaningless. The
    /// fingerprint is computed over the canonical sorted representation, so
    /// the same membership always resolves to the same SnapshotIRI (§18
    /// invariant 11).
    pub fn create(membership: impl IntoIterator<Item = String>) -> Result<Self, SnapshotError> {
        let membership: BTreeSet<String> = membership.into_iter().collect();

        if membership.is_empty() {
            return Err(SnapshotError::EmptyMembership);
        }

        let fingerprint = identity::snapshot_fingerprint(&membership);

        Ok(Self {
            iri: identity::snapshot_iri(&fingerprint),
            membership,
            fingerprint,
        })
    }

    pub fn iri(&self) -> &str {
        &self.iri
    }

    pub fn membership(&self) -> &BTreeSet<String> {
        &self.membership
    }

    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_membership_resolves_to_same_snapshot() {
        let a =
            Snapshot::create(["https://c.example/1".into(), "https://c.example/2".into()]).unwrap();
        let b = Snapshot::create([
            "https://c.example/2".into(),
            "https://c.example/1".into(),
            "https://c.example/1".into(),
        ])
        .unwrap();

        assert_eq!(a, b);
        assert_eq!(a.membership().len(), 2);
    }

    #[test]
    fn empty_membership_is_rejected() {
        assert_eq!(
            Snapshot::create(std::iter::empty()).unwrap_err(),
            SnapshotError::EmptyMembership
        );
    }
}
