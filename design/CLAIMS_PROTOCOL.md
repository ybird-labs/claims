# Claims Engine Core Protocol Specification

Date: 2026-09-12 (revises 2026-09-10)
Status: **Draft**

Change note (2026-09-12): the per-instance engine namespace is deleted.
Engine-minted names now use one protocol-fixed, origin-free reference
form under the protocol-owned `claims` URI scheme (§5.3; scheme
registration per RFC 7595 is planned but not yet done). The
validation-result vocabulary moves under the same scheme (§8). An
additive, non-identity resolution-gateway convention is added (§5.4).
Fingerprint suites and preimage layouts are unchanged; every claim-level
fingerprint in §11 is byte-identical to the 2026-09-10 revision. The
snapshot fingerprint (V-SNAP-1) and the §8 example changed because their
inputs contain ClaimIRIs; all values were regenerated from the reference
generator. Grounding: `design/CONTENT_ADDRESSED_NAMING_ACADEMIC_REVIEW.md`
(origin-free identity conditional on operated resolution; DyLDO decay;
trusty-URI precedent) and the ATProto parallel (no server in names; hash
as trust anchor). RFC 6920 (`ni:`) was rejected on semantics: its
verification model is hash-of-retrieved-bytes; ours is
hash-of-constructed-preimage.

This document specifies the core, single-engine claims protocol: the two
protocol objects (Claim, Snapshot), byte-exact identity derivation, the
admission floor, the validation-claim vocabulary, and the attestation
boundary. It is derived from `design/CLAIMS_ENGINE_DOMAIN_MODEL.md`
(2026-09-12 revision) and the reference semantics of the `spike/` codebase.
The spike is not in the main branch; it is preserved at git tag
`spike-archive-2026-09`, and paths named `spike/...` in this document refer
to that tag.

Out of scope, deliberately: federation, claim packages, schema-hash naming,
release records, registries, and any transport or API binding. Schema
*authoring* and validation *tooling* are also out of scope; only the
declaration and verdict surfaces they touch are specified here.

## 1. Requirements language

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and
"OPTIONAL" in this document are to be interpreted as described in BCP 14
[RFC 2119] [RFC 8174] when, and only when, they appear in all capitals, as
shown here.

All examples, notes, and diagrams are informative. Everything else is
normative.

## 2. Conformance

Conformance is judged per class. This specification defines five classes:

1. **Conforming claim document** — submitted material that satisfies the
   admission floor (§7). Says nothing about truth, schema conformance, or
   attestation.
2. **Conforming fingerprint processor** — given the inputs of §6, produces
   the exact canonical bytes, fingerprint, and IRI this specification
   defines — for claims, snapshots, and submitted material — or raises the
   defined error.
3. **Conforming admission processor** — applies exactly the checks of §7,
   no more and no fewer, and reports the defined results and errors.
   Conformance is explicitly *not* "checks schema conformance": an
   admission processor that rejects a claim for failing its declared
   schema does not conform.
4. **Conforming attestation suite specification** — an external document
   that satisfies the requirements of §9.
5. **Conforming verifier** — recomputes and checks fingerprints, IRIs, and
   attestation commitments per §6 and §9. Verifier conformance is never
   "determines truth".

## 3. Terminology and notation

- **Octet string**: a finite sequence of bytes. `||` is concatenation of
  octet strings. `0x0A` is the line-feed octet. String literals in
  preimage layouts (e.g. `"schema "`) denote their UTF-8 octets.
- **UTF-8(s)**: the UTF-8 encoding of Unicode string *s*, with no BOM and
  no terminator.
- **SHA-256(m)**: the SHA-256 digest of octet string *m* [FIPS 180-4].
- **lowercase hex**: the 64-character representation of a 32-byte digest
  using digits `0-9` and lowercase `a-f`.
- **Canonical N-Quads**: the output of the RDFC-1.0 canonicalization
  algorithm [RDF-CANON] with SHA-256 blank-node hashing, serialized as
  canonical N-Quads, encoded as UTF-8. Each quad line ends with `0x0A`;
  the empty dataset canonicalizes to the empty string.
- **Absolute IRI**: an IRI per RFC 3987 with a scheme; relative references
  are not absolute IRIs.
- **Canonical IRI-set order**: bytewise lexicographic order of the UTF-8
  encodings of the IRI strings (equivalently, Unicode code point order),
  after removing duplicates. "Duplicate" means byte-identical UTF-8; no
  IRI normalization is applied.
- **Claim value**: the pair (declared schema references, canonical
  content) — the only input to claim identity.
- **Submitted material**: the exact octet string that arrived at the
  engine. Distinct from the Claim value derived from it.
- **Reference form**: the protocol-fixed, origin-free URI form of
  engine-minted names (§5.3). No deployment value participates in it.

## 4. Objects

### 4.1 Claim

A **Claim** is an immutable, content-addressed semantic statement that
declares the schemas it claims to conform to. Its value is:

- **declared schema references**: a non-empty, duplicate-free set of
  absolute IRIs, in canonical IRI-set order;
- **canonical content**: non-empty canonical N-Quads in which every quad
  is in the default graph (no quad carries a graph label).

The Claim's fingerprint and ClaimIRI are derived from this value and
nothing else (§6.1). Same value, same ClaimIRI, same Claim.

**The exclusion rule.** No submission fact — submitter identity, receipt
time, transport metadata, the submitted bytes, or their fingerprint —
is part of a Claim's value. An engine MUST NOT inject any such fact into
the Claim value or the fingerprint preimage. (Content *authored by a
submitter* may assert anything, including material fingerprints — that is
ordinary asserted content, not injection.)

The engine is transparent machinery: it holds no privileged facts, and
its records are never a basis for trust. Proof happens only through
attestation (§9).

### 4.2 Snapshot

A **Snapshot** is a stable selection of Claims: a non-empty,
duplicate-free, unordered set of ClaimIRIs. Its fingerprint is computed
over the canonical sorted membership alone (§6.2) — not over Claim
contents, Claim fingerprints, or the SnapshotIRI itself. Same membership,
same SnapshotIRI, same Snapshot.

## 5. Fingerprint suites

### 5.1 Suite identifiers are frozen versioned tuples

Claim and snapshot fingerprint preimages begin with a versioned suite
tag; the submitted-material suite is deliberately untagged (§5.2). A
suite tag permanently fixes, as one tuple: input definition,
canonicalization profile, preimage layout, digest algorithm, and digest
representation. A published tag MUST NOT be reinterpreted; any change to
any element of the tuple requires a new tag. Distinct tags provide
domain separation between the tagged suites; §10 states that guarantee,
its computational nature, and its one deliberate limit — the untagged
material suite.

### 5.2 The v1 suites

This specification freezes three v1 suites (the tags are the exact octets
used by the reference spike, `spike/src/identity.rs`):

| Suite tag (exact ASCII) | Input | Canonicalization | Digest |
|---|---|---|---|
| `claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v1` | Claim value (§4.1) | RDFC-1.0 → canonical N-Quads, UTF-8 | SHA-256, lowercase hex |
| `claims-spike/snapshot-membership/sorted-iris-utf8/sha256/v1` | Snapshot membership (§4.2) | canonical IRI-set order, UTF-8 | SHA-256, lowercase hex |
| `claims-spike/submitted-material/raw-bytes/sha256/v1` | exact received octets | none | SHA-256, lowercase hex |

The submitted-material suite's tag names the procedure but is NOT part of
its preimage: the preimage is the raw received octets, so the fingerprint
equals a plain SHA-256 file hash of the material (see §10 for the
consequence).

A conforming fingerprint processor MUST implement all three v1 suites.

### 5.3 Reference form: the `claims` URI scheme

Engine-minted names — ClaimIRIs and SnapshotIRIs — use exactly one
protocol-fixed, origin-free reference form under the protocol-owned
`claims` URI scheme. No deployment value participates in identity; there
is no per-instance engine namespace. Registration status, stated
honestly: provisional registration of the scheme per RFC 7595 is
planned; it is not yet registered.

Canonical byte-forms (normative; exactly one form per name, no aliases):

```text
ClaimIRI    = "claims:claim:"    || pipeline || ":" || fingerprint
SnapshotIRI = "claims:snapshot:" || pipeline || ":" || fingerprint
pipeline    = "rdfc10-sha256"
fingerprint = 64 lowercase hex characters
```

Every octet is lowercase — scheme, kind segment, pipeline segment, hex
digits. A string differing in any octet from the canonical byte-form
(uppercase anywhere, percent-encoding, added slashes, whitespace) is not
a name of this protocol; wherever this specification checks name shape,
such a string MUST be rejected, never normalized into a valid name.
(One form, no aliases: consumer-side normalization is where lookalike
identities creep in.)

The `pipeline` segment names the identity pipeline, not the preimage
tag. It maps to the frozen v1 suites of §5.2:

| Name form | `pipeline` | Fingerprint suite (§5.2) |
|---|---|---|
| `claims:claim:…` | `rdfc10-sha256` | `claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v1` |
| `claims:snapshot:…` | `rdfc10-sha256` | `claims-spike/snapshot-membership/sorted-iris-utf8/sha256/v1` |

Suite tags and preimage layouts are unchanged by the reference form. A
future pipeline (new canonicalization or digest) requires a new
`pipeline` segment value mapped to a new suite tag — never a
reinterpretation of `rdfc10-sha256`.

These names are absolute IRIs (scheme `claims`, opaque path) and are
valid wherever this specification requires an absolute IRI, including
inside canonical N-Quads content.

### 5.4 Resolution gateways (additive, non-identity)

Names in the reference form carry no resolution authority and never
promise persistence: a name that resolves nowhere is still a valid name,
and resolution failure is not evidence about the named value.

As an additive convention, any host MAY expose an HTTP gateway at a
well-known-style path carrying the name verbatim:

```text
https://<host>/.well-known/claims/<name>
e.g. https://<host>/.well-known/claims/claims:claim:rdfc10-sha256:<64 hex>
```

Rules:

- Any host may run a gateway; no gateway is special; a gateway's host
  never enters identity. The same name MAY be fetched from any number of
  gateways.
- A gateway response is a candidate value only. The client MUST verify
  it byte-exactly: recompute the fingerprint from the returned value
  under the suite the name's `pipeline` segment maps to (§6) and compare
  it to the name's fingerprint segment. A response that fails
  recomputation MUST be rejected. This applies at every gateway, always
  — there are no trusted gateways.
- Identity never depends on any gateway: nothing in §§6–7 reads, cites,
  or varies with this convention.

## 6. Identity algorithms

Each algorithm is normative: a conforming fingerprint processor MUST
follow the stated steps or produce byte-identical results by other means.
Each has a verifier dual, ending in an explicit accept/reject.

### 6.1 Claim fingerprint and ClaimIRI

Inputs: `schemas` — the declared schema references, already validated and
in canonical IRI-set order (§7 steps 1–2); `content` — the canonical
N-Quads octet string, already satisfying the Claim content constraints
(§7 steps 3–7: canonical, non-empty, default graph only).

```text
preimage = UTF-8(tag) || 0x0A
        || for each iri in schemas, in canonical IRI-set order:
               "schema " || UTF-8(iri) || 0x0A
        || "---" || 0x0A
        || content

fingerprint = lowercase hex of SHA-256(preimage)
ClaimIRI    = "claims:claim:rdfc10-sha256:" || fingerprint
```

where `tag` is `claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v1`.
No octet is appended after `content`: the canonical N-Quads already end
with `0x0A` (per quad line), and the empty case cannot arise because
admission rejects empty content.

Example (informative; this is test vector V-CLAIM-2, §11.2). For the
schema set `{https://x.example/schema/1}` and the one-triple canonical
content shown on line 4, the preimage is exactly these four
LF-terminated lines:

```text
claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v1
schema https://x.example/schema/1
---
<https://x.example/s> <https://x.example/p> "v" .
```

giving fingerprint
`db3dba6d98dde0c6160b44ac8fc5c411e1e35038e7a8360c8fc4e6195911d86f`.

*Verifier dual.* Inputs: a ClaimIRI, a claimed schema set, and claimed
canonical content. Verification needs no access to any engine store and
never checks conformance to the declared schemas. Steps, in order; each
failure is terminal:

1. Parse the IRI as the canonical byte-form
   `claims:claim:rdfc10-sha256:<64 lowercase hex>` (§5.3), byte-exactly;
   reject (`ERR_IRI_SHAPE`) if it does not match — including a
   snapshot-shaped name, a name under any other scheme (e.g. an
   `https:` URL), an unknown pipeline segment, or uppercase hex.
2. Validate the schema set exactly as §7 steps 1–2 (non-empty; every
   reference a well-formed absolute IRI; deduplicate and sort into
   canonical IRI-set order), rejecting with the same errors.
3. Parse the claimed content as N-Quads; reject (`ERR_MALFORMED`) on
   failure. Reject (`ERR_EMPTY_CONTENT`) if it is empty and
   (`ERR_NAMED_GRAPH`) if any quad carries a graph label.
4. Check canonicality: canonicalize the parsed dataset with RDFC-1.0
   and compare the result byte-for-byte with the claimed content;
   reject (`ERR_NOT_CANONICAL`) on inequality. A verifier MUST NOT
   trust the "canonical" label on its input.
5. Recompute the fingerprint as above. Accept if and only if it equals
   the IRI's hex segment; otherwise reject
   (`ERR_FINGERPRINT_MISMATCH`).

### 6.2 Snapshot fingerprint and SnapshotIRI

Input: `members` — a non-empty set of ClaimIRIs. Every member MUST match
the canonical ClaimIRI byte-form
`claims:claim:rdfc10-sha256:<64 lowercase hex>` (§5.3); on any member
that does not — a SnapshotIRI, a URL under any other scheme, arbitrary
text — raise `ERR_MEMBER_NOT_CLAIM_IRI`
(a Snapshot is a selection of Claims, §4.2, and this is the only shape
that names one). Then remove duplicates and sort into canonical IRI-set
order. If the set is empty, raise `ERR_EMPTY_SNAPSHOT`.

```text
preimage = UTF-8(tag) || 0x0A
        || for each iri in members, in canonical IRI-set order:
               UTF-8(iri) || 0x0A

fingerprint = lowercase hex of SHA-256(preimage)
SnapshotIRI = "claims:snapshot:rdfc10-sha256:" || fingerprint
```

where `tag` is `claims-spike/snapshot-membership/sorted-iris-utf8/sha256/v1`.
Because the preimage is computed over the canonical sorted set,
insertion order and duplicates cannot affect identity.

*Verifier dual.* Inputs: a SnapshotIRI and a claimed membership set.
Steps, in order; each failure is terminal:

1. Parse the IRI as the canonical byte-form
   `claims:snapshot:rdfc10-sha256:<64 lowercase hex>` (§5.3),
   byte-exactly; reject (`ERR_IRI_SHAPE`) if it does not match.
2. Check every claimed member against the ClaimIRI shape exactly as in
   derivation above; reject (`ERR_MEMBER_NOT_CLAIM_IRI`) on the first
   member that does not match. A verifier MUST NOT hash an unchecked
   membership: a matching digest over non-ClaimIRI strings is not a
   Snapshot.
3. Canonicalize the membership (deduplicate, sort); reject
   (`ERR_EMPTY_SNAPSHOT`) if it is empty.
4. Recompute the fingerprint as above. Accept if and only if it equals
   the IRI's hex segment; otherwise reject
   (`ERR_FINGERPRINT_MISMATCH`).

Note (informative): the reference spike's `snapshot_fingerprint`
predates the member-shape check and hashes any string set; this
specification requires the check. The text wins (§11 authority rule).

### 6.3 Submitted-material fingerprint

```text
fingerprint = lowercase hex of SHA-256(material)
```

over the exact received octets, with no prefix. This fingerprint
identifies bytes, not a protocol object; it mints no IRI. It exists to be
cited — by validation verdicts (`judged_material`, §8) and by
attestations (§9).

*Verifier dual.* Given material octets and a claimed fingerprint:
recompute as above. Accept if and only if they are equal; otherwise
reject (`ERR_FINGERPRINT_MISMATCH`).

## 7. Admission

Admission turns submitted material into a Claim, enforcing only the
**base claim schema floor**. The inputs are the submitted material (an
octet string) and the declared schema references (supplied in the
submission envelope). Steps, in order; each failure is terminal:

1. If the declared schema reference list is empty, reject:
   `ERR_NO_SCHEMA_DECLARATION`.
2. For each declared reference: if it is not a well-formed absolute IRI,
   reject: `ERR_INVALID_SCHEMA_IRI`. Deduplicate and sort the survivors
   into canonical IRI-set order. Declared references MAY dangle — nothing
   is fetched, resolved, or checked for registration.
3. If the material is not valid UTF-8, reject: `ERR_NOT_UTF8`.
4. Parse the material as **self-contained** JSON-LD and transform it to
   an RDF dataset under the processing profile of §7.1. The processor
   MUST NOT fetch remote contexts; a document referencing a context it
   does not embed fails. On any parse or expansion failure, reject:
   `ERR_MALFORMED`.
5. Canonicalize the resulting dataset with RDFC-1.0 to canonical
   N-Quads. On canonicalization failure (including a resource-exhaustion
   abort, see §10), reject: `ERR_CANONICALIZATION`.
6. If the canonical N-Quads are empty, reject: `ERR_EMPTY_CONTENT`.
7. If any canonical quad carries a graph label (any content outside the
   default graph), reject: `ERR_NAMED_GRAPH`.
8. Compute the claim fingerprint and ClaimIRI (§6.1). Durably record the
   Claim under its ClaimIRI. The Claim is thereafter immutable.

Admission MUST NOT check conformance to any declared schema. Conformance
is L1's judgment, recorded after the fact as a validation claim (§8) —
a Claim can exist unvalidated or failing validation.

**Idempotency** is a consequence of content addressing, not a rule an
implementation adds: the same Claim value always derives the same
ClaimIRI, so admitting an already-admitted value MUST succeed and MUST
resolve to the existing Claim. There is no duplicate-claim error.

Error names above identify testable outcomes; their wire representation
is out of scope. (The reference spike's error variants map one-to-one
except `ERR_NAMED_GRAPH`, which the spike predates — see vector N-5.)

### 7.1 JSON-LD processing profile

The step-4 transformation determines which canonical content — and
therefore which ClaimIRI — a given material derives, so its
identity-affecting options are frozen here with the same force as a
suite tag: any change requires a new version of this specification. The
transformation is the Deserialize JSON-LD to RDF algorithm of
[JSON-LD11-API] with:

| Option | Value |
|---|---|
| processing mode | `json-ld-1.1` [JSON-LD11] |
| base IRI | none |
| expansion context (`expandContext`) | none |
| document loader | none — any remote context or document reference fails (`ERR_MALFORMED`) |
| `produceGeneralizedRdf` | `false` |
| `rdfDirection` | `null` — an `@direction` yields a literal without direction |

The rules below close the identity-affecting discretion the referenced
algorithms leave open; each cites the vector that demonstrates the fork
it prevents.

**Native numbers decode as binary64.** [JSON-LD11-API] §8.6 leaves the
precision of native JSON numbers implementation-defined. This profile
fixes it: a native JSON number is decoded to an IEEE 754 binary64 value
(round-to-nearest, ties-to-even) before the conversion algorithm
classifies it, and the algorithm's `xsd:integer`/`xsd:double` split and
canonical lexical mappings then apply to the decoded value. An integer
beyond binary64's exact range therefore rounds silently:
`9007199254740993` (2⁵³ + 1) decodes to 9007199254740992 and derives
the same Claim as authoring `9007199254740992` (vector V-CLAIM-7,
§11.8). A processor that preserves such a value exactly does not
conform. Note (informative): authors needing exact integers beyond 2⁵³
should author a typed literal — e.g. `{"@value":"9007199254740993",
"@type":"http://www.w3.org/2001/XMLSchema#integer"}` — whose lexical
form is preserved verbatim.

**Language tags are preserved byte-for-byte.** [JSON-LD11-API] permits
a processor to normalize language tags to lowercase; this profile
forbids it: at no stage — JSON-LD deserialization, canonicalization, or
the N-Quads parsing of a verifier (§6.1) — may a processor alter the
case of a language tag. A tag appears in the canonical N-Quads exactly
as authored; literals differing only in tag case derive distinct
fingerprints (vector V-CLAIM-4, §11.5).

Preservation extends to term identity: two literals differing only in
language-tag case are **distinct RDF terms** (literal term equality is
character-by-character, as in RDF 1.1), so one dataset can contain
both. A processor MUST NOT merge case-variant literals or drop one:
keeping "the first seen" makes the ClaimIRI depend on authoring order
(vector V-CLAIM-5, §11.6).

**JSON literals from `@json` values are serialized with JCS.** A value
whose expanded type is the keyword `@json` becomes a literal of
datatype `rdf:JSON`, and [JSON-LD11] §10.2 leaves SHOULD-level
discretion in that serialization (a character may be written literally
or as a Unicode escape). This profile closes it: the lexical form
produced by that serialization MUST be the JSON Canonicalization Scheme
[RFC 8785] serialization of the value — JCS string escaping, object-key
order, and number serialization (vector V-CLAIM-6, §11.7). The rule
governs only this serialization step: a literal the author spells out
directly — an explicit string `@value` typed with the full `rdf:JSON`
IRI — is an ordinary typed literal and keeps its authored lexical form,
like every other literal (see the note in §11.7).

**Base IRI.** The "none" above is the *initial* base: an embedded
context can still establish a base via `@base`, per the [JSON-LD11]
context-processing rules, and references it resolves are ordinary
absolute IRIs. For example,

```json
{"@context":{"@base":"https://x.example/","p":"https://x.example/p"},"@id":"s","p":"v"}
```

produces the same absolute-subject triple as V-CLAIM-2 (§11.2). A
relative reference that remains unresolved after context processing
never becomes an absolute IRI, and the algorithm drops the triples
depending on it; a document whose content entirely drops is rejected at
step 6 (`ERR_EMPTY_CONTENT`). Note (informative): a reference left
unresolved drops *silently* — authors should either use absolute IRIs
or embed the `@base` that resolves their relative ones.

Note (informative): the reference spike inherits a library-default
synthetic base IRI for string input, under which relative references
lacking any embedded `@base` still resolve (to `x-string:///…`). That
default is deliberately not carried forward; no initial base IRI is the
normative choice, and the text wins (§11 authority rule).

## 8. Validation claims

Validation is a judgment about an existing Claim, never a write-path
gate. A verdict is recorded as a **validation claim** — an ordinary Claim
admitted through §7 like any other, with no special path and no special
authority.

Because claim content is an RDF dataset, the vocabulary is a set of RDF
terms, not JSON field names. The vocabulary is protocol-fixed and
origin-free, under the same scheme as the reference form (§5.3): all
terms live under `vr:` = `claims:vocab:validation-result:1.0.0#`, and a
validation claim declares the schema reference
`claims:vocab:validation-result:1.0.0`. The fragment form is chosen
because it makes the versioned vocabulary a single IRI — the declared
schema reference — with every term a fragment of it, each a valid
absolute IRI in canonical N-Quads. A verdict is a node — typically a
blank node — of type `vr:ValidationResult`, asserting:

| Term (under `vr:`) | Value form | Meaning |
|---|---|---|
| `validator` | IRI | the judging service; REQUIRED |
| `target_claim` | IRI | the ClaimIRI judged; REQUIRED |
| `schema_version` | IRI | the schema version judged against; REQUIRED |
| `outcome` | string literal | `conforms` or `violations`; REQUIRED |
| `violations` | `xsd:integer` literal | violation count — a nonnegative integer: `0` when `outcome` is `conforms`, positive when `violations`; REQUIRED |
| `judged_material` | string literal, 64 lowercase hex | submitted-material fingerprint (§6.3); present if and only if the verdict judged an authored form (see below) |
| `validated_at` | `xsd:dateTime` literal | assertion time — ordinary content like any other; REQUIRED |

Example (informative). A verdict over V-CLAIM-2 (§11.2), judged from
its authored form, canonicalizes to:

```text
_:c14n0 <claims:vocab:validation-result:1.0.0#judged_material> "e2edcb5e1c6e0445eefc5e31cdc676edbaddaab510de2b38aa9fd601c6cde33d" .
_:c14n0 <claims:vocab:validation-result:1.0.0#outcome> "conforms" .
_:c14n0 <claims:vocab:validation-result:1.0.0#schema_version> <https://x.example/schema/1> .
_:c14n0 <claims:vocab:validation-result:1.0.0#target_claim> <claims:claim:rdfc10-sha256:db3dba6d98dde0c6160b44ac8fc5c411e1e35038e7a8360c8fc4e6195911d86f> .
_:c14n0 <claims:vocab:validation-result:1.0.0#validated_at> "2026-09-10T12:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
_:c14n0 <claims:vocab:validation-result:1.0.0#validator> <https://validators.example/l1> .
_:c14n0 <claims:vocab:validation-result:1.0.0#violations> "0"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <claims:vocab:validation-result:1.0.0#ValidationResult> .
```

This claim declares the schema set `{claims:vocab:validation-result:1.0.0}`
and derives claim fingerprint
`617d0c2160486acbdcefd91fbbd6c7432e0ac8a4d2c3f3cc9e6837c3238576a8`
(ClaimIRI
`claims:claim:rdfc10-sha256:617d0c2160486acbdcefd91fbbd6c7432e0ac8a4d2c3f3cc9e6837c3238576a8`),
generated by `spike/examples/print_vectors.rs` through the spike
canonicalizer.

The engine's verdict vocabulary is exactly `conforms | violations`. A
validation claim's `outcome` MUST be one of these two values. Richer
judgment vocabularies (e.g. tri-state review outcomes) are user-space
claim-type schemas layered on top as ordinary claims; they are not
validation claims and this section does not constrain them.

The authoritative conformance judgment is defined over the canonical
dataset. A verdict computed over an *authored form* (e.g. JSON Schema
over submitted JSON-LD) is submission-scoped — one Claim may accumulate
many differently-authored submissions, and a verdict over one authored
form says nothing about the others. `judged_material` is the scope
marker, and its presence carries exactly one meaning:

- **present** — the verdict judged exactly the authored bytes with that
  submitted-material fingerprint (§6.3), and nothing more. An
  authored-form verdict MUST include it. The example above is this
  case: it judged V-CLAIM-2's 76 submitted bytes.
- **absent** — the verdict judged the canonical dataset, and therefore
  the Claim itself under every authored form. A canonical-dataset
  verdict MUST NOT include it.

## 9. Attestation boundary

Attestation is the only proof mechanism in this protocol. Engine
records — storage, logs, projections — prove nothing to anyone else;
immutability gains adversarial weight exclusively through attestation.

An **attestation** commits to a fingerprint: a claim fingerprint, a
snapshot fingerprint, or a submitted-material fingerprint. The
committed-to octets are the 64 lowercase-hex characters as defined in
§6, or an encoding of them that the attestation suite specification
defines byte-exactly.

What an attestation can prove is bounded:

- At most **existence by a time**: that the fingerprinted value existed
  no later than the substrate's time bound for the commitment.
- Never truth, accuracy, schema conformance, or endorsement of the
  fingerprinted content. A verifier MUST NOT treat an attestation as any
  of these.

The attestation *mechanism* — substrates, signatures, anchoring,
transport — is out of scope. A conforming attestation suite
specification MUST define, completely enough to implement and verify
independently: (a) exactly which fingerprint octets are committed and any
encoding applied to them; (b) the commitment procedure; (c) the
verification procedure, ending in explicit accept/reject; (d) the time
bound the substrate provides and its failure modes.

## 10. Security considerations

- **Poison graphs.** RDFC-1.0 canonicalization has pathological inputs.
  A processor MUST bound canonicalization work and abort with
  `ERR_CANONICALIZATION` rather than run unbounded (RDFC-1.0 §7.1
  dataset poisoning).
- **Remote contexts.** Fetching contexts at admission would make claim
  identity depend on remote, mutable state. §7 step 4 forbids it; a
  processor that fetches does not conform.
- **Domain separation.** Claim and snapshot preimages begin with
  distinct frozen tags (§5.1), so equal payload bytes under any two
  tagged suites — v1 or future — are different preimage octet strings
  and, by SHA-256's collision resistance, derive different
  fingerprints. This is a computational guarantee, not a mathematical
  impossibility, and it does not cover the submitted-material
  fingerprint, which is deliberately untagged (§5.2): it equals a plain
  SHA-256 of the bytes. So material
  whose octets happen to equal a claim or snapshot *preimage* has, under
  the material suite, exactly that claim or snapshot fingerprint — no
  cryptographic attack required. A bare fingerprint therefore carries no
  evidence of its object kind: an attestation over a fingerprint (§9)
  MUST be interpreted against the value and fingerprint suite the
  attesting party intended, never as proof that the fingerprint names a
  Claim, a Snapshot, or material. The untagged suite MUST NOT be used
  where a domain-separated commitment is required.
- **Trust laundering.** Validation claims are ordinary claims: anyone can
  admit a claim shaped like a verdict. Consumers derive trust from
  *which validators they choose to believe* plus attestation — never from
  the fact that a verdict exists in the store.
- **Suite downgrade/reinterpretation.** A verifier MUST reject a
  fingerprint claimed under a tag it does not recognize rather than guess
  (`ERR_UNKNOWN_SUITE`).

## 11. Test vectors

**Authority rule.** These vectors are evidence, not definition: if a
vector conflicts with the normative text, the text wins and the vector is
corrected. All hash values below were produced by running the reference
spike's fingerprint code (§§11.1–11.4 and the §8 example via
`cd spike && cargo run --example print_vectors` in a checkout of tag
`spike-archive-2026-09`,
`spike/examples/print_vectors.rs`, which also runs the spike's
canonicalizer; V-CLAIM-4, V-CLAIM-6, and V-CLAIM-7 by driving
`canon::jsonld_to_canonical_nquads`
and `identity::claim_fingerprint` directly; V-CLAIM-5 by driving
`identity::claim_fingerprint` and
`identity::submitted_material_fingerprint` over the normative canonical
content — the spike's canonicalizer diverges there, see §11.6) and
independently re-derived from the preimage layouts with
`shasum -a 256`; none is hand-written.

In every preimage block below, each line ends with a single `0x0A` and
there are no octets after the final `0x0A`.

### 11.1 V-CLAIM-1: same claim from two authored forms (positive)

Materials: `spike/fixtures/material_good_a.jsonld` and
`spike/fixtures/material_good_b.jsonld` — the same carbon-verification
statement with keys and context entries in different order. Both declare
the single schema
`https://claims.example/schema/carbon-storage-verification/1.0.0`.

Submitted-material fingerprints (different bytes, per §6.3):

```text
material_good_a: 7ebfcab934e6ecd18c52a2f5c2880e76226b2b1bf7f14b964fcd12cc948158ef
material_good_b: 0531c0a295410d6750d4bef2371e3f898fe6dfed8c80212f8b93992e885d3f15
```

Both canonicalize to the identical canonical N-Quads (1205 bytes, 9
lines):

```text
<https://verifiers.example/alice> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://claims.example/schema/carbon-storage-verification/1.0.0/Verifier> .
<https://verifiers.example/alice> <https://claims.example/schema/carbon-storage-verification/1.0.0/accreditation_id> "ACR-2031-0042" .
<https://verifiers.example/alice> <https://claims.example/schema/carbon-storage-verification/1.0.0/name> "Alice Chen" .
_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://claims.example/schema/carbon-storage-verification/1.0.0/CarbonStorageVerification> .
_:c14n0 <https://claims.example/schema/carbon-storage-verification/1.0.0/site> <https://sites.example/field-7> .
_:c14n0 <https://claims.example/schema/carbon-storage-verification/1.0.0/storage_method> "biochar soil amendment" .
_:c14n0 <https://claims.example/schema/carbon-storage-verification/1.0.0/tons_co2> "5"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:c14n0 <https://claims.example/schema/carbon-storage-verification/1.0.0/verified_at> "2026-06-30T09:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
_:c14n0 <https://claims.example/schema/carbon-storage-verification/1.0.0/verifier> <https://verifiers.example/alice> .
```

Both therefore derive the same identity:

```text
claim fingerprint: 7259a54210527dcb0bdc086bda4f08e44b89c7734ac5b5a48c835889a94ac187
ClaimIRI: claims:claim:rdfc10-sha256:7259a54210527dcb0bdc086bda4f08e44b89c7734ac5b5a48c835889a94ac187
```

Resubmitting either material is idempotent: same Claim, no error (§7).

### 11.2 V-CLAIM-2: minimal claim with full preimage (positive)

Material (exact 76 bytes):

```text
{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":"v"}
```

```text
submitted-material fingerprint: e2edcb5e1c6e0445eefc5e31cdc676edbaddaab510de2b38aa9fd601c6cde33d
declared schemas: {https://x.example/schema/1}
canonical n-quads: <https://x.example/s> <https://x.example/p> "v" .
```

The full claim-fingerprint preimage — 4 lines, each ending `0x0A` — and
the resulting fingerprint are shown beside the algorithm in §6.1; they
are this vector's values.

```text
ClaimIRI: claims:claim:rdfc10-sha256:db3dba6d98dde0c6160b44ac8fc5c411e1e35038e7a8360c8fc4e6195911d86f
```

Independent check: `printf` of exactly the four LF-terminated preimage
lines piped to `shasum -a 256` reproduces the fingerprint.

### 11.3 V-CLAIM-3: schema-set order insensitivity (positive)

Same content as V-CLAIM-2, declaring two schemas supplied in reverse
order: `[https://x.example/schema/2, https://x.example/schema/1]`. The
canonical IRI-set order is `schema/1`, then `schema/2`, so the preimage
carries two `schema ` lines in that order.

```text
claim fingerprint: 487e48b401645af2aeca763db86e7d0cbc7ec86f6534c1116c437304a89ee7f6
ClaimIRI: claims:claim:rdfc10-sha256:487e48b401645af2aeca763db86e7d0cbc7ec86f6534c1116c437304a89ee7f6
```

Note (informative): a different declared schema set over identical
content is a different Claim — compare V-CLAIM-2.

### 11.4 V-SNAP-1: snapshot order insensitivity (positive)

Membership supplied as `[V-CLAIM-2 IRI, V-CLAIM-3 IRI]` — not in
canonical order: V-CLAIM-3's fingerprint (`487e…`) sorts before
V-CLAIM-2's (`db3d…`). Full preimage (3 lines, each ending `0x0A`):

```text
claims-spike/snapshot-membership/sorted-iris-utf8/sha256/v1
claims:claim:rdfc10-sha256:487e48b401645af2aeca763db86e7d0cbc7ec86f6534c1116c437304a89ee7f6
claims:claim:rdfc10-sha256:db3dba6d98dde0c6160b44ac8fc5c411e1e35038e7a8360c8fc4e6195911d86f
```

```text
snapshot fingerprint: babc970eb1d84b99227962243509c677492e8d051a07466727752644632469d2
SnapshotIRI: claims:snapshot:rdfc10-sha256:babc970eb1d84b99227962243509c677492e8d051a07466727752644632469d2
```

Independent check: the same three LF-terminated lines through
`shasum -a 256` reproduce the fingerprint. Supplying the members in
sorted order, or with a duplicate —
`[V-CLAIM-2 IRI, V-CLAIM-3 IRI, V-CLAIM-2 IRI]` — yields the same
preimage and the same identity.

### 11.5 V-CLAIM-4: language-tag case is identity (positive)

Exercises the language-tag rule of §7.1. Material (exact 107 bytes),
declaring `{https://x.example/schema/1}`:

```text
{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":{"@value":"v","@language":"en-US"}}
```

```text
submitted-material fingerprint: 8ac3d2a2da1cf884d1b87afe9398fc5795fe6da2d469c2ccecae1ace9c0ed7f1
canonical n-quads: <https://x.example/s> <https://x.example/p> "v"@en-US .
claim fingerprint: 1446c165c0ddc64cedcdafd93ba2994af128430b4697959442f97e94664a8f12
ClaimIRI: claims:claim:rdfc10-sha256:1446c165c0ddc64cedcdafd93ba2994af128430b4697959442f97e94664a8f12
```

The tag survives as `@en-US` — authored case, untouched. The same
material with `"@language":"en-us"` canonicalizes to `"v"@en-us .` and
derives claim fingerprint
`7117d2dbccb213007027ff68bd740cdb79bd409b05647b10a54b9727760f88cf` —
a different Claim. A processor that lowercases the tag computes the
second fingerprint from the first material and does not conform.

### 11.6 V-CLAIM-5: case-variant literals are distinct terms (positive)

Exercises the term-identity rule of §7.1: case-variant literals survive
dataset construction as distinct terms. Material (exact 144 bytes),
declaring `{https://x.example/schema/1}`:

```text
{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":[{"@value":"v","@language":"en-US"},{"@value":"v","@language":"en-us"}]}
```

Both literals survive as distinct terms, so the canonical N-Quads are 2
lines (`@en-US` sorts before `@en-us`: `U` is `0x55`, `u` is `0x75`):

```text
<https://x.example/s> <https://x.example/p> "v"@en-US .
<https://x.example/s> <https://x.example/p> "v"@en-us .
```

```text
submitted-material fingerprint: 518410ab9cc5e283b5f38587afd8182f67c658f59df2aaf4506b06e8be4e71aa
claim fingerprint: a955b33516984c91e84ad441c2647a5aff21a845196ae0e80d709aef3d386a51
ClaimIRI: claims:claim:rdfc10-sha256:a955b33516984c91e84ad441c2647a5aff21a845196ae0e80d709aef3d386a51
```

Reversing the two array entries changes the submitted bytes (and their
material fingerprint) but not the dataset, so both orders derive this
one Claim. A processor that merges case-variant literals instead
derives V-CLAIM-4's `1446…` from the first order and `7117…` from the
second — identity forked by authoring order, nonconforming either way.

Note (informative): the reference spike's canonicalizer merges
case-variant literals, keeping the first seen, and exhibits exactly
that order dependence. The normative text wins (§11 authority rule).

### 11.7 V-CLAIM-6: JSON literals serialize canonically (positive)

Exercises the JSON-literal rule of §7.1. Material (exact 106 bytes),
declaring `{https://x.example/schema/1}`:

```text
{"@context":{"p":{"@id":"https://x.example/p","@type":"@json"}},"@id":"https://x.example/s","p":{"x":"a"}}
```

```text
submitted-material fingerprint: bb5b5a09bf10aea73c8661aef868dd2c65a7fb8cfc8db6dc97ef67437a136712
canonical n-quads: <https://x.example/s> <https://x.example/p> "{\"x\":\"a\"}"^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON> .
claim fingerprint: a6cd647ae7496e7152246528804ad845a34e69f252805efe3364bcb95f5dbf8e
ClaimIRI: claims:claim:rdfc10-sha256:a6cd647ae7496e7152246528804ad845a34e69f252805efe3364bcb95f5dbf8e
```

The embedded JSON value `{"x":"a"}` survives as its JCS serialization:
`a` written literally, no escapes. Writing it as the Unicode escape
`\u0061` instead — permitted by [JSON-LD11] §10.2's SHOULDs, forbidden
here — yields the lexical form `{"x":"\u0061"}` (in N-Quads:
`"{\"x\":\"\\u0061\"}"`) and claim fingerprint
`c903f38683311307e43a2a57247a3fce12f7611cffdf6c4851bf771910f95cbe` —
a different Claim from the same material. A processor emitting that
form does not conform.

Note (informative): the JCS rule governs only the serialization of
`@json` values (§7.1). Authoring the literal directly — replacing this
vector's `"p":{"x":"a"}` with
`"p":{"@value":"{ \"x\": \"a\" }","@type":"http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON"}`
(and dropping `"@type":"@json"` from the context) — is an ordinary
typed literal whose authored spelling survives verbatim: it
canonicalizes to `"{ \"x\": \"a\" }"^^<…#JSON>` and derives claim
fingerprint
`02cf8b20638c044ac7cd8bb80992694c8f4d004bbdd724fab2a738ca527cbc48` —
a distinct Claim, exactly as any two literals with different bytes are.

### 11.8 V-CLAIM-7: native numbers decode as binary64 (positive)

Exercises the number rule of §7.1. Two materials, each exactly 89
bytes, each declaring `{https://x.example/schema/1}`:

```text
{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":9007199254740993}
{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/s","p":9007199254740992}
```

```text
submitted-material fingerprint (…993): 4261bf73caf5a2c1486a633f8a422dab65ea5cae832910269d83ed12b54b9839
submitted-material fingerprint (…992): cd6f5d81e36afd1e2aaef72c85cb60c7a7ea4c956341d2a2f976230ea913dbca
```

`9007199254740993` (2⁵³ + 1) is not exactly representable in binary64
and decodes to 9007199254740992, so both materials canonicalize to the
same single quad:

```text
<https://x.example/s> <https://x.example/p> "9007199254740992"^^<http://www.w3.org/2001/XMLSchema#integer> .
```

```text
claim fingerprint: 69b23cef2899de7265de9cfc5aa75a67860f5cada0359914e4815765df1008ba
ClaimIRI: claims:claim:rdfc10-sha256:69b23cef2899de7265de9cfc5aa75a67860f5cada0359914e4815765df1008ba
```

One Claim from both materials. A processor that preserves the integer
exactly emits `"9007199254740993"` and derives a different fingerprint
from the first material; it does not conform.

### 11.9 Negative vectors: one per floor rule

Each names the exact rejection of §7. "Result" is the required outcome;
no fingerprint or IRI exists for a rejected submission. N-1 through N-5
declare `["https://x.example/schema/1"]` — a valid declaration, so the
material check under test is reached (declarations are checked first,
§7 steps 1–2).

| ID | Material / declaration | Result |
|---|---|---|
| N-1 | octets `7b ff 7d` | `ERR_NOT_UTF8` (§7 step 3) |
| N-2 | `not json` | `ERR_MALFORMED` (§7 step 4) |
| N-3 | `{}` | `ERR_EMPTY_CONTENT` (§7 step 6): parses and canonicalizes to the empty string |
| N-4 | `{"@context":"https://schema.org","name":"x"}` | `ERR_MALFORMED` (§7 step 4): remote context MUST NOT be fetched, so expansion fails |
| N-5 | `{"@context":{"p":"https://x.example/p"},"@id":"https://x.example/g","@graph":[{"@id":"https://x.example/s","p":"v"}]}` | `ERR_NAMED_GRAPH` (§7 step 7): canonicalizes to `<https://x.example/s> <https://x.example/p> "v" <https://x.example/g> .` — a quad with a graph label |
| N-6 | valid material, declaration `["not-an-absolute-iri"]` | `ERR_INVALID_SCHEMA_IRI` (§7 step 2) |
| N-7 | valid material, declaration `[]` | `ERR_NO_SCHEMA_DECLARATION` (§7 step 1) |

Note (informative): the reference spike rejects N-1 through N-4, N-6,
and N-7 with corresponding error variants; it does not yet implement the
N-5 check (its canonicalizer produces the quad shown, which this
specification requires the admission processor to then reject). The
normative text wins.

### 11.10 Negative vectors: derivation and verification

These exercise §6.2's membership rules and the verifier duals of §6.
Every input below reuses values already derived in §§11.2–11.5; no new
digest appears. The positive duals — each vector of §§11.1–11.8
presented to its own verifier — accept.

| ID | Input | Result |
|---|---|---|
| N-8 | snapshot membership `[]` | `ERR_EMPTY_SNAPSHOT` (§6.2) |
| N-9 | snapshot membership `[V-CLAIM-2 IRI, V-SNAP-1 SnapshotIRI]`, presented to derivation or to the verifier | `ERR_MEMBER_NOT_CLAIM_IRI` (§6.2): the second member is snapshot-shaped, not a ClaimIRI |
| V-VER-1 | V-CLAIM-2's ClaimIRI with V-CLAIM-3's value (schema set `{schema/1, schema/2}`, same content) | reject `ERR_FINGERPRINT_MISMATCH` (§6.1 step 5): recomputed `487e…` ≠ `db3d…` |
| V-VER-2 | V-SNAP-1's SnapshotIRI presented to the claim verifier | reject `ERR_IRI_SHAPE` (§6.1 step 1): wrong IRI kind |
| V-VER-3 | V-CLAIM-2's ClaimIRI and schema set with content `_:b0 <https://x.example/p> "v" .` | reject `ERR_NOT_CANONICAL` (§6.1 step 4): well-formed N-Quads, but RDFC-1.0 relabels `_:b0` to `_:c14n0` |
| V-VER-4 | any fingerprint claimed under tag `claims-spike/claim-value/rdfc10-nquads-utf8/sha256/v2` | reject `ERR_UNKNOWN_SUITE` (§10): the tag is not a frozen v1 suite |

## 12. References

- [RFC 2119] Key words for use in RFCs to Indicate Requirement Levels.
- [RFC 8174] Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words.
- [RFC 3987] Internationalized Resource Identifiers (IRIs).
- [RFC 8785] JSON Canonicalization Scheme (JCS).
- [RDF-CANON] RDF Dataset Canonicalization (RDFC-1.0), W3C Recommendation.
- [JSON-LD11] JSON-LD 1.1, W3C Recommendation.
- [JSON-LD11-API] JSON-LD 1.1 Processing Algorithms and API, W3C
  Recommendation.
- [FIPS 180-4] Secure Hash Standard (SHS).
- [RFC 7595] Guidelines and Registration Procedures for URI Schemes
  (the planned provisional registration path for the `claims` scheme,
  §5.3).
- [RFC 6920] Naming Things with Hashes — considered and rejected for the
  reference form: its verification model is hash-of-retrieved-bytes,
  while this protocol's is hash-of-constructed-preimage (§6).
- `design/CONTENT_ADDRESSED_NAMING_ACADEMIC_REVIEW.md` — grounding for
  the origin-free reference form.
- `design/CLAIMS_ENGINE_DOMAIN_MODEL.md` (2026-09-12) — the domain model
  this specification normatively encodes.
- `spike/src/identity.rs`, `spike/src/l0.rs`, `spike/src/canon.rs` — the
  reference implementation whose semantics the v1 suites freeze.
