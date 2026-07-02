//! L1: schema validation as judgment (design doc §13).
//!
//! A validator evaluates an already-admitted claim against a schema version
//! and records the outcome as a validation claim — an ordinary claim admitted
//! back into L0 through the normal submission path. Validation is never an
//! admission gate.

use crate::l0::{AdmissionError, L0Store};
use crate::registry::SchemaRegistry;

/// IRI of the engine-published claim-type schema for validation outcomes.
pub const VALIDATION_RESULT_SCHEMA_IRI: &str =
    "https://claims.example/schema/validation-result/1.0.0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationOutcome {
    pub conforms: bool,
    pub violations: usize,
    /// ClaimIRI of the validation claim recording this judgment in L0.
    pub validation_claim_iri: String,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum L1Error {
    #[error("unknown claim: {0}")]
    UnknownClaim(String),
    #[error("schema version is not registered: {0}")]
    UnregisteredSchema(String),
    #[error("claim has no retained submitted material to judge")]
    NoSubmittedMaterial,
    #[error("submitted material is not JSON: {0}")]
    MaterialNotJson(String),
    #[error("json schema artifact is invalid: {0}")]
    BadSchemaArtifact(String),
    #[error("failed to admit validation claim: {0}")]
    Admission(#[from] AdmissionError),
}

/// An L1 validator service with its own stable identity. Its judgments are
/// attributed to it inside validation-claim content — user-space provenance,
/// exactly like any other claim (design doc §9).
#[derive(Clone, Debug)]
pub struct Validator {
    pub iri: String,
}

impl Validator {
    pub fn new(iri: &str) -> Self {
        Self {
            iri: iri.to_string(),
        }
    }

    /// Judge `claim_iri` against `schema_iri` and record the outcome as a
    /// validation claim. `validated_at` is assertion time and therefore lives
    /// in the validation claim's content (design doc §9).
    pub fn validate(
        &self,
        store: &mut L0Store,
        registry: &SchemaRegistry,
        claim_iri: &str,
        schema_iri: &str,
        validated_at: &str,
    ) -> Result<ValidationOutcome, L1Error> {
        if store.claim(claim_iri).is_none() {
            return Err(L1Error::UnknownClaim(claim_iri.to_string()));
        }

        let schema = registry
            .get(schema_iri)
            .ok_or_else(|| L1Error::UnregisteredSchema(schema_iri.to_string()))?;

        // Phase 1 (structural): JSON Schema over the authored form, i.e. the
        // retained submitted material minus its @context.
        let material = store
            .submissions_for(claim_iri)
            .first()
            .map(|record| record.material.clone())
            .ok_or(L1Error::NoSubmittedMaterial)?;
        let mut instance: serde_json::Value = serde_json::from_slice(&material)
            .map_err(|e| L1Error::MaterialNotJson(e.to_string()))?;
        if let Some(object) = instance.as_object_mut() {
            object.remove("@context");
        }

        let compiled = jsonschema::validator_for(&schema.json_schema)
            .map_err(|e| L1Error::BadSchemaArtifact(e.to_string()))?;
        let violations = compiled.iter_errors(&instance).count();
        let conforms = violations == 0;

        // Phase 2 (graph-level, e.g. SHACL over the canonical dataset) is
        // where a production validator would go deeper; out of scope here.

        // Record the judgment as an ordinary claim, admitted through the
        // normal submission path.
        let validation_registration = registry
            .get(VALIDATION_RESULT_SCHEMA_IRI)
            .ok_or_else(|| L1Error::UnregisteredSchema(VALIDATION_RESULT_SCHEMA_IRI.to_string()))?;
        let material = serde_json::json!({
            "@context": validation_registration.context["@context"],
            "type": "ValidationResult",
            "validator": self.iri,
            "target_claim": claim_iri,
            "schema_version": schema_iri,
            "outcome": if conforms { "conforms" } else { "violations" },
            "violations": violations,
            "validated_at": validated_at,
        });

        let admission = store.admit(
            material.to_string().as_bytes(),
            &self.iri,
            &[VALIDATION_RESULT_SCHEMA_IRI],
            validated_at,
        )?;

        Ok(ValidationOutcome {
            conforms,
            violations,
            validation_claim_iri: admission.claim_iri,
        })
    }
}
