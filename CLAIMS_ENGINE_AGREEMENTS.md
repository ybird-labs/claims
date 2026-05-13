# Claims Engine — Working Agreements

Date: 2026-04-25

This document captures the current working agreement about the scope and product direction of the Claims Engine.

## 1. Product Focus

The Claims Engine should currently be focused on building **claim graphs**.

It is not, at this stage, a marketplace, carbon registry, crediting system, asset issuance platform, or financial product. Those may become downstream applications later, but they are not part of the immediate product scope.

The Claims Engine should be understood as:

> An abstract graph system for creating, linking, storing, querying, snapshotting, and validating structured claims.

The domain of interest is regenerative/ecological impact accounting, but the engine itself should remain abstract enough to represent many kinds of claims.

## 2. Core Product Boundary

The Claims Engine is responsible for:

1. Accepting structured claims.
2. Representing those claims as linked semantic data.
3. Building an L0 claim graph.
4. Linking claims to evidence, actors, timestamps, locations, methods, and other claims.
5. Supporting relationships between claims such as support, contradiction, endorsement, provenance, derivation, and verification.
6. Creating scoped snapshots of parts of the graph.
7. Computing derivable fingerprints/digests of claims and snapshots.
8. Remaining compatible with later validation of snapshots, claims, or subgraphs against explicit schemas.
9. Producing validation results as new claims in the graph when validation is implemented.

The Claims Engine is not currently responsible for:

- marketplaces,
- carbon registry workflows,
- credit issuance,
- tokenization,
- payments,
- asset trading,
- credit retirement,
- buyer/seller flows,
- portfolio management,
- downstream financial products.

A useful boundary statement:

> For now, the Claims Engine stops at producing claim graphs, snapshots, and referenceable claim artifacts, while remaining compatible with later validation claims. Anything that uses those claims to create credits, certificates, markets, payments, or governance actions is downstream and out of scope.

## 3. Claims Engine v1

The agreed v1 direction is:

1. Accept JSON-LD claims.
2. Normalize/canonicalize them into RDF.
3. Store/index them as an L0 claim graph.
4. Create scoped snapshots.
5. Compute derivable snapshot hashes/fingerprints.
6. Publish, persist, anchor, or attest claims/snapshots through substrate-neutral mechanisms.
7. Remain ready to validate claims, snapshots, or subgraphs against explicit, schema-specific rules later. *Further research needed on exact validation mechanics.*
8. Later produce L1 validation results as claims that may themselves be signed or attested.
9. Let downstream systems reference those validation results.

The key revision is that the Claims Engine should **not** be hard-bound to Regen `x/data`.

Instead, it should be substrate-neutral. Regen `x/data`, Ethereum attestations, EAS, IPFS, Arweave, institutional repositories, local stores, or other systems can all be possible publication/anchoring/attestation backends, not core dependencies.

At the product level, this means:

> The Claims Engine creates canonical claim, snapshot, and validation artifacts. Those artifacts can then be published, stored, anchored, or attested through different substrates depending on context.

We are not yet defining concrete engineering interfaces for these storage or attestation mechanisms. The current agreement is product-level: the Claims Engine should remain portable and substrate-neutral.

## 4. Layer Model

### L0 — Claim Graph

L0 is the main product focus.

It is the open, evolving graph of claims and their relationships. It contains structured assertions about observations, measurements, evidence, methods, actors, places, times, endorsements, contradictions, and provenance.

L0 does not globally decide truth. It represents claims and the relationships between them.

### Snapshot — Stable Graph View

A snapshot is a narrow evidence bundle: a stable, bounded set of claim references from part of the L0 graph.

It provides a stable evidence bundle that can be referenced, fingerprinted, discussed, attested, anchored, and validated. Purpose, timestamping, publication, commitment, and attestation belong to surrounding records rather than the narrow Snapshot value. A snapshot fingerprint is a derivable digest of the canonical snapshot value. It supports comparison against later commitments or records, but is not standalone proof by itself. *Further research needed on exact canonicalization, hashing, and inclusion-proof mechanics.*

Snapshots allow a dynamic graph to support institutional or programmatic checkpoints without freezing the whole graph. The engine helps create the narrow snapshot artifact; a paper-style checkpoint is represented by that Snapshot plus surrounding commitment, attestation, or validation claims.

### L1 — Schema Validation Claim

L1 records schema-specific validation results for claims, snapshots, or subgraphs.

An L1 result is itself a claim, for example:

> Snapshot S conforms to Schema X, according to Validator Y, at Time T, with Result Z.

When added, L1 should focus on additive validation over claims, snapshots, or subgraphs using explicit RDF-compatible schemas or rule sets. The engine should not hard-code a specific domain schema or use case. *Further research needed on the exact validation mechanism and supported schema formats.*

### Lₙ — Downstream Consumers

Lₙ represents downstream products, workflows, assets, triggers, or governance actions that may consume claims, snapshots, or validation results.

This layer is acknowledged conceptually but is out of scope for the current product focus.

## 5. Product Positioning

The Claims Engine can be described as:

> A substrate-neutral engine for creating, linking, snapshotting, and validating semantic claim graphs.

Or:

> A system for turning distributed evidence and assertions into reusable, inspectable, and validatable claim graphs.

Or, more simply:

> Claim graph infrastructure for impact accounting.

The initial product promise is:

> Users can represent complex impact evidence as an inspectable graph of claims instead of a pile of documents.

## 6. Important Conceptual Agreements

### 6.1 The Claim Is the Core Object

The core object is not a credit, certificate, asset, report, or registry entry.

The core object is a claim in a graph of related claims.

A claim may describe:

- an observation,
- a measurement,
- an action,
- a method,
- a location,
- an actor,
- a time period,
- evidence,
- an endorsement,
- a contradiction,
- a verification result,
- a validation result.

### 6.2 Claims Are Linked

Claims become valuable because they link to other claims.

Important relationship types include:

- supports,
- contradicts,
- endorses,
- disputes,
- derives from,
- cites,
- verifies,
- invalidates,
- qualifies,
- supersedes,
- was generated by,
- was observed at,
- used method,
- has evidence.

### 6.3 Validation Is Not the Same as Truth

The graph can contain conflicting or competing claims.

Validation means a claim, snapshot, or subgraph satisfies a specific schema, rule set, or process. It does not mean the system has determined universal truth.

Different schemas, authorities, communities, or institutions may validate the same graph differently.

### 6.4 Attestation/Anchoring Is Not the Same as Truth or Validation

Publishing, storing, anchoring, or attesting a claim or snapshot does not make it true, and does not mean it has been validated.

These mechanisms provide some combination of:

- availability,
- integrity,
- timestamping,
- authorship,
- discoverability,
- revocation,
- lineage.

Truth, validity, or acceptability depends on schema-specific validation, evidence quality, verification processes, and governance context.

Claims and snapshots may both be attested or anchored. A claim attestation is a portable assertion-level commitment. A snapshot attestation is a bounded evidence-set or checkpoint commitment.

### 6.5 Substrate Neutrality Is a Product Principle

The Claims Engine should not require a single blockchain, registry, storage layer, or attestation network.

It should produce portable semantic claim, snapshot, and validation artifacts that can be used across multiple ecosystems. Regen `x/data`, EAS, IPFS, Arweave, institutional repositories, local stores, and similar systems are possible backends, not core dependencies.

### 6.6 Validation Is Additive

Validation should be represented as additional, schema-specific claims about claims, snapshots, or subgraphs.

The engine should not mutate original claims into globally approved or rejected states. Instead, validation, endorsement, dispute, contradiction, and review results should become new claims in the graph.

Trust modules, scoring modules, reputation systems, and insight layers may later interpret these validation claims, but they are deferred and are not part of the current claim graph core.

### 6.7 Base Relationship Vocabulary Is Likely Needed

The Claims Engine will likely need a small, generic base vocabulary for claim-to-claim relationships so that graph navigation and interoperability are possible.

This vocabulary should remain abstract and extensible, not domain-specific. Users should be able to extend it with their own RDF vocabularies.

*Further research needed: define the minimal base claim relationship vocabulary.*

## 7. Deferred Concerns

The following are intentionally deferred:

- marketplace design,
- carbon registry design,
- asset issuance,
- token economics,
- credit lifecycle management,
- credit retirement,
- payment workflows,
- buyer/seller experiences,
- advanced trust/reputation scoring,
- verifier marketplaces,
- formal contradiction calculus,
- global ontology governance,
- full downstream productization.

These may become important later, but they should not distract from the current goal: building the claim graph core.

## 8. Product Surface

The Claims Engine should be API/library-first for v1, because the primary user is technical integrators and developers.

However, the product should include a lightweight graph viewer or explorer for inspection, debugging, and demonstration. The viewer should help people see claims, relationships, snapshots, and validation claims, but it should not become the main product surface yet.

Agreed direction:

1. Core API/library/CLI first.
2. Lightweight graph explorer second.
3. No full end-user workflow UI yet.

## 9. Store Direction

The Claims Engine should be store-agnostic at the core, likely organized as a multi-crate system.

However, v1 should include one concrete graph store implementation so the MVP is usable and demonstrable.

The concrete graph store has not been selected yet. An RDF-native store is likely appropriate, but this requires deeper research before choosing.

*Further research needed: choose the first concrete RDF graph store implementation.*

## 10. Next Deliverable

The next deliverable should be a **product MVP definition**.

This should define what the minimum viable Claims Engine product must prove, who it serves, what capabilities are included, what is explicitly excluded, and what open questions remain for later research.

## 11. Current North Star

The near-term north star is:

> Build the abstract claim graph core: claims enter the system, get linked into a semantic graph, can be snapshotted, and later validation results can become new claims in the graph.

In shorthand:

> Claim graph in, claim graph out.
