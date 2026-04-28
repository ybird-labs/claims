use claims::{
    application::{ApplicationError, ClaimService},
    domain::{
        AssertedAt, AssertedContent, AssertionProvenance, AssertorIri, CanonicalNQuads,
        CanonicalRdfContentEncoding, CanonicalRdfDataset, Claim, ClaimFingerprint, ClaimId,
        ClaimIri, ClaimValue, DateTimeUtc, Sha256Digest,
    },
    infrastructure::InMemoryClaimRepository,
};

#[test]
fn claim_service_inserts_and_reads_claim_through_in_memory_repository() {
    let repository = InMemoryClaimRepository::new();
    let service = ClaimService::new(repository);
    let claim = make_claim("claim-1", "https://example.com/claims/1", [1; 32]);

    service.insert_claim(claim.clone()).unwrap();
    assert_eq!(service.get_claim(claim.id()).unwrap(), Some(claim.clone()));
    assert_eq!(service.get_claim_by_iri(claim.iri()).unwrap(), Some(claim));
}

#[test]
fn claim_service_inserts_same_claim_twice() {
    let repository = InMemoryClaimRepository::new();
    let service = ClaimService::new(repository);
    let claim = make_claim("claim-1", "https://example.com/claims/1", [1; 32]);

    service.insert_claim(claim.clone()).unwrap();
    let err = service.insert_claim(claim.clone()).unwrap_err();
    assert_eq!(
        err,
        ApplicationError::ClaimAlreadyExists(claim.id().clone())
    );
}

#[test]
fn claim_service_get_unknown_claim() {
    let repository = InMemoryClaimRepository::new();
    let service = ClaimService::new(repository);
    let claim_id = ClaimId::new("missing");
    assert_eq!(service.get_claim(&claim_id).unwrap(), None);
}

fn make_claim(id: &str, iri: &str, digest: [u8; 32]) -> Claim {
    Claim::new(
        ClaimId::new(id),
        ClaimValue::new(
            ClaimIri::new(iri).unwrap(),
            asserted_content(),
            assertion_provenance(),
        ),
        ClaimFingerprint::claim_value_rdfc10_canonical_nquads_utf8_sha256_v1(Sha256Digest::new(
            digest,
        )),
    )
}

fn asserted_content() -> AssertedContent {
    let nquads = CanonicalNQuads::from_canonicalized(
        "<https://example.com/s> <https://example.com/p> <https://example.com/o> .\n",
    )
    .unwrap();
    AssertedContent::new(CanonicalRdfDataset::new(
        CanonicalRdfContentEncoding::ClaimsRdfc10CanonicalNQuadsUtf8V1,
        nquads,
    ))
}
fn assertion_provenance() -> AssertionProvenance {
    AssertionProvenance::new(
        AssertorIri::new("https://example.com/assertors/alice").unwrap(),
        AssertedAt::new(DateTimeUtc::parse_rfc3339("2026-04-25T10:00:00Z").unwrap()),
    )
}
