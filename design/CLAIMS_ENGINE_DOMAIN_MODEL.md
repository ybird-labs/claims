# Claims Engine Domain Model — Current Design

Date: 2026-04-25

This document captures the current abstract domain model for the Claims Engine.
It is a design-understanding document, not an implementation plan.
It preserves a two-source basis: this current domain model and `design/original_paper.md`.

## 1. Core premise

The Claims Engine is not a JSON-LD document store.

JSON-LD is one possible input or output serialization. The underlying domain is about immutable, provenance-bearing semantic assertions that are RDF-compatible and can be projected into graph form.

## 2. Claim

A **Claim** is:

> An accepted, durable, named semantic assertion with provenance.

Structurally:

```text
Claim =
  canonical ClaimIRI
  canonical asserted semantic content
  assertion provenance
  derivable canonical Claim fingerprint
```

Where:

```text
assertion provenance =
  assertor
  asserted_at
```

A Claim is the core durable domain object.

A Claim is not:

- raw submitted JSON-LD,
- an API payload,
- an envelope alone,
- a database row,
- an RDF parser object,
- a transport message,
- a graph index entry.

## 3. Claim identity

A Claim has both:

```text
ClaimId
canonical ClaimIRI
```

The `ClaimId` is the domain identity. The `ClaimIRI` is the canonical semantic identity/reference form.

Settled invariant:

```text
ClaimId -> exactly one canonical ClaimIRI
ClaimIRI -> exactly one immutable accepted Claim
```

The `ClaimIRI` identifies the Claim itself, not merely a graph, document, parser object, or storage record.

The `ClaimIRI` may be used in graph projection to organize or associate projected content, but its domain meaning is the Claim itself.

## 4. Asserted semantic content

A Claim's asserted content is:

> A canonical RDF-compatible semantic value.

For equality, fingerprinting, and projection, that value has:

> A canonical RDF dataset representation.

The Claim is semantic, not syntactic.

The Claim content is not:

- raw JSON-LD text,
- input formatting,
- transport envelope,
- parser-specific representation,
- database encoding.

Settled invariant:

```text
Claim asserted content must be non-empty.
```

Submitted material is separate from Claim content:

```text
submitted material = what arrived
claim content = accepted canonical semantic value
```

## 5. Submitted material

Original submitted material is separate from the Claim value by default.

Examples:

```text
JSON-LD document
Turtle document
N-Quads document
external attestation payload
API request body
file
```

Submitted material may be preserved as ingestion/audit evidence.

Submitted material may have its own fingerprint if exact received bytes need to be proven, but that is distinct from the derivable Claim fingerprint.

```text
submitted material fingerprint = exact received representation
Claim fingerprint = digest derived from accepted semantic Claim value
```

## 6. Assertion provenance

A Claim's mandatory provenance is:

```text
assertor
asserted_at
```

### 6.1 Assertor

The `assertor` is:

> The stable semantic identity of the entity responsible for making the assertion.

The assertor must have a canonical IRI or IRI-equivalent form.

It is not a mutable display label.

For example, the assertor should not be merely:

```text
"Alice"
"Land Agency"
"Bob's wallet"
```

unless that value is itself a stable, canonical, unambiguous semantic identifier.

The Claim fingerprint commits to the canonical assertor identifier, not to a mutable display name.

### 6.2 asserted_at

`asserted_at` is:

> The canonical instant when the assertor made the assertion.

It is timezone-aware at boundaries and canonicalized as an instant in the Claim value.

Equivalent timezone representations of the same instant must canonicalize to the same value.

Example:

```text
2026-04-25T10:00:00Z
2026-04-25T06:00:00-04:00
```

These represent the same instant and therefore canonicalize to the same `asserted_at` value.

If local time or timezone context is itself meaningful, it belongs in asserted semantic content, not in the core provenance timestamp.

## 7. Claim fingerprint

A **Claim fingerprint** is a derivable canonical cryptographic digest over the full immutable Claim value. It is computed from the canonical Claim value; it is not unconditional intrinsic stored state and is not part of its own preimage.

Its computation covers:

```text
canonical ClaimIRI
canonical RDF dataset representation of asserted content
canonical assertor identifier / IRI-equivalent
canonical asserted_at instant
```

Its computation does not cover:

```text
the fingerprint field itself
submitted material
ingestion metadata
accepted_at
operational state
storage location
database row id
API request id
```

Identity is part of the Claim fingerprint because identity is part of the Claim.

By itself, the fingerprint is recomputable from the same canonical Claim value. It supports trust or integrity checks only when compared to an independent commitment, such as:

```text
local immutable admission record
package digest
external anchor
attestation
signature
```

The commitment preimage remains the canonical ClaimIRI, canonical asserted content, canonical assertor, and canonical asserted_at. The exact canonicalization profile and digest suite remain unresolved.

## 8. Claim acceptance and durable admission

Submitted material becomes a Claim only after durable admission into the authoritative claim record.

Acceptance requires:

```text
canonical ClaimIRI
non-empty canonical asserted semantic content
assertor
asserted_at
derivable canonical Claim fingerprint
durable admission
```

**Durable admission** means:

> The Claims Engine recognizes the object as an accepted immutable Claim in its authoritative claim record.

It does not mean:

```text
indexed in L0
included in a Snapshot
published externally
anchored on-chain
stored in any specific database
```

After durable admission, the Claim is immutable.

Before durable admission, the material is submitted/candidate material, not a Claim.

## 9. accepted_at

`accepted_at` records:

> When the Claims Engine accepted submitted material as a durable Claim.

It is admission/audit metadata.

It is not:

```text
assertion provenance
part of asserted semantic content
part of the Claim fingerprint
```

Distinction:

```text
asserted_at = when the assertor made the assertion
accepted_at = when this engine accepted it as a Claim
```

## 10. No duplicate Claims

The Claims Engine does not admit duplicate Claims.

Claim uniqueness is determined by:

```text
canonical asserted semantic content
canonical assertor identifier
canonical asserted_at
```

Therefore:

```text
same content + same assertor + same asserted_at
= same Claim
= same ClaimIRI
```

Different submissions of the same claim substance are ingestion/audit history, not new Claims.

Differences that are only about ingestion do not make a new Claim:

```text
different submitter
different submitted_at
different source file
different API request
different serialization
```

A repeated assertion at a different `asserted_at` is a different Claim.

```text
same content + same assertor + same asserted_at = same Claim
same content + same assertor + different asserted_at = different Claim
```

### Document later

This uniqueness rule should be documented carefully because it affects identity policy:

> Claim uniqueness is based on canonical asserted semantic content + canonical assertor identifier + canonical asserted_at, not on submitted material or ingestion context.

Also document the interaction with `ClaimIRI`:

> The ClaimIRI identifies the unique accepted Claim produced from that canonical claim substance.

## 11. Snapshot

A **Snapshot** is:

> A stable named selection of immutable Claim references.

A Snapshot is not a Claim by default.

Structurally:

```text
Snapshot =
  canonical SnapshotIRI
  non-empty duplicate-free unordered set of canonical ClaimIRIs
  derivable canonical Snapshot fingerprint
```

Snapshot membership is by canonical `ClaimIRI`, not local-only ID.

For this engine's core domain:

```text
Snapshot membership selects Claims accepted by this engine.
```

Snapshots may be referenced by Claims because Snapshots have canonical IRIs.

## 12. Snapshot identity

A Snapshot has both:

```text
SnapshotId
canonical SnapshotIRI
```

The `SnapshotIRI` is the canonical semantic identity/reference form of the Snapshot.

A SnapshotIRI identifies the unique Snapshot for a canonical membership set.

```text
canonical membership set -> exactly one SnapshotIRI
```

How SnapshotIRI is produced is not yet decided.

Possible mechanisms remain out of scope for now:

```text
deterministically derived
assigned with uniqueness enforcement
externally named under rules
```

## 13. Snapshot membership

Snapshot membership is:

> A non-empty duplicate-free unordered set of canonical ClaimIRIs.

Order has no semantic meaning.

These are the same membership:

```text
{ClaimA, ClaimB}
{ClaimB, ClaimA}
```

Duplicates are not meaningful:

```text
{ClaimA, ClaimA} = {ClaimA}
```

For fingerprinting, membership is represented as:

```text
canonical sorted set of ClaimIRIs
```

## 14. No duplicate Snapshots

Snapshot uniqueness is determined by its canonical membership set.

```text
same canonical membership set
= same Snapshot
= same SnapshotIRI
```

Therefore:

```text
same membership -> same SnapshotIRI -> same Snapshot fingerprint
```

Two Snapshots cannot have the same membership under different SnapshotIRIs.

If different labels, purposes, releases, or descriptions are needed, those should be expressed as Claims about the Snapshot or through another later concept, not by duplicating the Snapshot.

## 15. Snapshot fingerprint

A **Snapshot fingerprint** is a derivable canonical cryptographic digest over the full immutable Snapshot value. It is computed from the canonical Snapshot value; it is not unconditional intrinsic stored state and is not part of its own preimage.

Its computation covers:

```text
canonical SnapshotIRI
canonical sorted duplicate-free non-empty set of canonical ClaimIRIs
```

Its computation does not cover:

```text
the fingerprint field itself
full Claim contents
Claim fingerprints
operational state
```

Claim contents are checked through recomputed Claim fingerprints compared against independent Claim commitments.

Snapshot membership is checked through a recomputed Snapshot fingerprint compared against an independent Snapshot commitment.

For this domain model, a Snapshot is narrowly:

```text
canonical SnapshotIRI
duplicate-free unordered ClaimIRI set
```

The broader paper concept of a snapshot/checkpoint maps to:

```text
Domain Snapshot + SnapshotCommitment/SnapshotAttestation
```

## 16. Commitments, anchors, and attestations

Fingerprints, commitments, anchors, attestations, and validation are separate concepts.

```text
fingerprint = what is committed to
commitment/anchor/attestation = where, by whom, and when it was committed
validation = schema/process judgment about the Claim, Snapshot, or graph state
```

A **ClaimCommitment** or **ClaimAttestation** records an independent commitment to a Claim fingerprint. It is useful for portable assertion-level integrity because the Claim fingerprint can be recomputed elsewhere and compared to the committed value.

A **SnapshotCommitment** or **SnapshotAttestation** records an independent commitment to a Snapshot fingerprint. It is useful for bounded evidence-set or checkpoint integrity because the Snapshot fingerprint can be recomputed from the SnapshotIRI and canonical membership set and compared to the committed value.

ClaimAttestation and SnapshotAttestation are complementary:

```text
ClaimAttestation = portable assertion-level commitment
SnapshotAttestation = bounded evidence-set/checkpoint commitment
```

Neither kind of attestation is the same as validation. Validation is a separate schema, process, or authority judgment over content, membership, or graph state.

## 17. Relationships

There is no separate `Relationship` primitive at this abstraction level.

A Claim can assert relations about anything addressable:

```text
Claims
Snapshots
people
organizations
documents
events
resources
external IRIs
```

So relationships are part of asserted semantic content, not separate core objects.

Content vocabulary constraints are out of scope for now.

### Revisit later

We need to later discuss:

> What constraints, if any, exist on asserted semantic content vocabulary and required shapes?

## 18. L0 and graph projections

Accepted Claims are the source of truth.

The **L0 claim graph** is:

> The canonical semantic projection over accepted Claims.

L0 does not own Claims.

Claims do not become real by being inserted into L0.

Projection is conceptually layered:

```text
L0 asserted-content projection:
  what Claims assert

metadata/provenance/snapshot projection:
  facts about Claims and Snapshots
```

These layers are separate but query-composable.

This allows queries across asserted content and provenance while preserving the distinction between:

```text
what the Claim says
```

and:

```text
metadata about the Claim
```

## 19. Asserted content vs provenance

A Claim has two distinct parts:

```text
asserted content = what the Claim says
provenance = who asserted it and when
```

These are both part of the Claim value, but they are not the same thing.

Projection invariant:

```text
L0 asserted-content projection:
  what Claims say

Claim metadata/provenance projection:
  facts about Claims
```

Simple rule:

> Do not confuse the thing said with the record of saying it.

## 20. External/source identifiers

The canonical `ClaimIRI` identifies:

> The immutable Claim accepted by our Claims Engine.

External/source identifiers identify external artifacts or records, such as:

```text
EAS attestation UID
IPFS CID
blockchain transaction/event ID
external registry URI
source document URI
```

By default, external/source identifiers are separate ingestion/audit/source context.

They do not automatically become the `ClaimIRI`.

An external identifier may appear inside asserted semantic content if the Claim explicitly asserts something about that external artifact.

### Revisit later

We need to later define when external identifiers belong in:

```text
ingestion/audit evidence
asserted semantic content
provenance
identity mapping
```

## 21. Core invariants

```text
1. ClaimIRI is permanent once accepted.

2. ClaimIRI identifies the Claim itself, not merely a graph.

3. Claim fingerprint is derivable from the canonical Claim value and excludes itself.

4. Snapshot fingerprint is derivable from the canonical Snapshot value and excludes itself.

5. Assertor is a stable canonical semantic identity, IRI / IRI-equivalent.

6. asserted_at is a canonical instant.

7. Claim asserted content is non-empty.

8. Claim asserted content has canonical RDF dataset representation.

9. Same claim substance resolves to same ClaimIRI.

10. Snapshot membership is a non-empty, duplicate-free, unordered ClaimIRI set.

11. Snapshot fingerprint uses canonical sorted membership representation.

12. Same Snapshot membership resolves to same SnapshotIRI.

13. Fingerprints provide trust/integrity only when compared to independent commitments, anchors, attestations, signatures, or immutable admission records.

14. Domain Snapshot is canonical SnapshotIRI plus duplicate-free unordered ClaimIRI set; paper snapshot/checkpoint is Domain Snapshot plus SnapshotCommitment/SnapshotAttestation.

15. L0 is derived from accepted Claims, not source of truth.

16. Asserted content, provenance, and system audit metadata remain distinct.
```

## 22. Remaining ambiguities / revisit later

```text
1. Exact RDF canonicalization method/version.

2. Exact ClaimIRI generation method.

3. Exact SnapshotIRI generation method.

4. Exact IRI scheme / namespace policy.

5. External/source identifier placement.

6. Content vocabulary constraints and required shapes.

7. Exact canonicalization profile and digest suite for Claim/Snapshot fingerprints.

8. Whether provenance/signature/authority concepts expand beyond assertor/asserted_at.

9. How submitted material/audit evidence is retained and fingerprinted.

10. Exact physical/query representation of L0 and metadata projections.
```

## 23. Compact definition

```text
A Claim is an immutable accepted semantic assertion with provenance.
It has canonical ClaimIRI, non-empty canonical RDF-compatible asserted content,
mandatory assertor/asserted_at provenance, and a derivable canonical Claim
fingerprint.

A Snapshot is a stable named selection of immutable Claim references.
It has canonical SnapshotIRI, a non-empty duplicate-free unordered set of
canonical ClaimIRIs, and a derivable canonical Snapshot fingerprint.

Fingerprints are recomputable digests over canonical values. They support
trust or integrity when compared to independent commitments, anchors,
attestations, signatures, or immutable admission records.

Claim attestations provide portable assertion-level commitment. Snapshot
attestations provide bounded evidence-set/checkpoint commitment.

The L0 claim graph is the canonical semantic projection over accepted Claims.

Submitted material is separate ingestion/audit evidence.

Relationships are not separate primitives; Claims may assert relations about
anything addressable, including Claims and Snapshots.
```
