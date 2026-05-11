# Regen Claims Engine - An Evolved Impact Accounting Data Model v4

Author Details: Austin Wade Smith & Sam Bennetts

Date: 09/02/25

Version: 4

This draft is a WIP, for a Concept Note scheduled to be released Fall 2025. It builds on the work of Regen Registry, Regen Foundation, and the Framework WG.

Many sections are still raw and lack consistent formatting. For internal reference only.

## Abstract

The Regen Claims Engine is an impact accounting framework that holds the richness of semantic data of environmental actor networks while enabling progressive milestones and checkpoints to be codified and thus integrated into institutional workplans, budgets, and governance.

Leveraging semantic data structures, blockchains, and attestations systems, our framework structures Impact claims as modular building blocks to satisfy a wide range of impact accounting schemas useful in both the private and public sector.

Claims are composed of linked data graphs including measurement / evidence and verification / validation.

Adopting the L0 → L1 → Lₙ stack, claims are recursively composable: primitives at one layer serve as evidence for the next, aggregating into products and programmatic triggers—assets, certificates, and contract inputs.

Isolating verification / validation as a constituent component of claims definitions means a diversity of verification protocols may be used in our stack. This reflects an effort to evolve the industry of institutional validation and verification bodies (VVB's), which while helpful, prove to be exceedingly costly with the potential for perverse incentives of value capture at the verification phase undermining overall impact accounting program.

Leveraging blockchain anchoring, like the Regen data module (x/data), means lightweight anchors of integrity are stored on chain, with mappings to rich semantic data ontologies held off-chain.

## 1) Introduction

Efforts to uphold regenerative stewardship with living systems entail countless interactions across organisms and institutions.

Data related to human–environmental interaction must reflect the complex and networked nature of the systems and actions which they describe.

Systems of impact accounting engage in the challenge of codifying actions and outcomes within these complex systems so that they are legible to financial, technical, and political processes.

This comes at a time when verifiable ecological state and outcomes are essential for informed climate resilience strategies, risk underwriting, and natural asset evaluations.

Any robust system of impact accounting must reflect a rich graph of interactions between networks of human and non-human actors while simultaneously accommodating the structured schemas of accounting that institutions require.

The Claims Engine is an impact accounting framework that holds the semantic richness of actor networks while enabling progressive milestones and checkpoints to be integrated into institutional workplans, budgets, and governance.

We aim to move beyond "proof of PDF" toward transparent, replayable verification built from interoperable claims, so originators are enfranchised and verification is scoped to the claim.

Our goal is to create a system where a common data substrate is instrumental for the creation of composable and nested claims according to a wide range of impact schemas.

## 2) Context & Pain Points

There is a crisis in the integrity of impact accounting systems: natural systems are oversimplified, and perverse economic incentives alienate the communities and organizations responsible for originating regenerative impact.

Environmental data is fragmented across repositories and incompatible schemas. Workflows in traditional climate-finance institutions remain document-centric.

Verification and authority are centralized, producing slow and costly processes that obscure local knowledge and evolving evidence.

A better system invites open participation at L0 and provides portable, product-agnostic schema validation at L1, with the flexibility to bundle claims as components into higher order frameworks at Ln.

## 3) Core Premises

Modularity and recursion: everything is a claim; relations—support, provenance, endorsement, contradiction—carry meaning. Claims compose into claims; it's claims all the way down.

### 3.1 Layer Architecture

L0 - An open graph of linked claim data serving as the data substrate and source of distributed state.

Snapshots - claims about the state of a bounded view of the graph at a specific timestamp. In effect an attestation of graph state.

L1 - validation of graph snapshots according to product or contract schemas. Essentially claims of validity over the state of the graph at a particular time according to specified schemas through automated or participatory systems of validation and verification.

Lₙ - artifacts and actions—assets, contracts, or operational triggers—derived from validated L1 Claims.

## 4) Claim Graph — Layer 0

### 4.1 Why L0

Beyond "proof-of-PDF." Environmental claims still live mostly in static documents that demand manual review and offer little transparency. A living claim graph lets evidence, context, and verification stay connected and inspectable.

From credits to claims (wider surface). Carbon credits were a start; organizations now need to prove impact across many contexts—supply chains, grants, ESG, stewardship. An L0 graph lets the same observations serve multiple narratives without rework.

We seek to build a public substrate for environmental state and outcomes. If financial systems can settle transparent transactions, environmental systems deserve queryable, verifiable claims—a substrate others can build on.

### 4.2 RDF & Linked Data

RDF gives the graph its form. Information is expressed as triples—subject, predicate, object—so what used to be "metadata" becomes first-class, linkable knowledge.

The triplet structure corresponds to a node - edge - node geometry. Overlapping subjects or objects creates a graph or a network of claims.

Relational richness. Borrowing from the logic of semantic data, this approach creates a network of metadata. Claims of measurements, methods, locations, evaluators, and endorsements interrelate, supporting or contradicting one another. The result is a knowledge graph of impact, not a pile of files.

(More context to be provided on the opportunities of a semantic approach to claims graphs in the impact accounting spaces.)

### 4.3 Vocabulary & Ontologies

Open by default, bounded when warranted. L0 welcomes new predicates and local terms; closed or enumerated lexicons can be applied where a program demands them.

No chain-wide lock-in. When tighter control is needed, it is enforced above L0 via schema-specific shapes and rules, preserving adaptability at the base.

### 4.4 Linked Data

Familiar JSON, semantic inside. To create the semantic graph, contributors publish claims in more human-readable formats such JSON-LD: everyday JSON plus a @context that maps keys to IRIs. This keeps authoring approachable while producing proper linked data.

Contexts give lightweight shape. By choosing or composing contexts (ontologies/lexicons), submitters indicate which vocabulary they speak. L0 remains open; the shape implied by contexts foreshadows stricter shape validation at L1.

Interoperability between contributions to the graph, as well as between graphs relies on shared ontologies. Ontological mappings, potentially with AI assistance can streamline consistency across lexicons

### 4.5 Anchoring with Integrity

Regen's x/data module anchors canonical fingerprints (IRIs) for raw content or RDF graphs, recording authorship and time on-chain while content remains in appropriate stores.

Independent verification. Anyone can re-canonicalize and re-hash to confirm exactly what was asserted—integrity without centralization.

### 4.6 Where L0 lives

Hybrid placement. Hashes and IRIs are recorded on-chain to establish immutable references and timestamps.

The underlying images, datasets, and RDF graphs are stored off-chain in distributed or institutional repositories to keep content accessible and appropriately managed.

Accountability. Digital signatures tie submissions to authors while keeping data where it belongs.

### 4.7 What RDF unlocks

Semantic search & reuse. Claims become discoverable by meaning, not file names; monitor once, report anywhere/anyhow becomes real.

Cross-program portability. The same observations can satisfy multiple schemas (SDGs, GHG methods, EU Taxonomy, Hypercerts) without duplication.

Evidence-centric governance. Because links are explicit, communities and institutions can trace how a conclusion was reached and contest it with counter-claims—rigor with room for dissent.

## 5) Snapshots (stable views of L0 at time T)

### 5.1 The Basis of a Snapshot.

A snapshot is a signed, time-stamped statement about what a bounded view of the claim graph looked like at a particular moment. It does not freeze the graph; it fixes a reference point that others can verify and discuss.

How a snapshot is scoped. A snapshot refers to a view—for example, a named subgraph or a query that selects the relevant claims (methods, measurements, evaluations, endorsements). Clear scoping makes the checkpoint reproducible: anyone can rerun the view and check that the same claims are in scope.

Why snapshots matter. Institutions need milestones—checkpoints that funding, certification, or governance can rely on—without losing the richness of the underlying discourse. Snapshots provide that milestone while preserving links back to evidence.

How a snapshot becomes verifiable. Before anchoring, the selected view is canonicalized so that the same logical graph produces the same bytes. This supports independent re-verification and prevents disputes over formatting.

### 5.2 Complementary anchoring paths

To make snapshots portable across ecosystems while preserving a single source of truth, we propose two complementary anchoring paths. They bind the same snapshot to two forms of verifiability—one as a content fingerprint in Regen's data layer, the other as an attested statement in an attestation network.

Data-module path (Regen x/data). Anchor a canonical fingerprint of the view as a Regen IRI (a content hash recorded on-chain) that anyone can re-derive from the underlying graph.

Attestation path (EAS-style). Publish a thin attestation that binds the snapshot root/hash ↔ IRI, with optional parent_root (to show lineage) and a view_id (to name the scope). This adds attester identity, revocation/expiry options, and easy discovery.

### 5.3 Lineage and Selective Disclosure

Lineage without lock-in. Snapshots can reference prior snapshots to form a DAG over time (e.g., Q2 builds on Q1), but lineage is optional and policy-dependent.

Non-final by design. New evidence can arrive after a snapshot; that does not invalidate the checkpoint. It simply invites a later snapshot with a new scope or time, keeping progress legible without pretending the story is finished.

Selective disclosure. When only part of a snapshot must be shown, Merkle inclusion proofs over canonical statements allow a verifier to see "just enough" while maintaining linkage to the anchored whole. Today this is feasible off-chain; on-chain support can follow if warranted.

### 5.4 Feeding into Schema Validations

A schema at L1 points to a snapshot and asks: Does this checkpoint contain the claims and relations required by this program? The result can authorize products or trigger actions, while the snapshot ensures everyone references the same state.

## 6) Schema Validation — L1 (The "Claims Engine")

### 6.1 What L1 Does

Evaluate a snapshot against arbitrary schemas. L1 assesses a bounded, time-stamped snapshot (from §5) against whatever rule-set the context demands—carbon/ecocredit programs, bio-cultural stewardship frameworks, SDGs, EU Taxonomy, GHG program methods, Hypercerts v2.

### 6.2 Anatomy of a Claim (at L1)

An L1 claim = evidence + verification.

The evidence is the referenced snapshot and the verification states how it was checked.

Conventional Verification. Peer review / PGS, institutional or third-party audits, and programmatic or automated checks (sensors, remote sensing, ML), with room for stake/bonds/reputation where appropriate.

Verification schemas may be arbitrarily complex. Form a simple attestation to reputation based validation network to more complex forms of off-chain validation like Actively Validated Services (AVS's).

Automated validation (non-exclusive). It can combine deterministic queries over the snapshot (e.g., SPARQL), schema-specific shape/constraint checks where closure is needed, and assisted analysis in which tools propose matches and humans review and sign the verdict.

### 6.3 Approving or Scoring Claims

Outcomes. L1 produces explicit conformance—binary pass/fail or a graded score that expresses uncertainty—accompanied by a concise rationale and stable references to the snapshot IRI (and, where used, its attestation and lineage).

Quantifiable uncertainty may be indexed as reflection of how reinforced or not claims are on one another.

(Scoring / Ranking systems merits further elaboration)

### 6.4 Semantic Richness and Authority

Semantics & authority live at L1. Unit and measurement semantics are handled at the schema level.

Authority is pluggable per schema (attester sets, roles, revocation/expiry). This allows different programs to set distinct requirements on roles without imposing chain-wide constraints.

There is more to elaborate on here on the ways L1 schemas specify units, roles etc.

### 6.5 Cross-links and Flow (L0 → L1 → Lₙ)

Upstream dependency. Every L1 evaluation names the snapshot it assessed (IRI; optional attestation/lineage), preserving a verifiable bridge to L0.

Downstream use. L1 results are consumable by Lₙ: they can authorize products or trigger actions while ensuring everyone references the same state of the graph.

## 7) Lₙ — Products, Assets, Inputs & Triggers

### 7.1 What Lₙ Represents

Downstream artifacts and actions. After L1 validation, creators decide what to emit or activate. Lₙ is the layer where validated claims become concrete outputs—either assets that can be held and exchanged, or triggers that drive programmatic behavior.

### 7.2 Assets (issued from validated snapshots)

Credits and certificates. Mint ecological credits, attestations of compliance, or domain-specific certificates that reference the validated snapshot and (where used) its attestation/lineage.

Tokens and hypercerts. Create transferrable or soulbound tokens, including Hypercerts, that encode rights, acknowledgements, or allocations tied back to the same snapshot IRI.

Portability by design. Because each asset points to an identical, verifiable snapshot, different markets and registries can interpret the same evidence without duplication.

### 7.3 Triggers (programmatic actions driven by verified claims)

Funds and disbursements. Release or escrow funds when a snapshot passes a schema, enabling performance-based payouts and milestone-linked financing as "Living Covenants"

Roles, access, badges. Assign roles or badges, grant access to sensitive knowledge, or open governance permissions as the system recognizes verified states.

Interoperable orchestration. Triggers emit machine-readable signals that other systems can subscribe to (oracles, workflows, DAOs), keeping L1 results legible beyond a single platform.

### 7.4 The Oracle Function of L1

General-purpose oracle. L1 serves downstream systems with schema-verified claims, providing a consistent interface: what snapshot was checked, under which schema, by whom, and with what outcome.

### 7.5 Shared Reference, Fewer Failures

One snapshot, many outcomes. All Lₙ outputs and inputs reference the same snapshot (and, if present, its attestation and lineage), producing a unified audit trail and helping to resist double counting across programs.

### 7.6 Granularity & Composition

Coarse or fine. Validation can target broad, composite graphs or smaller, atomic L1 claims. Finer granularity improves auditability and makes complex outcomes easier to assemble from well-understood parts.

### 7.7 Claims All the Way Down

Lₙ outputs are themselves claims: they can return to L0 as new evidence or endorsements, be re-evaluated under other schemas at L1, and fuel subsequent cycles without losing provenance.

## THE FOLLOWING SECTIONS ARE STILL IN RAW FORM.

## 8) Interoperability with Existing Systems (Regen ecocredits as one adapter)

### 8.1 Ecocredit as a purpose-built L1 for crediting**

The Regen Ecocredit module is a specialized L1: it issues and manages ecological credits using CreditType (units/precision) and CreditClass (roles/issuance).

It binds evidence to a defined crediting workflow with clear authorities, denominations, and issuance rules.

### 8.2 Layer separation enables diversity

By separating L0 (claims) from L1 (validation), the Claims Engine keeps primitives in the open graph while allowing many L1 approaches.

Crediting is one L1 among several—not the template all programs must follow.

### 8.3 Same evidence, multiple schemas

A single snapshot can be evaluated against different schemas—SDGs, EU Taxonomy, bio-cultural stewardship, Hypercerts v2—without re-authoring evidence.

This makes "monitor once, report anywhere/anyhow" practical.

### 8.4 One common soil, many products**

Treating L0 as the shared substrate lets product applications grow from the same claims base.

Credits, certificates, attestations, governance gates, financing triggers, and research syntheses can all reference the same snapshot.

### 8.5 Ecocredits as an adapter, not a constraint

Use the Ecocredit module as one L1 adapter when credits are needed; choose other L1 schemas and product paths when they are not.

This avoids shaping L0 around any one product family.

### 8.6 Direct references preserve verifiability

Existing metadata IRI fields in ecocredit records can point directly to L0 snapshots (and, where used, their attestations/lineage). This strengthens auditability without changing ecocredit internals.

### 8.7 From policy to practice via chunked L1 claims

Large sustainability commitments can be decomposed into smaller L1 claims, each tied to a concrete snapshot and schema check.

Those atomic claims can be recomposed to evidence portfolio-level or policy-level achievements with clear traceability.

### 8.8 Why Lₙ is "N"

Downstream outputs and actions can chain indefinitely: an L1 result may produce an L₂ asset, which becomes evidence for an L₃ governance action, and so on.

We use Lₙ to acknowledge this open-ended recursion while preserving links back to originating snapshots.
