# Claims Engine - Detailed Working Definition

Date: 2026-04-25

This document captures the detailed working definition of the Claims Engine based on the current product discussion.

It is not a final technical specification. It is a shared product-level understanding of what the Claims Engine is, what it is not, and what the first MVP should prove.

## 1. Core Definition

The Claims Engine is an abstract graph system for creating, linking, storing, querying, snapshotting, and eventually validating structured claims.

It is best understood as:

> Claim graph infrastructure for impact accounting.

Or:

> A substrate-neutral engine for creating, linking, snapshotting, and validating semantic claim graphs.

Or:

> A system for turning distributed evidence and assertions into reusable, inspectable, and validatable claim graphs.

The domain of interest is regenerative/ecological impact accounting, but the Claims Engine itself should remain abstract enough to represent many types of claims.

## 2. What the Claims Engine Is Not

The Claims Engine is not currently:

- a marketplace,
- a carbon registry,
- a crediting system,
- an asset issuance platform,
- a tokenization system,
- a payment system,
- a credit retirement system,
- a buyer/seller workflow,
- a portfolio management system,
- a downstream financial product.

Those may become downstream applications later, but they are not part of the immediate Claims Engine scope.

The current boundary is:

> For now, the Claims Engine stops at producing claim graphs, snapshots, and referenceable claim artifacts. Anything that uses those claims to create credits, certificates, markets, payments, or governance actions is downstream and out of scope.

## 3. Product Focus

The current product focus is building **claim graphs**.

The first product is not a registry or marketplace. The first product is the graph itself.

The Claims Engine should help people and systems:

1. submit claims,
2. represent claims as semantic linked data,
3. connect claims to other claims,
4. inspect relationships between claims,
5. create stable evidence bundles/snapshots,
6. later validate snapshots, claims, or subgraphs through additional claims.

The guiding idea is:

> The engine is not the judge; it is the graph where assertions and validations can be made legible.

## 4. Primary User

The primary user for v1/MVP is a **technical integrator or developer**.

This user wants to integrate with claim graph infrastructure, not use a full end-user impact accounting workflow.

Secondary future users may include:

- schema/methodology authors,
- verifiers/reviewers,
- project operators,
- community organizations,
- downstream application developers.

But the MVP should optimize for developers first.

## 5. Core Claim Concept

### 5.1 The Claim Is the Core Object

The core object is not a credit, certificate, asset, report, or registry entry.

The core object is:

> A claim in a graph of related claims.

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
- a dispute,
- a verification result,
- a validation result,
- a snapshot,
- a schema-related assertion.

### 5.2 Operational Definition of a Claim

For v1, a claim is a **named assertion bundle**.

It is usually submitted as a JSON-LD document and canonicalized into RDF.

Internally, it can be understood as:

> A bounded set of semantic assertions with its own identity and provenance.

Triples are the atomic semantic statements inside a claim, but the operational claim object is a named bundle of triples.

This avoids making the product operate at the level of one RDF triple at a time, while preserving the semantic graph foundation.

### 5.3 Minimum Claim Metadata

Every claim should include minimum provenance metadata.

Required:

1. claim id,
2. JSON-LD context / semantic content,
3. assertion content,
4. claimant or author,
5. timestamp.

Optional but first-class:

- evidence references,
- method/procedure,
- signature,
- schema reference,
- location,
- related claims,
- confidence/uncertainty,
- license/access policy.

Principle:

> L0 should be open, but not provenance-free. Every claim should answer: who said what, and when?

## 6. Claim Input Format

The MVP input format should be:

> A minimal Claims Engine claim envelope with flexible JSON-LD assertion content inside.

The envelope makes the claim operational by carrying identity and provenance.

The JSON-LD content keeps the claim semantically flexible and domain-neutral.

The claim envelope should not force a specific ecological, carbon, biodiversity, grant, or compliance schema.

## 7. L0 Claim Graph

L0 is the main product focus.

L0 is the open, evolving graph of claims and relationships between claims.

It contains structured assertions about:

- observations,
- measurements,
- evidence,
- methods,
- actors,
- places,
- times,
- endorsements,
- contradictions,
- disputes,
- provenance,
- validation results.

L0 does not globally decide truth. It represents claims and relationships.

Presence in L0 means:

> This claim exists in the graph with provenance.

It does not mean:

> This claim is true, approved, verified, valid, or accepted by any authority.

## 8. Permissiveness of L0

The Claims Engine should be permissive about claims entering L0.

The engine should not reject claims because they are:

- controversial,
- contradictory,
- incomplete,
- low-confidence,
- later disputed,
- not accepted by a domain schema.

The engine should only require that a claim satisfy the basic operational/provenance envelope and be representable as semantic data.

Contradiction is not a bug. Contradiction is part of the model.

A graph may contain:

- claims,
- counterclaims,
- endorsements,
- disputes,
- failed validation results,
- superseded claims,
- minority interpretations.

## 9. Claims About Claims

The Claims Engine should support claims about claims.

This includes claims that:

- support another claim,
- contradict another claim,
- endorse another claim,
- dispute another claim,
- cite another claim,
- derive from another claim,
- validate another claim,
- invalidate another claim,
- supersede another claim,
- qualify another claim.

This preserves the principle:

> Claims all the way down.

## 10. Relationship Vocabulary

The Claims Engine will likely need a small, generic base vocabulary for claim-to-claim relationships so that graph navigation and interoperability are possible.

However, the exact vocabulary is not defined yet.

The vocabulary should be:

- abstract,
- minimal,
- extensible,
- not domain-specific,
- compatible with RDF/linked data,
- aligned with existing standards where useful.

Potential relationship concepts may include:

- supports,
- contradicts,
- endorses,
- disputes,
- validates,
- invalidates,
- derives from,
- cites,
- supersedes,
- has evidence,
- has target,
- has schema,
- has result.

These are candidates only, not final decisions.

*Further research needed: define the minimal base claim relationship vocabulary.*

## 11. Snapshots

A snapshot is a narrow domain evidence bundle:

> A stable, bounded set of claim references from the graph, which other claims can later validate, endorse, or dispute.

A snapshot is not an engine declaration of truth.

The broader paper-style checkpoint can add purpose, time, publication, commitment, or attestation records around that narrow snapshot. It is someone saying:

> This specific set of claims is the bundle I want to reference for this purpose at this time.

The engine helps create the snapshot artifact, but does not decide whether the bundle is complete, correct, valid, or authoritative.

A snapshot, narrowly defined, is the evidence bundle itself. A commitment, attestation, publication, or anchor of that snapshot is a separate artifact or event that can reference the snapshot and its fingerprint.

## 12. Snapshot Fingerprints

A snapshot should have a stable hash/fingerprint derivable from its canonical snapshot value: the SnapshotIRI and canonical membership set of ClaimIRIs.

The snapshot is both:

1. semantic - it defines which claims are in scope;
2. cryptographic - its canonical value has a stable digest that can be compared against a stored, attested, or anchored commitment to detect whether the bundle differs from that commitment.

The fingerprint itself does not prove truth, validity, authorship, absence of modification, or external acceptance. It gives the system and downstream substrates a stable digest to compare against independent commitments.

The exact technical mechanism is deferred.

Open research questions include:

- How should snapshots be canonicalized?
- Should we hash RDF canonical form, JSON-LD canonical form, or both?
- Should we use RDF Dataset Canonicalization, such as RDFC-1.0?
- Is the snapshot a named graph, RDF dataset, Merkle DAG, or content-addressed artifact?
- How do we prove inclusion of individual claims in a snapshot?
- How do we support selective disclosure?
- How should member claim contents be verified against their own claim fingerprints and independent commitments when needed?

*Further research needed: exact canonicalization, hashing, and inclusion-proof mechanics.*

## 13. Validation

Validation is important, but it is not the center of the MVP.

The long-term model is:

> A claim, snapshot, or subgraph can be evaluated against a user-defined RDF-compatible schema, and the result becomes another claim in the graph.

Validation is additive and schema/process-specific. It will often be an L1 process over snapshots or selected subgraphs, rather than a property of L0 presence itself.

Validation is separate from publishing, anchoring, or attesting. A snapshot can be anchored without being validated, and a validation result can itself later be published, anchored, or attested.

The Claims Engine should not hard-code a specific domain schema.

It should not decide:

- what makes a valid carbon claim,
- what makes a valid biodiversity claim,
- what makes a valid grant milestone,
- what makes a valid community endorsement,
- what makes a valid compliance claim.

Those rules are supplied by users, applications, schemas, or later modules.

The engine should support the general pattern:

```text
graph/snapshot + RDF-compatible schema -> validation result claim
```

## 14. Validation Is Additive

Validation should be represented as additional claims about claims, snapshots, or subgraphs.

The engine should not mutate original claims into globally approved or rejected states.

Instead:

```text
Claim A exists.
Claim B validates Claim A under Schema X.
Claim C disputes Claim A.
Claim D endorses Claim A.
Claim E says Claim A failed Schema Y.
```

All of these are claims in the graph.

This means:

- original claims remain intact,
- validation is plural,
- multiple validators can disagree,
- failed validation is useful information,
- validation can be schema-specific,
- validation can be time-bound,
- validation can be superseded.

Trust modules, scoring modules, reputation systems, and insight layers may later interpret these validation claims, but they are not part of the current claim graph core.

## 15. Validation Is Not Truth

Validation means a claim/snapshot/subgraph satisfies a specific schema or rule set.

It does not mean the system has determined universal truth.

Different schemas, authorities, communities, or institutions may validate the same graph differently.

The Claims Engine should record validation claims, not pronounce final validity.

## 16. Substrate Neutrality

The Claims Engine should be substrate-neutral.

It should not require a single blockchain, registry, storage layer, or attestation network.

Regen `x/data`, Ethereum attestations, EAS, IPFS, Arweave, institutional repositories, local stores, or other systems may all become possible publication, anchoring, or attestation backends.

At the product level:

> The Claims Engine creates canonical claim, snapshot, and validation artifacts. Those artifacts can then be published, stored, anchored, or attested through different substrates depending on context.

Claims and snapshots can both be published, stored, anchored, or attested. A snapshot commitment or attestation is not the same thing as the snapshot itself; it is a substrate-specific reference to, statement about, or commitment to that snapshot.

Storage, anchoring, attestation, and publication are not the same as truth or validation.

Publishing, storing, anchoring, or attesting a claim may provide:

- availability,
- integrity,
- timestamping,
- authorship,
- discoverability,
- revocation,
- lineage.

But it does not make the claim or snapshot true, valid, or accepted under a schema. Validation remains an additive graph process defined by specific schemas, methods, authorities, communities, or applications.

## 17. Store Direction

The Claims Engine should be store-agnostic at the core.

The product is expected to be organized in a modular/multi-crate way, where the core claim model is not defined by a specific database.

However, the MVP should include one concrete graph store implementation so the product can be used and demonstrated.

The concrete graph store is not decided yet.

An RDF-native store is likely appropriate, but this requires deeper research before choosing.

*Further research needed: choose the first concrete RDF graph store implementation.*

## 18. Product Surface

The Claims Engine should be API/library-first for v1/MVP.

The primary user is a technical integrator/developer, so the main product surface should be developer-oriented.

The product may include:

- API,
- library,
- CLI,
- lightweight graph explorer.

The lightweight graph explorer is useful for:

- inspection,
- debugging,
- demonstration,
- helping people see claims and relationships.

But the graph explorer should not become the main product yet.

No full non-technical end-user workflow UI is required for MVP.

## 19. MVP Definition

The MVP is:

> A developer-facing system for creating an abstract semantic claim graph from structured JSON-LD claims, inspecting relationships between claims, and creating stable snapshots of selected claims with derivable fingerprints and a clear separation from commitment/attestation.

The MVP must prove:

1. claims can be submitted as structured JSON-LD with minimum provenance metadata;
2. claims can be normalized into a semantic graph;
3. claims can reference and relate to other claims;
4. developers can inspect claims and their immediate relationships;
5. a set of claims can be bundled into a snapshot;
6. the snapshot has a derivable stable fingerprint/hash that can be compared against a stored, attested, or anchored commitment to detect differences;
7. the system remains abstract and does not depend on a specific domain, registry, marketplace, or carbon-credit workflow.

The MVP should support:

- single-claim submission,
- batch claim import,
- claim lookup,
- immediate relationship traversal,
- snapshot creation,
- snapshot fingerprinting,
- enough internal structure to later reference claim and snapshot fingerprints from commitment/attestation records.

The MVP does not need to fully solve validation, provide full external attestation, or choose a final external anchoring substrate.

## 20. MVP Non-Goals

The MVP does not need to solve:

- validation mechanics,
- trust scoring,
- reputation,
- marketplace flows,
- carbon registry flows,
- asset issuance,
- specific ecological methodologies,
- final RDF store selection,
- final base vocabulary,
- final snapshot inclusion-proof design,
- final commitment/attestation model,
- final external anchor strategy,
- full graph query language,
- non-technical user workflows.

## 21. Deferred Decisions and Research Backlog

The following require further research or later product decisions:

1. Exact snapshot canonicalization and hashing mechanism.
2. Inclusion proofs and selective disclosure for snapshots.
3. First concrete RDF graph store implementation.
4. Minimal base claim relationship vocabulary.
5. Exact RDF-compatible validation mechanism and supported schema formats.
6. MVP sample dataset/domain.
7. Commitment/attestation model for claims and snapshots.
8. Publication/anchoring/attestation backend strategy.
9. External anchor strategy and substrate selection criteria.
10. Long-term trust/reputation/scoring modules.
11. Lightweight graph explorer scope.
12. Full query language and graph traversal model.

## 22. Current North Star

The near-term north star is:

> Build the abstract claim graph core: claims enter the system, get linked into a semantic graph, can be snapshotted, and snapshot fingerprints can be compared against stored, attested, or anchored commitments to detect changes.

In shorthand:

> Claim graph first. Snapshot second. Validation later.

Or:

> Claim graph in, claim graph out.
