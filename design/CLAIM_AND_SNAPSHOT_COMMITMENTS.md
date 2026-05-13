# Claim and Snapshot Commitments

This design note aligns with `design/CLAIMS_ENGINE_DOMAIN_MODEL.md` and `design/original_paper.md`. It clarifies how Claims, Snapshots, fingerprints, commitments, anchors, attestations, and validation relate.

## Core Distinctions

A fingerprint or digest is a recomputable digest of canonical content. It is not, by itself, proof that a Claim is true, accepted by a trusted party, anchored at a time, unchanged from an independently observed state, or portable across systems.

Trust and portability come from comparing the recomputed fingerprint or digest to an independent commitment, anchor, attestation, signature, immutable record, or other external proof surface.

The Claims Engine should therefore distinguish:

- the domain object being fingerprinted,
- the recomputable digest of that object,
- a local record that the engine committed to that digest,
- an external anchor or attestation that independently records the digest,
- validation judgments about whether a Claim or Snapshot satisfies a schema or process.

## Terms

### Claim Fingerprint / Digest

A Claim fingerprint, or Claim digest, is a cryptographic digest derived from the full immutable Claim value, excluding the fingerprint field itself.

Per the current domain model, it is derived from:

```text
canonical ClaimIRI
canonical RDF dataset representation of asserted content
canonical assertor identifier / IRI-equivalent
canonical asserted_at instant
```

It does not cover submitted material, ingestion metadata, accepted_at, operational state, storage location, API request IDs, or the fingerprint field itself.

The digest can be recomputed from the canonical Claim value. Recomputing it shows consistency with that value and canonicalization profile; it does not prove external trust.

### Snapshot Fingerprint / Digest / Root

A Snapshot fingerprint, Snapshot digest, or Snapshot root is a cryptographic digest derived from the full immutable Snapshot value, excluding the fingerprint field itself.

Per the current domain model, it is derived from:

```text
canonical SnapshotIRI
canonical sorted duplicate-free non-empty set of canonical ClaimIRIs
```

It does not directly digest full Claim contents or Claim fingerprints. Snapshot membership consistency is checked through the Snapshot fingerprint. The contents of each member Claim are checked through the corresponding Claim fingerprint and any relevant independent commitment, anchor, attestation, signature, package digest, or immutable/admission record.

The term root is useful when the Snapshot digest is implemented as a Merkle root or another accumulator root. The abstract domain requirement is the same: a deterministic digest over the canonical Snapshot value.

### LocalCommitment

A LocalCommitment is the Claims Engine's own durable record that it computed and accepted a digest for a Claim or Snapshot.

It may record fields such as:

```text
subject IRI
subject type: Claim or Snapshot
digest value
digest algorithm
canonicalization profile
committed_at
local admission or checkpoint reference
```

A LocalCommitment is useful for audit and replay inside this engine. It is not equivalent to an independent external anchor unless another system can verify it independently.

### ExternalAnchor

An ExternalAnchor is an independent immutable or append-only record that binds a Claim or Snapshot digest to an external system.

Examples include:

```text
blockchain data record
transaction event
registry entry
content-addressed publication
timestamping service record
```

An ExternalAnchor should preserve enough metadata to verify what was anchored: subject IRI, digest, digest algorithm, canonicalization profile, anchoring system, anchor identifier, anchoring time, and any submitting or signing identity available from that system.

### ClaimAttestation

A ClaimAttestation is a portable proof or commitment for one assertion: one immutable Claim.

It binds a ClaimIRI and Claim digest to an attester, signature or attestation mechanism, time, and optional expiry or revocation semantics.

Claim attestations are useful when a verifier needs to ask:

```text
Did this attester commit to this exact Claim value?
```

### SnapshotAttestation

A SnapshotAttestation is a portable proof or commitment for a bounded evidence set or checkpoint: one immutable Snapshot.

It binds a SnapshotIRI and Snapshot digest/root to an attester, signature or attestation mechanism, time, optional parent snapshot/root, scope/view identifier, and optional expiry or revocation semantics.

Snapshot attestations are useful when a verifier needs to ask:

```text
Did this attester commit to this exact bounded set of Claims as a checkpoint?
```

ClaimAttestation and SnapshotAttestation are related but not interchangeable. A ClaimAttestation concerns one assertion. A SnapshotAttestation concerns a selected evidence set and can support validation, funding, certification, or governance checkpoints.

### ValidationResult

A ValidationResult is a schema or process judgment about a Claim, Snapshot, or derived object. It answers whether the subject satisfies a specified rule set, method, workflow, role policy, or verification process.

In the layer model from the original paper, validation is generally an L1 judgment over a Snapshot. It may be represented as another Claim or result object that references:

```text
validated SnapshotIRI
schema or process identifier
validator / verifier identity
validation time
pass/fail/score/status
rationale or evidence references
related attestation or anchor identifiers
```

Validation is separate from fingerprinting and anchoring. A Snapshot can be correctly fingerprinted and anchored while still failing a schema. A ValidationResult may itself be fingerprinted, anchored, attested, or asserted as a Claim.

## Verification Flows

### Claim Attestation Verification

To verify a ClaimAttestation:

1. Retrieve the Claim by ClaimIRI or receive the Claim value from the presenter.
2. Canonicalize the Claim using the attestation's stated canonicalization profile.
3. Recompute the Claim digest using the stated digest algorithm.
4. Compare the recomputed digest to the digest in the ClaimAttestation.
5. Verify the attestation signature, attester identity, anchor record, or immutable external record.
6. Check revocation, expiry, chain finality, timestamp, or other policy conditions required by the verifier.
7. Treat the Claim as matching the attested commitment only if the digest and independent proof surface both verify.

This flow shows that the attester or anchor committed to the exact canonical Claim value. It does not show that the asserted content is true unless the verifier's trust policy gives that attester or anchor that authority.

### Snapshot Attestation Verification

To verify a SnapshotAttestation:

1. Retrieve the Snapshot by SnapshotIRI or receive the Snapshot value from the presenter.
2. Canonicalize the Snapshot membership as a sorted duplicate-free non-empty set of canonical ClaimIRIs.
3. Recompute the Snapshot digest/root using the stated canonicalization profile and digest algorithm.
4. Compare the recomputed digest/root to the digest/root in the SnapshotAttestation.
5. Verify the attestation signature, attester identity, anchor record, or immutable external record.
6. Check revocation, expiry, chain finality, timestamp, lineage, parent root, or view/scope policy required by the verifier.
7. For any member Claim whose content matters, retrieve that Claim and verify its Claim fingerprint or ClaimAttestation separately.

This flow shows that the attester or anchor committed to the exact bounded set of ClaimIRIs. It does not directly show every member Claim's content, truth, or schema validity.

## Validation Is Separate

Validation is not the same as digest verification.

Digest verification asks:

```text
Does this canonical object match this commitment?
```

Attestation or anchor verification asks:

```text
Did this independent proof surface commit to that digest under an accepted trust policy?
```

Validation asks:

```text
Does this Claim or Snapshot satisfy this schema, method, role policy, or verification process?
```

For the MVP and likely early L1 design, validation should generally evaluate a Snapshot rather than an unbounded live graph. The Snapshot fixes the evidence set/checkpoint, and the ValidationResult records the judgment over that fixed subject.

Because validation results are themselves assertions, they may later be represented as Claims. For example, a validator can assert that Snapshot S passed Schema X at time T with rationale R.

## MVP Stance

The MVP should compute Claim fingerprints and Snapshot fingerprints now.

External attestation and anchoring can be deferred while keeping the model ready for both ClaimAttestation and SnapshotAttestation.

The immediate implementation should avoid coupling domain objects to a specific chain, attestation network, RDF parser, storage backend, or signature provider. The domain needs stable concepts and recomputable digests. External anchors and attestations can live at the application/infrastructure edge behind explicit ports when needed.

Recommended MVP posture:

- compute Claim fingerprints at durable Claim admission and store them only if admission or replay policy requires it,
- compute Snapshot fingerprints at Snapshot creation and store them only if checkpoint or replay policy requires it,
- retain algorithm and canonicalization metadata where needed for replay,
- model local commitments separately from external anchors,
- keep room for ClaimAttestation and SnapshotAttestation records without requiring them for initial admission,
- treat validation as a separate use case over Snapshots.

## Open Questions

1. Which canonicalization profile is authoritative for Claim asserted content and Snapshot membership?
2. Which digest algorithm is required for the first implementation, and how is algorithm agility represented?
3. Should ClaimIRI be deterministically derived from canonical claim substance, assigned with uniqueness enforcement, or externally named under policy?
4. Does local durable admission store the Claim digest and Snapshot digest as first-class records, or are they always recomputed from canonical state?
5. What is the first external adapter: Regen x/data, EAS-style attestation, content-addressed storage, timestamping service, or another registry?
6. How should revocation and expiry be represented for ClaimAttestation and SnapshotAttestation?
7. How should ValidationResult revocation, supersession, expiry, or replacement be represented if validation is asserted as another Claim?
