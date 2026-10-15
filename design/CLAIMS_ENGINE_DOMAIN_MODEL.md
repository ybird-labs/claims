# Claims Engine Domain Model — Current Design

Date: 2026-09-12 (revises 2026-09-10 — reference form made
protocol-fixed and origin-free, engine namespace deleted; see §19)

This document captures the current abstract domain model for the Claims Engine.
It is a design-understanding document, not an implementation plan.

It supersedes the 2026-07-02 model, incorporating the learnings of the
throwaway spike (`spike/`, 2026-07) that implemented that model end-to-end
and stress-tested it against a live carbon-credit registration review.
The spike is not in the main branch; it is preserved at git tag
`spike-archive-2026-09`, and `spike/` paths in this document refer to that tag.
Section 19 lists the deltas from 2026-07-02; earlier deltas live in the
superseded revisions (git history). It preserves a two-source basis: this
current domain model and `design/original_paper.md`.

## 1. Core premise

The Claims Engine stores immutable, content-addressed semantic claims and
lets independent parties judge them against schemas.

The engine defines what a claim *is*. Users define what specific claims
*mean*.

Layer model:

```text
L0 -> L1 -> L2
```

- **L0 — raw claim record.** Admits, content-addresses, and stores immutable
  claims. Enforces only the base claim schema (the structural floor).
  Projects the open linked-data claim graph.
- **L1 — schema validation.** Evaluates claims against user-defined
  claim-type schemas. Validation is a judgment, not an admission gate.
  Outcomes are recorded as validation claims, stored back into L0.
- **L2 — derived products.** Assets, certificates, contracts, governance
  actions, reports, and triggers derived from claims and validation claims.

## 2. Epistemics: the exclusion rule

(Revised 2026-09-10; previously "witnessed vs asserted".)

Submission facts — who submitted, when, exact bytes — are never part of
the Claim value; anything that matters to a statement's identity or
meaning is asserted content, shaped by claim-type schemas and judged at
L1. The engine is transparent machinery with no voice of its own: proof
happens only through attestation (§15); everything else is just data.

## 3. Claim

A **Claim** is:

> An immutable, content-addressed semantic statement that declares the
> schemas it claims to conform to.

Structurally:

```text
Claim value =
  canonical declared schema references
  canonical asserted semantic content

derived from the Claim value:
  canonical Claim fingerprint
  canonical ClaimIRI
```

A Claim is not:

- raw submitted JSON-LD,
- an API payload,
- an envelope alone,
- a database row,
- an RDF parser object,
- a transport message,
- a graph index entry,
- a provenance record — attribution, when meaningful, is inside content.

## 4. Asserted semantic content

A Claim's asserted content is:

> A canonical RDF-compatible semantic value.

For equality, fingerprinting, and projection, that value has:

> A canonical RDF dataset representation.

The Claim is semantic, not syntactic. Claim content is not raw JSON-LD text,
input formatting, transport envelope, parser-specific representation, or
database encoding.

Settled invariants:

```text
Claim asserted content must be non-empty.
Claim asserted content has a canonical RDF dataset representation.
Canonicalization profile: RDFC-1.0 with SHA-256 blank-node hashing;
the canonical form is UTF-8 canonical N-Quads.
Claim content is single-graph: it canonicalizes entirely into the
default graph. Content containing named graphs is rejected at admission.
```

The single-graph restriction exists because the projection (§17) assigns
each Claim's content to a named graph keyed by its ClaimIRI; content that
carried its own named graphs would be silently flattened. Rejecting it at
the floor keeps projection lossless, and the restriction can be relaxed
later without breaking any already-minted identity (rejecting less is
backward-compatible).

Submitted material is separate from Claim content:

```text
submitted material = what arrived
claim content = accepted canonical semantic value
```

## 5. Schema declaration

Every Claim declares at least one claim-type schema version it claims to
conform to:

```text
declared schema references =
  non-empty duplicate-free set of canonical SchemaVersionIris
```

The declaration means "this claim asserts conformance." It is declared at
L0 and judged at L1. L0 does not check conformance.

The declaration is part of the Claim value and therefore part of the
fingerprint preimage. Its canonical form is a sorted duplicate-free set.

The declaration lives in the envelope rather than inside asserted content
because content addressing makes in-content self-reference circular: content
cannot contain the ClaimIRI that is derived from that content. This is
settled (2026-07-15): the self-referential blank-node alternative is
rejected; the spike proved the envelope form fingerprint-clean end-to-end.

> **Superseded (2026-10-15).** The declaration now lives inside the content
> as `dcterms:conformsTo` on exactly one blank claim node; see
> `SPIKE_LEARNINGS_AND_WORK_PLAN.md` §3. The circularity above holds only
> when the claim node is the ClaimIRI. A blank node is fixed before
> hashing, so nothing circular remains.

## 6. Schemas

There are two levels of schema:

### 6.1 Base claim schema (engine-defined)

The structural floor every Claim must meet to exist at all. It is the L0
admission contract:

```text
content parses as a well-formed canonical RDF dataset
content is non-empty
content is single-graph: nothing outside the default graph
at least one declared schema version reference, each a well-formed IRI
the claim is immutable once admitted
```

### 6.2 Claim-type schemas (user-defined)

Users define what their claims look like: entities (a Verifier and the data
it carries), predicates (verified), and payload shapes (tons of carbon
stored, units, method).

Current direction for authoring: **LinkML** as the single schema source,
compiled to the artifacts each layer needs:

```text
LinkML schema (user-authored, one source of truth)
  -> JSON Schema      structural validation, good error messages
  -> JSON-LD @context the JSON -> RDF mapping, generated not hand-written
  -> SHACL            graph-level validation over the canonical form
```

Registered schema versions are immutable artifacts with canonical
`SchemaVersionIri`s. A schema change is a new version with a new IRI;
already-admitted claims keep pointing at the version they declared.

Schema registrations are a side registry of immutable versions, not Claims
(settled 2026-07-15). The recursive form — a registration claim citing the
artifact fingerprints — stays possible later as a purely additive layer;
nothing in this choice forecloses it.

## 7. Claim identity: content addressing

Claim identity is derived from the Claim value and nothing else:

```text
Claim fingerprint = digest over the canonical Claim value
ClaimIRI = deterministic function of the fingerprint alone,
           in the protocol-fixed, origin-free reference form
```

Therefore:

```text
same canonical Claim value
= same fingerprint
= same ClaimIRI
= same Claim
```

Settled invariant:

```text
ClaimIRI -> exactly one immutable Claim
canonical Claim value -> exactly one ClaimIRI
```

Admission is idempotent: submitting an already-admitted Claim value yields
the existing Claim. There is no duplicate-claim error at the domain level;
deduplication is content addressing itself and requires no records.

A local storage surrogate key (previously `ClaimId` as domain identity) is
an infrastructure concern, not part of the domain model.

Fingerprint construction (settled 2026-07-15) and reference form
(settled 2026-09-12):

```text
fingerprint preimage = versioned fingerprint-suite tag
                       + canonical sorted declared schema references
                       + canonical content
digest suite = SHA-256, lowercase hex
ClaimIRI    = claims:claim:rdfc10-sha256:<64 lowercase hex fingerprint>
SnapshotIRI = claims:snapshot:rdfc10-sha256:<64 lowercase hex fingerprint>
```

Every fingerprint preimage begins with a versioned suite identifier, so a
later change of digest, canonicalization profile, or preimage layout can
never collide with fingerprints minted under an older suite.

Engine-minted names use one protocol-fixed, origin-free reference form
under a protocol-owned URI scheme (provisional registration per RFC 7595
is planned, not yet done). No deployment value participates in identity;
there is no per-instance engine namespace. Each name has exactly one
canonical byte-form — lowercase everywhere, no aliases; the name's
pipeline segment (`rdfc10-sha256`) maps to the frozen v1 fingerprint
suites, with the mapping defined in the protocol spec
(`design/CLAIMS_PROTOCOL.md` §5.3). Resolution is an additive,
non-identity HTTP gateway convention (spec §5.4): any host may run a
gateway, none is special, byte-level verification is mandatory at every
gateway, and names never promise persistence — identity never depends on
any gateway.

## 8. The identity rule

> If it matters to identity, it goes in content.

The engine's identity law is canonical Claim value alone. The previous
model's rule (content + assertor + asserted_at) is gone; there is no
engine-level notion of "same statement asserted by someone else" or "same
statement asserted twice."

If a user's domain needs repeated assertions to be distinct Claims, the
distinguishing data — an assertion timestamp, an occasion identifier — must
be part of asserted content, governed by their claim-type schema.

One consequence deserves a warning, because it is an easy category error
(surfaced by the carbon stress test): normative rule versions — which
rulebook a judgment was made under — govern what an assertion *means*, so
they are content; the same judgment under two rule versions is two distinct
Claims. Claim-type schema versions govern payload *shape* and live in the
envelope declaration. They are different things and must not be conflated.

## 9. Provenance is content

Semantic provenance — who verified, who measured, when the assertion was
made — is user-space content, shaped by claim-type schemas.

The engine recommends, but does not mandate, standard vocabularies (e.g.
PROV-O: `prov:wasAttributedTo`, `prov:generatedAtTime`).

The three timestamps:

```text
event time      when the carbon was stored          content (user schema)
assertion time  when the verifier signed off        content (user schema)
ingestion time  when L0 accepted the claim          operational log (infrastructure)
```

External/source identifiers decompose the same way (settled 2026-07-15,
exercised against real registry data):

```text
entity identifiers (farm key, plot id)   content, as subject IRIs
retrieval coordinates (row, file, API)   operational log (infrastructure)
richer origin semantics                  optional user-space PROV content
```

## 10. Submitted material and authoring formats

Users author claims in JSON (JSON-LD). The canonical, identified,
fingerprinted L0 form is the RDF dataset derived from it:

```text
authoring/transport form = JSON-LD (or other RDF serializations)
authoritative form = canonical RDF dataset
```

JSON-LD contexts are generated from registered schemas and pinned; the
engine never resolves remote contexts at ingestion.

Submitted material is distinct from Claim content and has its own
fingerprint over the exact received bytes — pure data, citable by
verdicts (`judged_material`, §13). Whether and where such bytes are kept
is an operator concern outside the model:

```text
submitted material fingerprint = exact received bytes
Claim fingerprint = digest over the canonical Claim value
```

## 11. (Removed 2026-09-10)

Formerly "Ingestion audit record". Logging, byte retention, and how a
validator obtains authored bytes are operator concerns outside the model.
The exclusion rule lives in §2 and invariant 7; proof lives in §15.

## 12. Claim acceptance and durable admission

Submitted material becomes a Claim only after durable admission into the
authoritative claim record.

Admission requires exactly the base claim schema floor (§6.1) plus
derivation of the fingerprint and ClaimIRI. It does not require:

```text
L1 conformance to declared schemas
any attribution or timestamp inside content
indexing in the L0 graph projection
inclusion in a Snapshot
external publication or anchoring
```

**Durable admission** means the engine recognizes the object as an accepted
immutable Claim in its authoritative claim record. After durable admission,
the Claim is immutable. Before it, the material is submitted/candidate
material, not a Claim.

Declared schema versions may dangle at admission (settled 2026-07-15): only
IRI well-formedness is checked, so schemas may arrive after data and old
data can be validated against later schemas. The counterpart guard lives at
L1, which refuses to *judge* against an unregistered schema version (§13).

## 13. L1 validation

Validation is a judgment about an existing Claim, not a write-path gate.

A validation run evaluates one Claim against one declared (or any other)
schema version and records the outcome as a **validation claim** — an
ordinary Claim whose content asserts (shape settled 2026-07-15):

```text
ValidationResult =
  validator        IRI of the judging service
  target_claim     ClaimIRI judged
  schema_version   SchemaVersionIri judged against
  outcome          conforms | violations
  violations       count (optionally with detail)
  judged_material  submitted-material fingerprint; present when the check
                   ran over an authored form, not the canonical dataset
  validated_at     assertion time (content, like any other)
```

Validation claims conform to an engine-published claim-type schema, using
the same mechanism as user schemas.

Consequences:

```text
a Claim can exist unvalidated, or failing validation
multiple schemas can judge the same Claim
disagreeing validators coexist as competing claims
schema updates trigger re-validation as new claims, not mutation
consumer trust = claims + the validation claims the consumer accepts
```

Because validators are off the write path, they need not share the engine's
implementation language or process. A validation service may use native
LinkML/SHACL tooling and write validation claims back through the normal
submission path.

Validation may be phased, with one rule (settled 2026-07-15): the
authoritative conformance judgment is defined over the canonical dataset
(SHACL over the canonical graph). A structural check over an authored form
(JSON Schema) is submission-scoped — admission is idempotent, so one Claim
may accumulate many differently-authored submissions, and a verdict over one
authored form says nothing about the others. Such a verdict must cite the
submitted-material fingerprint it judged via `judged_material`.

The engine's verdict vocabulary stays deliberately minimal and mechanical:
`conforms | violations`. Richer judgment vocabularies — e.g. the tri-state
`satisfied | not_satisfied | unclear` outcomes of a human review weighing
multiple cited evidence claims — are user-space claim-type schemas, layered
on top as ordinary claims and judged by unchanged L1 machinery.

## 14. Snapshot

Unchanged from the previous model except one correction to the fingerprint
definition (below).

A **Snapshot** is:

> A stable named selection of immutable Claim references.

```text
Snapshot =
  canonical SnapshotIRI
  non-empty duplicate-free unordered set of canonical ClaimIRIs
  derivable canonical Snapshot fingerprint
```

Snapshot membership is by canonical ClaimIRI. Order has no meaning;
duplicates are not meaningful. For fingerprinting, membership is represented
as a canonical sorted set of ClaimIRIs.

Snapshot uniqueness is determined by canonical membership:

```text
same canonical membership set = same Snapshot = same SnapshotIRI
```

The Snapshot fingerprint is computed over the canonical sorted membership
set alone, under its own versioned fingerprint suite; the SnapshotIRI is
derived from that fingerprint in the same protocol-fixed, origin-free
reference form as ClaimIRIs (§7) (settled 2026-07-15 — the previous wording had the fingerprint
covering the SnapshotIRI, which is circular once the IRI derives from the
fingerprint). The fingerprint excludes itself, the SnapshotIRI, full Claim
contents, and Claim fingerprints. The broader paper concept of a
snapshot/checkpoint maps to Domain Snapshot +
SnapshotCommitment/SnapshotAttestation.

## 15. Commitments, anchors, and attestations

Unchanged from the previous model.

```text
fingerprint = what is committed to
commitment/anchor/attestation = where, by whom, and when it was committed
validation = schema/process judgment (now recorded as validation claims)
```

Fingerprints are recomputable digests over canonical values. They support
trust or integrity only when compared to independent commitments, anchors,
attestations, or signatures. Engine records prove nothing to anyone else;
immutability gains adversarial weight through attestation claims, not
through storage.

ClaimAttestation provides portable assertion-level commitment.
SnapshotAttestation provides bounded evidence-set/checkpoint commitment.
Neither is validation.

## 16. Relationships

Unchanged from the previous model. There is no separate `Relationship`
primitive. A Claim can assert relations about anything addressable: Claims,
Snapshots, schema versions, people, organizations, documents, events,
external IRIs. Relationships are asserted semantic content.

Claims citing Claims by ClaimIRI is a load-bearing pattern, not an edge case
(carbon stress test): judgment claims cite the evidence claims they weigh;
derivation claims carry `derived_from` input ClaimIRIs plus the rule
identity, turning provenance into walkable structure instead of prose notes.
Trust composes through these chains — a single projection query can follow
verdicts to judgments to cited evidence to that evidence's content.

## 17. L0 graph projections

Accepted Claims are the source of truth. The **L0 claim graph** is the
canonical semantic projection over accepted Claims. L0's graph does not own
Claims; Claims do not become real by being indexed.

Projection is conceptually layered:

```text
asserted-content projection:
  what Claims assert (including in-content provenance and
  validation-claim content); each Claim's canonical content in a named
  graph keyed by its ClaimIRI, blank nodes skolemized per claim
  (<ClaimIRI>#<canonical label>)

claim metadata projection (default graph):
  derivable facts about Claims and Snapshots
  (declared schemas, fingerprints, membership)
```

This two-layer named-graph layout is settled (2026-07-15): it is queryable
with standard SPARQL 1.1, composes trust with content in one query across
layers, and is rebuildable from the claim record at any time. The physical
store behind it is an infrastructure choice, never a source of truth.

These layers are separate but query-composable. Simple rule:

> Do not confuse the thing said with the record of receiving it.

## 18. Core invariants

```text
1. ClaimIRI is permanent once accepted and identifies the Claim itself.

2. Claim fingerprint and ClaimIRI are deterministically derived from the
   canonical Claim value and exclude themselves; every fingerprint
   preimage begins with its versioned suite tag.

3. Claim asserted content is non-empty, single-graph (nothing outside the
   default graph), and has a canonical RDF dataset representation.

4. Every Claim declares at least one schema version reference; the
   declaration is part of the Claim value.

5. The Claim value carries no engine-mandated attribution or assertion
   timestamp; such facts are asserted content.

6. Claim identity is the canonical Claim value alone; admission is
   idempotent.

7. No submission fact — submitter, receipt time, submitted bytes or
   their fingerprint — is ever part of the Claim value or the fingerprint
   preimage.

8. L1 conformance is never a precondition for L0 admission.

9. Validation outcomes are recorded as Claims; a verdict computed over an
   authored form cites the submitted-material fingerprint it judged.

10. Registered schema versions are immutable and canonically identified.

11. Snapshot membership is a non-empty, duplicate-free, unordered ClaimIRI
    set; the Snapshot fingerprint is computed over the canonical sorted
    membership alone; the SnapshotIRI derives from that fingerprint; same
    membership resolves to same SnapshotIRI.

12. Fingerprints provide trust/integrity only against independent
    commitments, anchors, attestations, or signatures.

13. The L0 graph is derived from accepted Claims, not the source of truth.

14. The engine is transparent machinery: it holds no privileged facts,
    and its records are never a basis for trust (§15).
```

## 19. Changes from the previous model (2026-07-02)

**2026-09-12 revision:** engine-minted names and in-content references
now use one protocol-fixed, origin-free reference form
(`claims:claim:rdfc10-sha256:<hex>`, `claims:snapshot:rdfc10-sha256:<hex>`,
§7); the per-instance engine namespace is deleted, not amended — no
deployment value participates in identity, and §20 item 2 (the
production namespace value) is resolved by this decision, not answered.
The validation-result vocabulary, which hung off the deleted namespace,
becomes protocol-fixed under the same scheme
(`claims:vocab:validation-result:1.0.0#<term>`, spec §8). Resolution is
an additive, non-identity gateway convention with mandatory byte-level
verification (spec §5.4). Preimage layouts are unchanged and never
inject the minted object's own IRI or a deployment namespace; declared
schema IRIs and authored content — including any cited ClaimIRIs —
remain hashed. The documented claim-vector fingerprints are unchanged
because their inputs reference no engine-minted IRI; the snapshot
fingerprint (member ClaimIRIs are in its preimage) and the spec's §8
example (its content cites a target ClaimIRI) changed and were
regenerated from the generator. This change fulfills §2: the engine has no voice of its own,
and now no name of its own either. Grounding:
`design/CONTENT_ADDRESSED_NAMING_ACADEMIC_REVIEW.md` (verdict:
origin-free identity, conditional on operated resolution; DyLDO decay;
trusty-URI precedent) and the ATProto parallel (no server in names; the
hash is the trust anchor; strongRef reduces to hash-only absent
mutability/ownership). RFC 6920 was rejected on semantics: its
verification model is hash-of-retrieved-bytes, ours is
hash-of-constructed-preimage.

**2026-09-10 revision:** the ingestion audit record is demoted from
domain object to infrastructure (§2, §11); the "engine can vouch for
witnessed facts" framing is withdrawn — the operator's own log carries no
adversarial weight, and dispute-grade timestamping is §15's attestation
layer; the metadata projection no longer carries submission records
(§17); §11 is removed outright; "admission records" is removed from the
trust-basis lists (§15, invariant 12, §21) — proof is only through
attestation, and attestation claims are what give immutability
adversarial weight. What remains domain: the exclusion rule (invariant 7)
alone; submitted-material fingerprints remain as pure data citable by
verdicts (§10, §13). Driver: re-examining what actually consumes
submission facts found nothing in the domain — deduplication was already
content addressing itself, and the engine is transparent machinery with
no voice of its own.

The list below records the earlier 2026-07-15 deltas from 2026-07-02.

Driven by the throwaway spike (`spike/`) that implemented the 2026-07-02
model end-to-end and stress-tested it against a live carbon-credit
registration review. The deltas from 2026-04-25 are listed in the
superseded revision (git history).

```text
1. Ratified from §20 (were provisional, now settled): RDFC-1.0 with
   SHA-256 blank-node hashing as the canonicalization profile; dangling
   schema declarations allowed at admission; envelope-level schema
   declaration; the validation-claim shape; SnapshotIRI derived from the
   membership fingerprint; the external/source identifier decomposition;
   the two-layer named-graph projection layout.

2. Claim content restricted to single-graph: content canonicalizing to
   anything outside the default graph is rejected at the L0 floor
   (was: any RDF dataset; the projection would have silently flattened
   nested named graphs).

3. Fingerprint construction settled: versioned suite-tagged preimages,
   SHA-256 lowercase hex, IRIs minted as
   <namespace>/{claim,snapshot}/sha256/<hex>. The engine namespace is a
   validated configuration value; its production value is deliberately
   still open (§20).

4. Snapshot fingerprint corrected: computed over the canonical sorted
   membership alone (was: "covers the canonical SnapshotIRI", circular
   once the IRI derives from the fingerprint).

5. Authored-form validation verdicts are submission-scoped: they cite
   the submitted-material fingerprint they judged (judged_material in
   the validation-claim shape). The authoritative conformance judgment
   is over the canonical dataset. (Surfaced by idempotent resubmission:
   one Claim, many authored forms.)

6. Schema registrations settled as a side registry of immutable
   versions, not Claims; the recursive form remains possible as a
   purely additive later layer (was §20 item 4).

7. Codified from the carbon stress test: the engine verdict vocabulary
   stays binary while richer judgment vocabularies (tri-state review
   outcomes, multi-evidence judgments) are user-space claim-type
   schemas; normative rule versions are content, not schema versions;
   Claims citing Claims (evidence lists, derivation chains) is a
   load-bearing pattern for trust composition. No new engine primitives
   were needed.
```

## 20. Remaining ambiguities / revisit later

```text
1. The timestamp-literal authoring profile within canonical content —
   more a schema-authoring convention than an engine rule.

2. (Resolved 2026-09-12: there is no production engine namespace value
   to fix — the per-instance namespace is deleted and names use the
   protocol-fixed, origin-free reference form; see §7 and §19.)

3. (Removed 2026-09-10: submission logging is an operator concern
   outside the model.)

4. Phase-2 validation specifics: SHACL over the canonical graph,
   reference resolution, and cross-claim constraints.
```

## 21. Compact definition

```text
A Claim is an immutable, content-addressed semantic statement. Its value is
a non-empty duplicate-free set of declared schema version references plus
non-empty single-graph canonical RDF-compatible asserted content. Its
fingerprint and ClaimIRI are derived from that value and nothing else.

Submission facts (who submitted, when, exact bytes) never enter the Claim
value. Everything anyone asserts — including who verified what and when —
is claim content, shaped by user-defined claim-type schemas and judged at
L1. The engine is transparent machinery; proof happens only through
attestation.

L0 admits and stores Claims that meet the base claim schema. L1 evaluates
Claims against declared claim-type schemas and records outcomes as
validation claims. L2 derives products from claims and validation claims.

A Snapshot is a stable named selection of immutable Claim references with a
derivable fingerprint. Fingerprints support trust only against independent
commitments, anchors, attestations, or signatures.

Relationships are not separate primitives; Claims may assert relations
about anything addressable, including Claims, Snapshots, and schemas.
```
