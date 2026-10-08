# Spike Learnings and Work Plan

Date: 2026-10-08
Status: **Agreed**

This document closes out the `spike/` codebase. It records what the spike
taught us, graded by how well each lesson is supported, and the agreed
plan for the work that follows. The spike is never merged to `main`; it is
preserved at git tag `spike-archive-2026-09`.

Sources: `spike/` (code, tests, README), `design/CLAIMS_PROTOCOL.md`
(2026-09-12 draft), `design/FEDERATION_PROTOCOL_DECISION_LOG.md`.

## 1. Learnings

### 1.1 Shown by running code

These hold in `spike/` today, with tests and vectors.

1. **A Claim is its declared schemas plus canonical content, nothing
   else.** Two authored forms of the same statement produce the same
   ClaimIRI (protocol §11.1, V-CLAIM-1).
2. **The identity math works as specified.** Suite-tagged preimage,
   RDFC-1.0 canonical N-Quads, SHA-256, lowercase hex. The spec's vectors
   are regenerated from `spike/examples/print_vectors.rs`.
3. **Snapshots are an unordered set of ClaimIRIs.** Membership order does
   not change the SnapshotIRI (V-SNAP-1).
4. **Validation results are ordinary claims.** The engine's verdict
   vocabulary is `conforms | violations` and nothing more.
5. **A real domain needed no new engine primitives.** Mapping a live
   carbon-credit registration review onto the model (spike README,
   "Learnings from a real domain") put richer verdicts, multi-evidence
   judgments, normative rule versions, and derived values all in
   user-defined claim types and content.

### 1.2 Decided in the spec, not shown by code

The spike predates these decisions. Its code still reflects the earlier
model.

6. **Names use the `claims:` scheme** (protocol §5.3). The spike mints
   `https://claims.example/...` names.
7. **The engine is transparent.** No submission fact enters a Claim's
   value; logging is an operator concern (protocol §4.1). The spike still
   carries a witnessed `SubmissionRecord` layer.
8. **Verdicts cite the material they judged** (`judged_material`,
   protocol §8). The spike does not implement it (decision log §6): it
   judges an arbitrary authored form and records none.
9. **Attestation is the only proof** (protocol §9). Only the boundary is
   specified. No attestation suite or anchoring code exists, so today the
   system can prove *what* a value is, not *when* it existed.

### 1.3 Never tested

10. **The real LinkML schema pipeline.** Every spike schema artifact is a
    hand-written imitation of LinkML output (spike README, "Deliberate
    simplifications"). The decision log (§7) calls this the least-proven
    link, and schema identity depends on generated artifacts being
    reproducible.

## 2. Work plan

| # | Step | Delivers |
|---|---|---|
| 1 | Merge PR #10: the spec, domain model, and this plan (design docs only; the spike stays at the archive tag) | spec, domain model, and plan on `main` |
| 2 | Port slice 0: write `AGENTS.md` for the new model (needs explicit approval) | guardrails match the spec |
| 3 | Port slice 1: rewrite the domain identity core in `crates/claims`, test-first, to the spec | learnings 1, 2, 6 in production code |
| 4 | Port slices 2–5: admission; schema registry and L1 validation with `judged_material`; projection; snapshots | proves learnings 7 and 8 |
| 5 | Run LinkML for real on one schema | tests learning 10 |
| 6 | Write the first attestation suite (Regen x/data anchor is the candidate) | first working instance of learning 9 |
| — | Federation (decision log D3–D8) | parked until a real counterparty exists |

The spike's vector generator covers V-CLAIM-1 to 3, V-SNAP-1, and the
§8 example only. V-CLAIM-4 to 7 are not added to it: they matter only
once a second implementation exists, so they become tests in
`crates/claims` during the port (step 4, admission slice). The spike is
not patched. Its canonicalizer merges case-variant literals (protocol
§11.6), the bug V-CLAIM-5 catches, so that vector's values come from the
normative text.

Step 5 does not depend on the port and can run alongside it. Running it
early means any surprise in schema identity lands before the registry
slice in step 4.

## 3. Naming decisions recorded with this plan

- **One name form.** The canonical hex name in protocol §5.3 is the only
  form used for identity, comparison, and RDF content. No aliases.
- **Compact encoding deferred.** A base58check encoding of the same
  fingerprint, for places where people handle names (QR codes, pasted
  IDs), may be added later as a non-identity convention like the §5.4
  gateways. Tools would convert it to the canonical name before any use.
  It waits for a concrete need.
- **Base58 does not give Regen compatibility.** Regen's IRIs hash content
  alone with BLAKE2b; ours hash schemas plus content with SHA-256. The
  Regen connection belongs in the attestation suite (step 6), which
  anchors our fingerprint on their ledger without changing our names.
