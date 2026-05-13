# Claims Engine — Product MVP Definition

Date: 2026-04-25

## MVP Summary

The Claims Engine MVP is a developer-facing system for creating an abstract semantic claim graph from structured JSON-LD claims, inspecting relationships between claims, and creating stable snapshots of selected claims with derivable fingerprints.

In one sentence:

> The MVP demonstrates the Claims Engine can act as abstract claim graph infrastructure: developers can submit provenance-bearing JSON-LD claims, build a linked semantic graph, inspect claim relationships, and create stable snapshots of claim bundles whose fingerprints can be compared against independent commitments later.

## Primary User

The primary MVP user is a **technical integrator / developer**.

This user wants to:

- submit structured claims,
- build or integrate with a claim graph,
- inspect relationships between claims,
- create stable evidence bundles/snapshots,
- reference those snapshots from other systems later.

The MVP is not optimized first for project operators, marketplace participants, carbon registry users, or non-technical workflow users.

## MVP Goal

The MVP must demonstrate that the Claims Engine can support the core loop:

```text
JSON-LD claims
→ L0 claim graph
→ inspectable claim relationships
→ scoped snapshot / evidence bundle
→ stable fingerprint that can be compared with a stored, anchored, or attested commitment to detect changes
```

This MVP focuses on **claim graph + snapshot**.

It does not need to fully implement validation, external attestation, trust scoring, registry integration, marketplace workflows, or downstream asset issuance.

## MVP Capabilities

### 1. Submit Claims

Developers can submit claims using a minimal Claims Engine claim envelope with flexible JSON-LD assertion content.

The envelope exists to make a claim operational. It should include the minimum provenance needed for the graph to answer:

> Who said what, and when?

Required claim metadata:

- claim id,
- JSON-LD context / semantic content,
- claimant or author,
- timestamp.

Optional but first-class metadata may include:

- evidence references,
- method/procedure,
- signature,
- schema reference,
- location,
- related claims,
- confidence/uncertainty,
- license/access policy.

### 2. Build an L0 Claim Graph

Submitted claims are represented as linked semantic data and stored/indexed as an L0 claim graph.

The graph should allow claims to reference and relate to other claims.

The Claims Engine does not decide whether a claim is true, approved, or valid. L0 means:

> This claim exists in the graph with provenance.

### 3. Inspect Claims and Relationships

Developers can inspect the graph enough to verify that claims are linked.

At minimum, the MVP should support:

- listing claims,
- fetching a claim by id,
- inspecting claim metadata,
- inspecting claim content,
- seeing outgoing relationships from a claim,
- seeing incoming relationships to a claim.

A lightweight graph explorer may exist for inspection, debugging, and demonstration, but the core MVP is API/library-first.

### 4. Support Single and Batch Claim Submission

The MVP should support both:

- single-claim submission,
- batch import of multiple related claims.

Batch import is useful for demonstrating that a meaningful graph can be constructed from multiple linked claims.

### 5. Create Snapshots

Developers can create a scoped snapshot from selected claims.

A snapshot is a narrow evidence bundle:

> A stable, bounded set of claim references from the graph, which other claims can later evaluate, endorse, or dispute.

Purpose, timestamping, publication, commitment, and attestation belong to surrounding records rather than the narrow Snapshot value.

For MVP purposes, the exact snapshot scoping mechanism can remain simple and be refined later.

### 6. Fingerprint Snapshots

Each snapshot should have a stable fingerprint/hash digest derived from its canonical snapshot value. The digest does not by itself prove truth, validation, authorship, or external acceptance; it can be compared with a stored, anchored, attested, signed, package-level, or immutable/admission commitment to detect differences.

The MVP should demonstrate that the same snapshot content produces the same fingerprint, and modified content produces a different fingerprint.

The MVP should compute claim and snapshot fingerprints, but it does not need to implement full external attestation or anchoring.

The model should remain ready for later `ClaimAttestation` and `SnapshotAttestation` concepts without requiring them in the MVP.

Exact canonicalization, hashing, attestation, anchoring, and inclusion-proof mechanics require further research.

## MVP Non-Goals

The MVP does not need to solve:

- marketplace design,
- carbon registry workflows,
- credit issuance,
- tokenization,
- payments,
- asset trading,
- credit retirement,
- buyer/seller flows,
- full downstream financial products,
- trust scoring,
- reputation systems,
- verifier marketplaces,
- formal contradiction calculus,
- human verifier workflow tooling,
- specific ecological methodologies,
- domain-specific validation schemas,
- external attestation or anchoring,
- final RDF store selection,
- final base relationship vocabulary,
- final snapshot inclusion-proof design,
- full semantic query language,
- non-technical user workflows.

## Validation in the MVP

Validation is important to the longer-term Claims Engine vision, but it is not the center of this MVP.

The MVP should remain compatible with later validation by preserving the idea that:

- validation is additive,
- validation results are claims about claims, snapshots, or subgraphs,
- validation should eventually operate against user-defined RDF-compatible schemas,
- the engine should not hard-code a specific domain schema or methodology.

For MVP, it is acceptable to defer the full validation mechanism. Validation remains separate from attestation and anchoring: validation interprets or evaluates claims, while attestation/anchoring records or commits to claim or snapshot state.

## Product Surface

The MVP should be API/library-first.

The primary surface should serve developers and integrators.

A lightweight graph viewer or explorer is useful as secondary tooling for:

- debugging,
- inspecting nodes and edges,
- demonstrating the claim graph concept,
- showing snapshots.

The graph explorer should not become the main product surface yet.

## Store Direction

The Claims Engine should be store-agnostic at the core.

The product is expected to be organized in a modular/multi-crate way, where the core claim model is not defined by a specific database.

However, the MVP should include one concrete graph store implementation so the product can be used and demonstrated.

The concrete graph store is not decided yet. An RDF-native store is likely appropriate, but further research is required.

## MVP Success Criteria

The MVP is successful if a developer can:

1. submit one or more provenance-bearing JSON-LD claims;
2. see those claims represented in an L0 semantic claim graph;
3. inspect claims and immediate relationships between claims;
4. create a scoped snapshot/evidence bundle from claims;
5. compute stable claim and snapshot fingerprints/hashes;
6. compare a snapshot fingerprint against a stored, anchored, or attested commitment to detect changes;
7. understand that validation, external attestation, and anchoring remain deferred and separate from the MVP claim graph core;
8. understand that the engine is abstract claim graph infrastructure, not a marketplace, registry, or carbon-credit product.

## MVP North Star

> Claim graph first. Snapshot second. Validation later.

Or:

> Build the abstract claim graph core before building downstream interpretation, trust, registry, or market systems.
