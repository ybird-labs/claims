# Agent Development Guardrails

These instructions apply to all agents and humans editing this repository.

## Architecture decision

This project currently uses a single Rust crate:

```text
crates/claims
```

Inside that crate, keep clear module boundaries:

```text
claims::domain
claims::application
claims::infrastructure
claims::projection
claims::api      // when added
```

We are intentionally starting with modules rather than separate `claims-domain`,
`claims-application`, and `claims-infrastructure` crates. Extract crates later
only when the boundary is proven by real reuse, heavy dependencies, multiple
binaries/adapters, or architectural drift.

## Non-negotiable dependency rule

The domain module must remain pure.

```text
domain must not depend on infrastructure, API, storage, projection, HTTP, DB,
RDF parser implementation, triplestore implementation, external SDKs, or DTOs.
```

Allowed dependency direction:

```text
infrastructure -> application -> domain
projection     -> domain
api            -> application/domain
```

Forbidden examples:

```rust
// Never inside claims::domain
use crate::infrastructure::*;
use crate::api::*;
use sqlx::*;
use axum::*;
use reqwest::*;
use sophia::*;        // parser/library-specific RDF types stay at the edge
use oxigraph::*;      // projection/store-specific types stay at the edge
use serde_json::Value; // API/input shape is not the domain model
```

If a use case needs behavior implemented by infrastructure, define a small port
/ trait in `application`, then implement it in `infrastructure`.

Do not put repository, storage, canonicalizer, projection, event-publishing, or
external-adapter ports in `domain` by default. Only add a trait to `domain` when
a documented domain invariant itself cannot be expressed without that pure
abstraction. Async IO-facing traits do not belong in `domain`.

## Domain modeling rules

Keep these distinctions encoded in Rust types:

```text
SubmittedMaterial != ClaimContent != Assertion != ClaimCandidate != Claim
ClaimIri != SnapshotIri
AssertedAt != AcceptedAt != SubmittedAt
ClaimFingerprint != SnapshotFingerprint != SubmittedMaterialFingerprint
canonical claim content != raw submitted material
L0/projection != source of truth
```

Do not collapse these into raw `String`, generic `DateTime<Utc>`, or generic
`Fingerprint` fields unless there is an explicit design decision.

## Where things belong

### `domain/`

Pure domain concepts and invariants:

```text
Claim
ClaimCandidate
Snapshot
SnapshotMembership
SubmittedMaterial
ClaimContent
Assertion
AssertionProvenance
ClaimIri / SnapshotIri
AssertedAt / AcceptedAt / SubmittedAt
ClaimFingerprint / SnapshotFingerprint
```

### `application/`

Use-case orchestration and commands:

```text
SubmitClaimMaterial
AdmitCanonicalClaim
CreateSnapshot
ResolveClaim
ResolveSnapshot
```

### `infrastructure/`

Adapters and implementation details:

```text
in-memory repositories
SQL/document/event-store repositories
RDF/JSON-LD parser integrations
canonicalization implementations
projection writers
external anchoring/attestation integrations
```

### `projection/`

Derived read/query models such as L0 graph indexes. These are rebuildable views,
not authoritative domain state.

## Before adding a dependency

Ask:

1. Is this dependency needed by pure domain logic?
2. Would adding it make `domain` know about transport, storage, RDF parser, or projection details?
3. Can this live in `infrastructure` behind a trait instead?

Default answer: dependencies belong at the edge, not in `domain`.

## Testing expectation

Domain tests should run without network, database, filesystem services, RDF
stores, or HTTP frameworks.

If a domain test needs one of those, the code probably belongs outside `domain`.
