/// Cryptographic commitment to an immutable [`ClaimValue`](crate::domain::ClaimValue).
///
/// A claim-value fingerprint commits to the canonical Claim IRI, canonical
/// asserted RDF dataset, canonical assertor IRI, and canonical `asserted_at`
/// instant. It excludes the fingerprint field itself, submitted material,
/// ingestion metadata, storage identity, and other operational metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ClaimFingerprint {
    suite: ClaimFingerprintSuite,
    digest: Sha256Digest,
}

impl ClaimFingerprint {
    pub fn new(suite: ClaimFingerprintSuite, digest: Sha256Digest) -> Self {
        Self { suite, digest }
    }

    pub fn claim_value_rdfc10_canonical_nquads_utf8_sha256_v1(digest: Sha256Digest) -> Self {
        Self::new(
            ClaimFingerprintSuite::ClaimValueRdfc10CanonicalNQuadsUtf8Sha256V1,
            digest,
        )
    }

    pub fn suite(&self) -> ClaimFingerprintSuite {
        self.suite
    }

    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }
}

/// Versioned algorithm/profile used to compute a [`ClaimFingerprint`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ClaimFingerprintSuite {
    /// SHA-256 over the Claims v1 canonical claim-value representation.
    ClaimValueRdfc10CanonicalNQuadsUtf8Sha256V1,
}

/// Cryptographic commitment to an immutable snapshot value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SnapshotFingerprint {
    suite: SnapshotFingerprintSuite,
    digest: Sha256Digest,
}

impl SnapshotFingerprint {
    pub fn new(suite: SnapshotFingerprintSuite, digest: Sha256Digest) -> Self {
        Self { suite, digest }
    }

    pub fn snapshot_membership_canonical_claim_iris_utf8_sha256_v1(digest: Sha256Digest) -> Self {
        Self::new(
            SnapshotFingerprintSuite::SnapshotMembershipCanonicalClaimIrisUtf8Sha256V1,
            digest,
        )
    }

    pub fn suite(&self) -> SnapshotFingerprintSuite {
        self.suite
    }

    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }
}

/// Versioned algorithm/profile used to compute a [`SnapshotFingerprint`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SnapshotFingerprintSuite {
    /// SHA-256 over the Claims v1 canonical snapshot membership representation.
    SnapshotMembershipCanonicalClaimIrisUtf8Sha256V1,
}

/// Cryptographic commitment to exact submitted material representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SubmittedMaterialFingerprint {
    suite: SubmittedMaterialFingerprintSuite,
    digest: Sha256Digest,
}

impl SubmittedMaterialFingerprint {
    pub fn new(suite: SubmittedMaterialFingerprintSuite, digest: Sha256Digest) -> Self {
        Self { suite, digest }
    }

    pub fn submitted_material_raw_bytes_sha256_v1(digest: Sha256Digest) -> Self {
        Self::new(
            SubmittedMaterialFingerprintSuite::SubmittedMaterialRawBytesSha256V1,
            digest,
        )
    }

    pub fn suite(&self) -> SubmittedMaterialFingerprintSuite {
        self.suite
    }

    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }
}

/// Versioned algorithm/profile used to compute a [`SubmittedMaterialFingerprint`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SubmittedMaterialFingerprintSuite {
    /// SHA-256 over the exact submitted material bytes received by the engine.
    SubmittedMaterialRawBytesSha256V1,
}

/// Fixed-size SHA-256 digest bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClaimFingerprint, ClaimFingerprintSuite, Sha256Digest, SnapshotFingerprint,
        SnapshotFingerprintSuite, SubmittedMaterialFingerprint, SubmittedMaterialFingerprintSuite,
    };

    #[test]
    fn claim_fingerprint_records_claim_value_suite_and_digest() {
        let digest = Sha256Digest::new([7; 32]);
        let fingerprint =
            ClaimFingerprint::claim_value_rdfc10_canonical_nquads_utf8_sha256_v1(digest);

        assert_eq!(
            fingerprint.suite(),
            ClaimFingerprintSuite::ClaimValueRdfc10CanonicalNQuadsUtf8Sha256V1
        );
        assert_eq!(fingerprint.digest(), &digest);
    }

    #[test]
    fn snapshot_fingerprint_records_snapshot_suite_and_digest() {
        let digest = Sha256Digest::new([8; 32]);
        let fingerprint =
            SnapshotFingerprint::snapshot_membership_canonical_claim_iris_utf8_sha256_v1(digest);

        assert_eq!(
            fingerprint.suite(),
            SnapshotFingerprintSuite::SnapshotMembershipCanonicalClaimIrisUtf8Sha256V1
        );
        assert_eq!(fingerprint.digest(), &digest);
    }

    #[test]
    fn submitted_material_fingerprint_records_submitted_material_suite_and_digest() {
        let digest = Sha256Digest::new([9; 32]);
        let fingerprint =
            SubmittedMaterialFingerprint::submitted_material_raw_bytes_sha256_v1(digest);

        assert_eq!(
            fingerprint.suite(),
            SubmittedMaterialFingerprintSuite::SubmittedMaterialRawBytesSha256V1
        );
        assert_eq!(fingerprint.digest(), &digest);
    }
}
