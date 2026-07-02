//! L0 graph projection and trust-composition queries (design doc §17).
//!
//! The projection is a rebuildable view derived from accepted claims — never
//! the source of truth. It layers:
//!
//! - asserted-content projection: each claim's canonical content, in a named
//!   graph keyed by its ClaimIRI, blank nodes skolemized per claim;
//! - claim metadata projection (default graph): witnessed facts — declared
//!   schemas and fingerprints.

use std::collections::BTreeSet;

use sophia::api::ns::{rdf, Namespace};
use sophia::api::prelude::*;
use sophia::api::term::SimpleTerm;
use sophia::inmem::dataset::LightDataset;

use crate::l0::L0Store;

/// Namespace for engine metadata-projection terms.
pub const ENGINE_NS: &str = "https://claims.example/engine#";

/// Namespace of the validation-result claim-type schema terms.
pub const VALIDATION_RESULT_NS: &str = "https://claims.example/schema/validation-result/1.0.0/";

#[derive(Clone, Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("failed to re-parse canonical claim content: {0}")]
    Content(String),
    #[error("failed to build projection: {0}")]
    Build(String),
}

fn iri_term(value: &str) -> SimpleTerm<'static> {
    SimpleTerm::Iri(IriRef::new_unchecked(value.to_string().into()))
}

/// Skolemize blank nodes under the claim's IRI so canonical labels from
/// different claims cannot collide in the merged projection.
fn skolemize(term: &SimpleTerm<'_>, claim_iri: &str) -> SimpleTerm<'static> {
    match term {
        SimpleTerm::BlankNode(label) => iri_term(&format!("{claim_iri}#{}", label.as_str())),
        other => other.clone().into_term(),
    }
}

/// Build the L0 projection dataset from the authoritative claim record.
pub fn l0_projection(store: &L0Store) -> Result<LightDataset, ProjectionError> {
    let engine = Namespace::new_unchecked(ENGINE_NS);
    let declares_schema = engine.get("declaresSchema").unwrap();
    let has_fingerprint = engine.get("sha256Fingerprint").unwrap();

    let mut projection = LightDataset::new();

    for claim in store.claims() {
        let claim_term = iri_term(claim.iri());

        for schema_iri in claim.declared_schemas() {
            projection
                .insert(
                    &claim_term,
                    declares_schema,
                    iri_term(schema_iri),
                    None::<&SimpleTerm<'_>>,
                )
                .map_err(|e| ProjectionError::Build(e.to_string()))?;
        }
        projection
            .insert(
                &claim_term,
                has_fingerprint,
                claim.fingerprint(),
                None::<&SimpleTerm<'_>>,
            )
            .map_err(|e| ProjectionError::Build(e.to_string()))?;

        let content: LightDataset = sophia::turtle::parser::nq::parse_str(claim.canonical_nquads())
            .collect_quads()
            .map_err(|e| ProjectionError::Content(e.to_string()))?;

        for quad in content.quads() {
            let quad = quad.map_err(|e| ProjectionError::Content(e.to_string()))?;
            let ([s, p, o], _) = quad.spog();
            projection
                .insert(
                    skolemize(s, claim.iri()),
                    skolemize(p, claim.iri()),
                    skolemize(o, claim.iri()),
                    Some(&claim_term),
                )
                .map_err(|e| ProjectionError::Build(e.to_string()))?;
        }
    }

    Ok(projection)
}

/// Trust composition (design doc §13): claims declaring `schema_iri` that
/// have a "conforms" validation claim from `validator_iri` against that same
/// schema version. Consumers choose which validators to accept; the engine
/// only makes the join queryable.
pub fn conforming_claims(
    projection: &LightDataset,
    schema_iri: &str,
    validator_iri: &str,
) -> BTreeSet<String> {
    let vr = Namespace::new_unchecked(VALIDATION_RESULT_NS);
    let vr_type = vr.get("ValidationResult").unwrap();
    let vr_validator = vr.get("validator").unwrap();
    let vr_target = vr.get("target_claim").unwrap();
    let vr_schema = vr.get("schema_version").unwrap();
    let vr_outcome = vr.get("outcome").unwrap();
    let engine = Namespace::new_unchecked(ENGINE_NS);
    let declares_schema = engine.get("declaresSchema").unwrap();

    let schema_term = iri_term(schema_iri);
    let validator_term = iri_term(validator_iri);

    let judgments: Vec<SimpleTerm<'static>> = projection
        .quads_matching(Any, [rdf::type_], [vr_type], Any)
        .filter_map(Result::ok)
        .map(|quad| quad.s().clone().into_term())
        .collect();

    let mut result = BTreeSet::new();
    for judgment in judgments {
        let attributed = projection
            .quads_matching([&judgment], [vr_validator], [&validator_term], Any)
            .any(|q| q.is_ok());
        let against_schema = projection
            .quads_matching([&judgment], [vr_schema], [&schema_term], Any)
            .any(|q| q.is_ok());
        let conforms = projection
            .quads_matching([&judgment], [vr_outcome], Any, Any)
            .filter_map(Result::ok)
            .any(|quad| quad.o().lexical_form().as_deref() == Some("conforms"));
        if !(attributed && against_schema && conforms) {
            continue;
        }

        for quad in projection
            .quads_matching([&judgment], [vr_target], Any, Any)
            .filter_map(Result::ok)
        {
            let target = quad.o();
            let declared = projection
                .quads_matching([target], [declares_schema], [&schema_term], Any)
                .any(|q| q.is_ok());
            if declared {
                if let Some(iri) = target.iri() {
                    result.insert(iri.as_str().to_string());
                }
            }
        }
    }

    result
}
