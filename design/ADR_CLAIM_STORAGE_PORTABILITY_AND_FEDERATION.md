# ADR: Claim Storage, Portability, and Federation

Date: 2026-05-11

Status: Discussion

## Context

The Claims Engine needs to support durable local admission, portable claim
exchange, optional external anchoring, and future federation between independently
run engine instances.

The domain model already distinguishes:

```text
SubmittedMaterial != ClaimCandidate != Claim
ClaimIRI != SnapshotIRI
ClaimFingerprint != SnapshotFingerprint != SubmittedMaterialFingerprint
canonical asserted content != raw submitted material
L0/projection != source of truth
```

This ADR records the current storage and portability direction so later
implementation can preserve those boundaries.

## Decision

The Claims Engine separates three concerns:

```text
ClaimRepository
  Authoritative store for Claims durably admitted by this engine instance.

ClaimPackageStore
  Non-authoritative retrieval/distribution storage for portable ClaimPackage
  artifacts.

ClaimAnchorPublisher
  Non-authoritative publisher of external commitments, anchors, or attestations.
```

A Claim is accepted by an engine instance only after durable admission into that
instance's authoritative `ClaimRepository`.

`ClaimFingerprint` is a derived digest of the canonical Claim value. It may be
included in packages and referenced by commitment records, but the core Claim
aggregate does not need to store it as intrinsic mutable state.

Package stores, IPFS, blob stores, Regen x/data, EAS, Ethereum, indexers,
gateways, and external repositories may provide availability, integrity,
timestamping, discovery, optional publication/distribution commitments, or audit
evidence. They do not decide Claim admission or truth by default.

## Local Authority

The Claims Engine is instance-authoritative, not globally authoritative by
default.

Each running instance has its own authoritative admission state:

```text
Engine A admitted Claim X
Engine B admitted Claim X
Engine C has not admitted Claim X
```

Those are local admission facts. No package store, anchor, or remote engine makes
a Claim universally accepted by all engines.

## Package Types

### SourcePackage / EvidencePackage

A `SourcePackage` or `EvidencePackage` represents raw submitted/source material.
It may contain:

```text
raw JSON-LD / RDF / source documents
source metadata
JSON-LD contexts or pinned context references
submitted material fingerprint
signatures
external source references
```

It is not a Claim. It is candidate/source material from which an engine may derive
canonical asserted content and then admit a Claim.

### ClaimPackage

A `ClaimPackage` is the portable interchange artifact for an already
canonicalized Claim value.

It should contain at least:

```text
ClaimIRI
canonical asserted RDF dataset
assertor
asserted_at
ClaimFingerprint derived from the canonical Claim value
canonicalization profile
digest / fingerprint suite
Claims Engine package/profile version
```

It may also contain optional non-fingerprint metadata:

```text
accepted_at
originating engine metadata
source package references
submitted material fingerprint
package storage receipts
anchor / attestation references
signatures
import/export provenance
```

Optional metadata is excluded from `ClaimFingerprint` unless a later fingerprint
suite explicitly says otherwise.

The package `ClaimFingerprint` is the packaged digest used for verification and
interchange. It is not evidence that the Claim aggregate carries a stored,
mutable fingerprint field.

## Importing Portable ClaimPackages

The engine may ingest `ClaimPackage` artifacts produced elsewhere.

An imported package is not automatically an accepted local Claim. Import follows
this shape:

```text
1. Fetch ClaimPackage from a package store, another engine, or a discovered URI.
2. Parse the package manifest.
3. Verify supported canonicalization and fingerprint suites.
4. Recompute ClaimFingerprint from the canonical Claim value.
5. Compare recomputed fingerprint with package ClaimFingerprint.
6. Check local ClaimIRI conflict rules.
7. Apply local admission policy.
8. If admitted, preserve the imported ClaimIRI and the verified package or
   commitment digest where applicable.
9. Optionally mirror the package into local package stores.
10. Record source locators, anchors, and import details as audit metadata.
```

The current import decision is identity adoption:

```text
When a verified external ClaimPackage is locally admitted, preserve its ClaimIRI
and preserve the verified package digest or committed digest where applicable.
```

This keeps the same Claim portable across repositories, package stores,
snapshots, anchors, and independently run engine instances.

## Identity Conflict Rules

Imports must handle identity conflicts explicitly:

```text
If imported ClaimIRI does not exist locally:
  verify package, then admit preserving ClaimIRI and the verified package or
  committed digest where applicable.

If imported ClaimIRI exists locally with the same ClaimFingerprint:
  treat as duplicate import, mirror, or additional source/audit receipt.

If imported ClaimIRI exists locally with a different ClaimFingerprint:
  reject or quarantine as an identity conflict.

If package ClaimFingerprint does not verify:
  reject.

If package uses unsupported canonicalization or fingerprint suite:
  reject or mark unsupported.
```

Never overwrite an existing admitted Claim because of an imported package.

## Admission and Publication Order

Default ordering:

```text
1. Admit Claim into authoritative ClaimRepository.
2. Create or serialize ClaimPackage.
3. Store ClaimPackage in one or more ClaimPackageStore backends.
4. Optionally publish anchors or attestations.
5. Record package and anchor receipts as audit/publication metadata.
```

Package storage and external anchoring are publication/distribution steps. They
should not block local admission by default.

External anchors, attestations, signatures, audit records, and admission records
may reference a committed `ClaimFingerprint`. Those records commit to the derived
canonical Claim digest; they do not make the fingerprint intrinsic mutable Claim
state.

If a deployment requires every accepted Claim to be externally retrievable or
anchored before use, that should be modeled as a deployment-specific admission or
publication policy, not as a default domain invariant.

## Federation Model

Multiple people may run independent Claims Engine instances.

Federation happens through portable packages and optional discovery/anchor layers:

```text
Engine A admits Claim X.
Engine A publishes a ClaimPackage.
Engine B fetches and verifies the ClaimPackage.
Engine B admits the same Claim locally, preserving ClaimIRI and the verified
package or committed digest where applicable.
Engine B projects the Claim into its local graph for analysis.
```

This provides:

```text
local admission authority
portable claim exchange
cross-engine identity adoption
local graph/intelligence analysis
optional package mirroring
optional external anchoring
```

## Deterministic Identity Direction

Long term, ClaimIRI generation should be deterministic from Claim substance so
independent engines can derive the same Claim identity when processing the same
canonical substance.

Define:

```text
ClaimSubstance =
  canonical asserted RDF dataset
  + canonical assertor identifier
  + canonical asserted_at
```

Then a future deterministic profile may define:

```text
ClaimSubstanceFingerprint =
  hash(versioned, domain-separated canonical ClaimSubstance preimage)

ClaimIRI =
  deterministic IRI derived from ClaimSubstanceFingerprint under a declared
  Claims Engine namespace/profile

ClaimFingerprint =
  hash(versioned, domain-separated canonical Claim value preimage)
```

Where the canonical Claim value preimage commits to:

```text
ClaimIRI
canonical asserted RDF dataset
canonical assertor identifier
canonical asserted_at
canonicalization profile
digest / fingerprint suite
```

This avoids circularity. `ClaimIRI` must not be derived from `ClaimFingerprint`
if `ClaimFingerprint` includes `ClaimIRI`.

`ClaimSubstanceFingerprint` is an identity-convergence helper, not a replacement
for `ClaimFingerprint`.

## Fingerprint Boundaries

`ClaimFingerprint` commits to the immutable canonical Claim value, not to
packaging, storage, or external publication metadata. The same digest can appear
in different roles:

```text
Derived ClaimFingerprint
  recomputable digest from the canonical Claim value

Packaged ClaimFingerprint
  digest included in a portable ClaimPackage for verification/interchange

Committed ClaimFingerprint
  digest referenced by a local commitment, anchor, attestation, signature,
  audit record, or admission record
```

These roles do not require the core Claim aggregate to store
`ClaimFingerprint` as intrinsic mutable state.

It should exclude by default:

```text
the fingerprint field itself
submitted material
accepted_at
storage location
IPFS CID
blob object key
Regen x/data identifier
EAS UID
Ethereum transaction hash
indexer row id
package envelope metadata
```

Package hashes and `ClaimFingerprint` are different:

```text
ClaimFingerprint
  commitment to the canonical Claim value

Package hash / CID
  commitment to a serialized package/envelope representation
```

Claim-level and Snapshot-level attestations are complementary. A Claim-level
attestation provides a portable assertion-level commitment. A Snapshot-level
attestation provides a bounded evidence-set or checkpoint commitment.

## RDF and Linked Data Requirements

Before federation is treated as interoperable, the engine must specify exact
canonicalization and verification details:

```text
RDF dataset canonicalization algorithm/version
canonical N-Quads UTF-8 byte rules
line ending and final newline policy
JSON-LD processing mode
JSON-LD context pinning / document loader policy
base IRI behavior
named graph policy
canonical assertor identifier policy
canonical asserted_at timestamp format and precision
domain-separated, length-delimited fingerprint preimage format
ClaimIRI deterministic namespace/profile
```

Core verification must not depend on dereferencing arbitrary external IRIs.
Dereferencing can support discovery, but it is not proof by itself.

This verification is integrity and canonicalization checking, not validation,
truth, or trust by itself.

If `ClaimIRI` appears inside the asserted RDF content, deterministic identity can
become self-referential. The default design should avoid self-referential
`ClaimIRI` inside asserted content unless a later profile defines a placeholder
or self-reference rule.

## Consequences

Positive consequences:

```text
clear DDD boundaries
portable claim exchange
support for local, IPFS, blob, Arweave, Regen, EAS, and Ethereum adapters
federation without global consensus by default
claim integrity can be verified independently from package/storage location
```

Tradeoffs and risks:

```text
accepted Claims may temporarily exist before package publication succeeds
requires retry/outbox/publication tracking
requires careful identity conflict handling
requires exact canonicalization and fingerprint preimage specifications
foreign ClaimIRIs enter local repositories through identity adoption
deterministic identity needs a precise namespace/profile decision
```

## Open Questions

```text
1. Exact ClaimPackage serialization format.
2. Exact ClaimSubstanceFingerprint preimage format.
3. Exact deterministic ClaimIRI namespace/profile.
4. Whether ClaimRepository should be named ClaimRepository or AdmittedClaimRepository.
5. Whether SourcePackage or EvidencePackage is the better term for raw inputs.
6. Which package metadata belongs in the package envelope versus audit records.
7. Whether publication/anchoring uses an outbox or another retry mechanism.
8. Which external namespaces or package signatures are trusted for identity adoption.
```
