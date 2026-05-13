# Claims Engine

The Claims Engine is an impact accounting framework for creating and composing provenance-bearing claim graphs.

It uses linked data, graph-based evidence, snapshot/evidence-set references, and optional commitments, attestations, or anchors to move impact accounting beyond static documents and into transparent, replayable claim networks.

## Core Idea

Claims are modular building blocks.

A claim may describe evidence, measurement, validation, endorsement, contradiction, provenance, or a downstream product. Claims can compose into higher-order claims, making the system recursive: claims all the way down.

## Layer Model

```text
L0 -> L1 -> Ln
```

- **L0:** open linked-data claim graph with snapshot and evidence-set references
- **Snapshots:** stable bounded evidence sets of claim references
- **L1:** schema and protocol-based validation, generally over snapshots but also possible over claims or subgraphs
- **Ln:** assets, certificates, contracts, governance actions, reports, and triggers derived from claims or validation results

## Why It Exists

Impact data is fragmented, document-centric, and expensive to verify.

The Claims Engine provides a shared substrate where evidence can be authored once, linked semantically, independently validated, committed to, attested, and reused across many programs, schemas, and products.

## Design Principles

- Semantic data over static PDFs
- Open graph at the base layer
- Schema-specific validation above the base layer
- Pluggable verification and authority models
- Portable evidence across programs
- Commitments, anchors, and attestations for integrity, timestamping, and portability, not truth or validation by themselves
- Recursion from raw evidence to products and governance actions


## Acknowledgements

This project is based on Sams and Austin's Claims Engine Paper.
