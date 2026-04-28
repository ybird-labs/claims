# Claims Engine

The Claims Engine is an impact accounting framework for creating, validating, and composing verifiable environmental claims.

It uses linked data, graph-based evidence, attestations, and blockchain anchoring to move impact accounting beyond static documents and into transparent, replayable claim networks.

## Core Idea

Claims are modular building blocks.

A claim may describe evidence, measurement, validation, endorsement, contradiction, provenance, or a downstream product. Claims can compose into higher-order claims, making the system recursive: claims all the way down.

## Layer Model

```text
L0 -> L1 -> Ln
```

- **L0:** open linked-data claim graph
- **Snapshots:** stable views of L0 at a specific time
- **L1:** schema validation over snapshots
- **Ln:** assets, certificates, contracts, governance actions, and triggers derived from validated claims

## Why It Exists

Impact data is fragmented, document-centric, and expensive to verify.

The Claims Engine provides a shared substrate where evidence can be authored once, linked semantically, independently verified, and reused across many programs, schemas, and products.

## Design Principles

- Semantic data over static PDFs
- Open graph at the base layer
- Schema-specific validation above the base layer
- Pluggable verification and authority models
- Portable evidence across programs
- Blockchain anchoring for integrity, not data storage
- Recursion from raw evidence to products and governance actions
