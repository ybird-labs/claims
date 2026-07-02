# Claims Engine spike (throwaway)

A deliberately disposable Rust spike proving the domain model in
`design/CLAIMS_ENGINE_DOMAIN_MODEL.md` (2026-07-02) end-to-end with the
carbon-verification example.

**This is not production code.** It is an isolated cargo project (own
`Cargo.toml` with an empty `[workspace]` table, own `Cargo.lock`), outside the
root workspace, exempt from CI and from `AGENTS.md` module rules. Delete it
with `rm -rf spike/` once its learnings are ported into `crates/claims`.

## Run

```sh
cd spike
cargo run            # narrated demo; asserts each proof, exits 0 on success
cargo run -- carbon  # carbon-project stress test (real data); proofs C1–C6
cargo test           # unit + end-to-end tests, including one test per C-proof
```

## What it proves

1. **Content addressing and idempotent admission.** Claims authored as
   JSON-LD are canonicalized with RDFC-1.0 (`sophia_c14n`) and
   content-addressed. Two differently-ordered JSON documents of the same
   claim (`fixtures/material_good_a.jsonld` vs `_b.jsonld`) yield the same
   ClaimIRI; resubmission is idempotent — same claim, new witnessed
   submission record with a different raw-material fingerprint.
2. **L0 enforces only the base-schema floor.** Empty content and missing
   schema declarations are rejected; a claim that will *fail* its declared
   schema is admitted. Admission is not conformance.
3. **L1 validation is a judgment.** A validator checks an admitted claim
   against a registered schema version's artifacts and records the outcome
   as a validation claim — an ordinary claim admitted back through the
   normal submission path. Failing claims coexist with their verdicts.
4. **Trust composition.** A query over the L0 projection (content projection
   in named graphs per claim, metadata projection in the default graph)
   returns claims declaring the carbon schema that have a "conforms" verdict
   from a chosen validator — the good claim survives, the bad one doesn't.
5. **Deterministic snapshots.** A snapshot over the trusted claim set has an
   order-independent fingerprint and a derived SnapshotIRI.
6. **The projection loads into a real graph database.** The L0 projection is
   loaded into an in-memory Oxigraph store (built without RocksDB — nothing
   touches disk) and queried with standard SPARQL 1.1. The trust-composition
   query re-expressed as SPARQL returns exactly the same result set as the
   programmatic query, and a single SPARQL query composes trust with claim
   content across projection layers — the total verified tons of CO₂ over
   exactly the trusted claims:

   ```sparql
   SELECT (SUM(?value) AS ?total) WHERE {
     GRAPH ?verdict {
       ?judgment rdf:type vr:ValidationResult ;
                 vr:validator <...> ;
                 vr:schema_version <...> ;
                 vr:outcome "conforms" ;
                 vr:target_claim ?claim .
     }
     ?claim ce:declaresSchema <...> .          # witnessed metadata layer
     GRAPH ?claim { ?s csv:tons_co2 ?value }   # asserted content layer
   }
   ```

   The store is rebuilt from the claim record on every construction; claims
   remain the only source of truth (see `src/graphdb.rs`).

## Carbon-project stress test (`cargo run -- carbon`)

A second demo drives the same unmodified engine with real data from a live
carbon-credit registration review (ybird-labs/carbon-project): two real
sites — `01.0035.00035` (CZ, 5 plots) and `02.4368.00441` (SK, 2 plots) —
and four numbered registration requirements (SL-003 land tenure, SL-006
homogeneity, SL-007 project start date, SL-009 historic activity), whose
real outcomes span `satisfied`, `not_satisfied`, and `unclear`.

Fixtures live in `fixtures_carbon/` and are generated, never hand-invented:

```sh
python3 tools/gen_carbon_fixtures.py /path/to/carbon-project
```

The generator reads `progress/site_index.json` and two per-site progress
files, emits deterministic JSON (byte-reproducible from the same checkout),
pseudonymizes landholder names (keys, dates, land-use values, acreages, and
LPIS/GSAA plot registration IDs stay real), and self-checks that no real
name leaks into any output. Airtable record IDs — retrieval coordinates —
are documented in `fixtures_carbon/manifest.json` only, never claim content.

The proofs:

1. **C1 — real entities, unchanged floor.** Site/plot evidence claims with
   entity IRIs minted from real farm/plot keys admit through the same L0
   floor; resubmission stays idempotent, every submission witnessed.
2. **C2 — derivation chains.** The site project start date is not an
   authoritative source field (the review derives it from the earliest
   active soil-sampling date). Here that is a *derivation claim* whose
   content cites its input ClaimIRIs (`derived_from`) and the rule IRI —
   structured, walkable provenance instead of prose notes. The demo walks
   from the derivation back to its inputs via the L0 projection.
3. **C3 — tri-state, multi-evidence judgments are ordinary claims.**
   Requirement judgments (`satisfied`/`not_satisfied`/`unclear`, ≥2 cited
   evidence ClaimIRIs each) admit as ordinary claims under a user-space
   claim-type schema — and unmodified L1 validates them against it. The
   engine's own validation vocabulary stays binary.
4. **C4 — trust composes through claim chains.** One SPARQL query follows
   satisfied SL-003 judgments (asserted content) → their cited evidence
   claims (via the witnessed `declaresSchema` metadata layer) → the
   evidence claims' asserted plot registration IDs, across named graphs,
   and matches the programmatic expectation exactly.
5. **C5 — normative rule versions are content.** Re-issuing a judgment
   with only `credit_class_version` bumped yields a different ClaimIRI;
   both coexist; SPARQL filters judgments by version.
6. **C6 — snapshots.** The snapshot over all carbon claims has an
   order-independent fingerprint.

## Visualize

`cargo run` writes two artifacts to `spike/out/` (gitignored) and prints
their paths:

- **`graph.html`** — a self-contained interactive view of the L0 projection:
  force-directed layout, pan/zoom, draggable nodes; click any node for an
  inspector listing its triples and the named graph (claim) each came from.
  Claim nodes are colored by verdict status (✓ conforms / ✕ violations /
  ? unjudged — derived from validation-claim content, not hardcoded);
  validation claims, statements, entities, and schema versions are visually
  distinct; metadata edges (`declaresSchema`) are dashed. Everything is
  inline — open it in any browser, no server or network needed.
- **`projection.nq`** — the projection as N-Quads, for external tools.

Optional SPARQL workbench (interactive queries in the browser, locally):

```sh
cargo install oxigraph-cli
oxigraph load --location /tmp/claims-db --file out/projection.nq
oxigraph serve --location /tmp/claims-db
# then open http://localhost:7878 — YASGUI query UI
```

## Provisional choices for design doc §20 (open items)

| §20 item | Choice made here |
|---|---|
| 1. Canonicalization profile | RDFC-1.0 as implemented by `sophia_c14n` 0.9 (SHA-256 bnode hashing) |
| 2. Digest suite / IRI encoding | SHA-256, lowercase hex, `https://claims.example/{claim,snapshot}/sha256/<hex>`; preimages are suite-tagged strings (see `src/identity.rs`) |
| 3. Dangling schema declarations | Allowed at admission (only well-formed-IRI checked); L1 refuses to *judge* against an unregistered schema |
| 4. Schemas as claims | No — side registry (`src/registry.rs`) |
| 5. Schema declaration placement | Envelope-level, part of the fingerprint preimage (avoids the content-addressing self-reference circularity) |
| 6. Validation-claim shape | `fixtures/validation-result.schema.json` — validator, target_claim, schema_version, outcome, violations, validated_at |
| 7. SnapshotIRI generation | Derived from the membership fingerprint, same scheme as ClaimIRIs |
| 8. External/source identifier placement | Decomposed: entity identifiers (farm/plot keys) are claim content as subject IRIs; retrieval coordinates (which Airtable row / workbook produced the bytes) are witnessed submission/audit facts, never content; richer origin semantics are optional user-space PROV-style content. Exercised by the carbon demo (C1; `fixtures_carbon/manifest.json`) |
| 10. Projection query representation | In-memory Oxigraph store (no RocksDB) loaded from the L0 projection; queried with SPARQL 1.1; rebuilt per construction, never persisted |

## Learnings from a real domain

Mapping a live carbon-credit registration review onto the model resolved
five questions, all the same way: **no new engine primitives** — everything
lands in user-defined claim-type schemas and content, which is the model's
core premise doing its job.

1. **Richer verdict vocabularies are user space.** The engine's
   validation-claim vocabulary stays `conforms | violations` (mechanical
   schema checks). Judgments like `satisfied | not_satisfied | unclear` —
   including "reviewed but evidence insufficient", which real review
   workflows treat as first-class — are user-space judgment claim-type
   schemas, typically rule-based, layered on top as ordinary claims.
2. **Source identifiers decompose** (resolves §20 item 8): entity
   identifiers into content as subject IRIs, retrieval coordinates into the
   witnessed SubmissionRecord, richer origin semantics into optional
   user-space PROV-style content.
3. **Multi-evidence judgments need no engine change.** L1 validation stays
   single-target; a judgment over many claims is a user-space claim whose
   content lists evidence ClaimIRIs (§16 relationships). Claims citing
   claims by IRI becomes a load-bearing pattern — trust composes through
   chains (C4).
4. **Normative rule versions are content, not schema versions.** A
   credit-class version governs what a verdict *means* without changing its
   payload shape. Per the identity rule (§8) it goes in content: the same
   judgment under two rule versions is two distinct content-addressed
   claims (C5). Claim-type schema versions (engine machinery) and normative
   rule versions (asserted content) are different things; conflating them
   is an easy category error.
5. **Derived values are claims with structured derivation provenance.**
   Where a source system lacks an authoritative field (the review derives
   site start dates from sampling records), the derivation is a claim
   carrying `derived_from` input ClaimIRIs and the rule identity — instead
   of the caveat living in prose notes, as it does in the source repo (C2).

## Deliberate simplifications

- **Schema artifacts are hand-authored fixtures** (`fixtures/*.context.jsonld`,
  `fixtures/*.schema.json`), written as-if generated from a LinkML source.
  LinkML's toolchain is Python-only; compilation was always an offline
  authoring step, not engine code, so the spike starts from its outputs.
- **L1 phase 2 (SHACL over the canonical graph) is stubbed out.** Phase 1
  (JSON Schema over the authored form) carries the judgment here. The design
  allows out-of-process validators, where pySHACL/LinkML run natively.
- **`accepted_at` is caller-supplied** instead of read from a real clock, so
  demo output (all IRIs and fingerprints) is deterministic run-to-run.
- In-memory stores, no persistence, no API layer, minimal error taxonomy.
- Submitted material must be *self-contained* JSON-LD: the parser uses
  sophia's `NoLoader`, so a document referencing a remote context fails
  instead of being fetched (the pinned-context rule, design doc §10).
