# Claims Engine Domain Model — Current Design

Date: 2026-07-02

This document captures the current abstract domain model for the Claims Engine.
It is a design-understanding document, not an implementation plan.

It supersedes the 2026-04-25 model. Section 19 lists the deltas from that
model. It preserves a two-source basis: this current domain model and
`design/original_paper.md`.

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

## 2. Epistemics: witnessed vs asserted

The engine distinguishes two kinds of fact and never mixes them:

```text
witnessed = what the engine observed itself
asserted  = what someone claims
```

Witnessed facts live in the ingestion audit record:

```text
submitter principal
submitted_at / accepted_at
exact submitted bytes and their fingerprint
```

Asserted facts live in claim content:

```text
everything else, including attribution
("verifier V verified ...") and assertion time
```

The engine can vouch for witnessed facts. It can never vouch for asserted
facts, so it does not bless attribution through mandated envelope fields.
From the engine's perspective, "who asserted this and when" is hearsay like
any other content — and, unlike the previous model's envelope provenance, it
is schema-validatable at L1.

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
```

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
cannot contain the ClaimIRI that is derived from that content. (A
self-referential blank-node form is a possible alternative; see §20.)

## 6. Schemas

There are two levels of schema:

### 6.1 Base claim schema (engine-defined)

The structural floor every Claim must meet to exist at all. It is the L0
admission contract:

```text
content parses as a well-formed canonical RDF dataset
content is non-empty
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

Whether schema registrations are themselves Claims is open (§20). It would
fit the recursive model and reuse L0's identity and immutability machinery.

## 7. Claim identity: content addressing

Claim identity is derived from the Claim value and nothing else:

```text
Claim fingerprint = digest over the canonical Claim value
ClaimIRI = deterministic function of the fingerprint under the engine namespace
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
the existing Claim plus a new submission record. There is no
duplicate-claim error at the domain level.

A local storage surrogate key (previously `ClaimId` as domain identity) is
an infrastructure concern, not part of the domain model.

## 8. The identity rule

> If it matters to identity, it goes in content.

The engine's identity law is canonical Claim value alone. The previous
model's rule (content + assertor + asserted_at) is gone; there is no
engine-level notion of "same statement asserted by someone else" or "same
statement asserted twice."

If a user's domain needs repeated assertions to be distinct Claims, the
distinguishing data — an assertion timestamp, an occasion identifier — must
be part of asserted content, governed by their claim-type schema.

## 9. Provenance is content

Semantic provenance — who verified, who measured, when the assertion was
made — is user-space content, shaped by claim-type schemas.

The engine recommends, but does not mandate, standard vocabularies (e.g.
PROV-O: `prov:wasAttributedTo`, `prov:generatedAtTime`).

The three timestamps:

```text
event time      when the carbon was stored          content (user schema)
assertion time  when the verifier signed off        content (user schema)
ingestion time  when L0 accepted the claim          audit record (witnessed)
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

Original submitted material is preserved as ingestion/audit evidence with
its own fingerprint:

```text
submitted material fingerprint = exact received bytes
Claim fingerprint = digest over the canonical Claim value
```

## 11. Ingestion audit record

Every submission produces a witnessed audit record, outside the Claim value:

```text
SubmissionRecord =
  SubmissionId
  submitter principal
  submitted_at / accepted_at
  submitted material (or reference) + submitted material fingerprint
  resulting ClaimIRI
```

The audit record is engine-witnessed operational truth. It is not part of
Claim identity, not part of the fingerprint preimage, and not asserted
content. One Claim may accumulate many submission records.

`accepted_at` records when this engine admitted the Claim. It is audit
metadata, not provenance and not content.

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

Whether a declared schema version must already be registered at admission,
or may dangle (schemas arriving after data), is an open policy question
(§20). The current lean is to allow dangling declarations, preserving the
ability to validate old data against new schemas.

## 13. L1 validation

Validation is a judgment about an existing Claim, not a write-path gate.

A validation run evaluates one Claim against one declared (or any other)
schema version and records the outcome as a **validation claim** — an
ordinary Claim whose content asserts something like:

```text
validator V evaluated claim C against schema version S
outcome: conforms | violations [...]
validated_at: ...
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

Validation may be phased: a structural JSON Schema check over the authored
form, then SHACL over the canonical graph (reference resolution, cross-claim
constraints).

## 14. Snapshot

Unchanged from the previous model.

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

The Snapshot fingerprint covers the canonical SnapshotIRI and the canonical
sorted membership set; it excludes itself, full Claim contents, and Claim
fingerprints. The broader paper concept of a snapshot/checkpoint maps to
Domain Snapshot + SnapshotCommitment/SnapshotAttestation.

## 15. Commitments, anchors, and attestations

Unchanged from the previous model.

```text
fingerprint = what is committed to
commitment/anchor/attestation = where, by whom, and when it was committed
validation = schema/process judgment (now recorded as validation claims)
```

Fingerprints are recomputable digests over canonical values. They support
trust or integrity only when compared to independent commitments, anchors,
attestations, signatures, or immutable admission records.

ClaimAttestation provides portable assertion-level commitment.
SnapshotAttestation provides bounded evidence-set/checkpoint commitment.
Neither is validation.

## 16. Relationships

Unchanged from the previous model. There is no separate `Relationship`
primitive. A Claim can assert relations about anything addressable: Claims,
Snapshots, schema versions, people, organizations, documents, events,
external IRIs. Relationships are asserted semantic content.

## 17. L0 graph projections

Accepted Claims are the source of truth. The **L0 claim graph** is the
canonical semantic projection over accepted Claims. L0's graph does not own
Claims; Claims do not become real by being indexed.

Projection is conceptually layered:

```text
asserted-content projection:
  what Claims assert (including in-content provenance and
  validation-claim content)

claim/audit metadata projection:
  witnessed facts about Claims and Snapshots
  (submission records, admission, membership)
```

These layers are separate but query-composable. Simple rule:

> Do not confuse the thing said with the record of receiving it.

## 18. Core invariants

```text
1. ClaimIRI is permanent once accepted and identifies the Claim itself.

2. Claim fingerprint and ClaimIRI are deterministically derived from the
   canonical Claim value and exclude themselves.

3. Claim asserted content is non-empty and has a canonical RDF dataset
   representation.

4. Every Claim declares at least one schema version reference; the
   declaration is part of the Claim value.

5. The Claim value carries no engine-mandated attribution or assertion
   timestamp; such facts are asserted content.

6. Claim identity is the canonical Claim value alone; admission is
   idempotent.

7. The ingestion audit record is engine-witnessed, lives outside the Claim
   value, and is excluded from the fingerprint preimage.

8. L1 conformance is never a precondition for L0 admission.

9. Validation outcomes are recorded as Claims.

10. Registered schema versions are immutable and canonically identified.

11. Snapshot membership is a non-empty, duplicate-free, unordered ClaimIRI
    set; the Snapshot fingerprint uses the canonical sorted representation;
    same membership resolves to same SnapshotIRI.

12. Fingerprints provide trust/integrity only against independent
    commitments, anchors, attestations, signatures, or immutable admission
    records.

13. The L0 graph is derived from accepted Claims, not the source of truth.

14. Witnessed audit metadata and asserted content remain distinct.
```

## 19. Changes from the previous model (2026-04-25)

```text
1. Assertion provenance (assertor, asserted_at) removed from the Claim
   value. Attribution and assertion time are user-space content,
   schema-validatable at L1 (was: mandated envelope fields).

2. Claim identity is content-addressed over the Claim value alone
   (was: content + assertor + asserted_at uniqueness rule).

3. ClaimIRI is deterministically derived from the Claim fingerprint
   (was: generation method unresolved).

4. Fingerprint preimage is declared schema references + canonical content
   (was: IRI + content + assertor + asserted_at).

5. Claims declare claim-type schema versions in the Claim value; the
   engine-defined base claim schema is the L0 admission floor
   (was: content vocabulary constraints out of scope).

6. L1 validation is an after-admission judgment recorded as validation
   claims (was: unspecified; implicitly gate-like).

7. Admission is idempotent on resubmission (was: duplicate rejection).

8. ClaimId demoted from domain identity to storage surrogate.

9. accepted_at and submitted material promoted into an explicit witnessed
   SubmissionRecord alongside the submitter principal.

10. Substrate settled as JSON-LD-authored, canonical-RDF-authoritative,
    with LinkML as the schema authoring source.
```

## 20. Remaining ambiguities / revisit later

```text
1. Exact RDF canonicalization profile and version (RDFC 1.0 assumed) and
   the timestamp-literal profile within it.

2. Exact digest suite and ClaimIRI encoding (hex/multibase, suite tag) and
   IRI namespace policy.

3. Whether declared schema versions must be registered at admission or may
   dangle (current lean: allow dangling).

4. Whether schema registrations are themselves Claims.

5. Envelope-level schema declaration vs in-content self-referential
   (blank-node) declaration.

6. Exact shape of the engine-published validation-claim schema.

7. Exact SnapshotIRI generation method.

8. External/source identifier placement (audit evidence vs content vs
   identity mapping).

9. Audit record retention, exposure, and whether submission records are
   ever surfaced as system-generated claims.

10. Exact physical/query representation of L0 and metadata projections.
```

## 21. Compact definition

```text
A Claim is an immutable, content-addressed semantic statement. Its value is
a non-empty duplicate-free set of declared schema version references plus
non-empty canonical RDF-compatible asserted content. Its fingerprint and
ClaimIRI are derived from that value and nothing else.

The engine witnesses ingestion (who submitted, when, exact bytes) in audit
records outside the Claim value. Everything anyone asserts — including who
verified what and when — is claim content, shaped by user-defined claim-type
schemas and judged at L1.

L0 admits and stores Claims that meet the base claim schema. L1 evaluates
Claims against declared claim-type schemas and records outcomes as
validation claims. L2 derives products from claims and validation claims.

A Snapshot is a stable named selection of immutable Claim references with a
derivable fingerprint. Fingerprints support trust only against independent
commitments, anchors, attestations, signatures, or admission records.

Relationships are not separate primitives; Claims may assert relations
about anything addressable, including Claims, Snapshots, and schemas.
```
