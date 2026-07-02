//! SPARQL-queryable projection: the L0 projection loaded into an in-memory
//! Oxigraph store.
//!
//! Provisional answer to design doc §20.10 (physical/query representation of
//! projections). The store is rebuilt from the authoritative claim record on
//! every [`SparqlProjection::load`] and nothing is persisted: claims remain
//! the only source of truth, the store is a disposable view (§17).

use std::collections::BTreeSet;

use oxigraph::io::RdfFormat;
use oxigraph::model::Term;
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use oxigraph::store::Store;
use sophia::api::prelude::*;
use sophia::turtle::serializer::nq::NqSerializer;

use crate::l0::L0Store;
use crate::projection::{l0_projection, ProjectionError, ENGINE_NS, VALIDATION_RESULT_NS};

#[derive(Debug, thiserror::Error)]
pub enum GraphDbError {
    #[error(transparent)]
    Projection(#[from] ProjectionError),
    #[error("failed to serialize projection to N-Quads: {0}")]
    Serialize(String),
    #[error("graph store error: {0}")]
    Store(String),
    #[error("SPARQL evaluation failed: {0}")]
    Query(String),
    #[error("unexpected SPARQL result shape: {0}")]
    ResultShape(String),
}

/// In-memory graph database over the L0 projection.
pub struct SparqlProjection {
    store: Store,
}

impl SparqlProjection {
    /// Rebuild the projection from the claim record and load it into a fresh
    /// in-memory store (metadata layer in the default graph, per-claim
    /// content in named graphs, exactly as [`l0_projection`] lays it out).
    pub fn load(claims: &L0Store) -> Result<Self, GraphDbError> {
        let projection = l0_projection(claims)?;

        let mut serializer = NqSerializer::new_stringifier();
        serializer
            .serialize_dataset(&projection)
            .map_err(|e| GraphDbError::Serialize(e.to_string()))?;

        let store = Store::new().map_err(|e| GraphDbError::Store(e.to_string()))?;
        store
            .load_from_reader(RdfFormat::NQuads, serializer.as_str().as_bytes())
            .map_err(|e| GraphDbError::Store(e.to_string()))?;

        Ok(Self { store })
    }

    /// Run a SPARQL SELECT and collect the IRI bindings of variable `var`.
    pub fn select_iris(&self, query: &str, var: &str) -> Result<BTreeSet<String>, GraphDbError> {
        let mut iris = BTreeSet::new();
        for solution in self.solutions(query)? {
            if let Some(Term::NamedNode(node)) = solution.get(var) {
                iris.insert(node.as_str().to_string());
            }
        }
        Ok(iris)
    }

    /// Run a SPARQL SELECT expected to bind `var` to one integer literal.
    pub fn select_integer(&self, query: &str, var: &str) -> Result<i64, GraphDbError> {
        let solutions = self.solutions(query)?;
        let solution = solutions.into_iter().next().ok_or_else(|| {
            GraphDbError::ResultShape("expected at least one solution".to_string())
        })?;
        match solution.get(var) {
            Some(Term::Literal(literal)) => literal
                .value()
                .parse::<i64>()
                .map_err(|e| GraphDbError::ResultShape(e.to_string())),
            other => Err(GraphDbError::ResultShape(format!(
                "expected integer literal for ?{var}, got {other:?}"
            ))),
        }
    }

    fn solutions(&self, query: &str) -> Result<Vec<oxigraph::sparql::QuerySolution>, GraphDbError> {
        let results = SparqlEvaluator::new()
            .parse_query(query)
            .map_err(|e| GraphDbError::Query(e.to_string()))?
            .on_store(&self.store)
            .execute()
            .map_err(|e| GraphDbError::Query(e.to_string()))?;
        let QueryResults::Solutions(solutions) = results else {
            return Err(GraphDbError::ResultShape(
                "expected SELECT solutions".to_string(),
            ));
        };
        solutions
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| GraphDbError::Query(e.to_string()))
    }
}

/// The trust-composition query (design doc §13) as standard SPARQL: claims
/// declaring `schema_iri` with a "conforms" verdict from `validator_iri`
/// against that same schema version. Verdicts are asserted content inside
/// validation-claim named graphs; the `declaresSchema` join is against the
/// witnessed metadata layer in the default graph.
pub fn trust_composition_query(schema_iri: &str, validator_iri: &str) -> String {
    format!(
        r#"PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX vr: <{VALIDATION_RESULT_NS}>
PREFIX ce: <{ENGINE_NS}>
SELECT DISTINCT ?claim WHERE {{
  GRAPH ?verdict {{
    ?judgment rdf:type vr:ValidationResult ;
              vr:validator <{validator_iri}> ;
              vr:schema_version <{schema_iri}> ;
              vr:outcome "conforms" ;
              vr:target_claim ?claim .
  }}
  ?claim ce:declaresSchema <{schema_iri}> .
}}"#
    )
}

/// One query composing trust with claim content across projection layers:
/// the sum of `property_iri` values found inside the named graphs of exactly
/// the trusted claims.
pub fn trusted_total_query(schema_iri: &str, validator_iri: &str, property_iri: &str) -> String {
    format!(
        r#"PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX vr: <{VALIDATION_RESULT_NS}>
PREFIX ce: <{ENGINE_NS}>
SELECT (SUM(?value) AS ?total) WHERE {{
  GRAPH ?verdict {{
    ?judgment rdf:type vr:ValidationResult ;
              vr:validator <{validator_iri}> ;
              vr:schema_version <{schema_iri}> ;
              vr:outcome "conforms" ;
              vr:target_claim ?claim .
  }}
  ?claim ce:declaresSchema <{schema_iri}> .
  GRAPH ?claim {{ ?subject <{property_iri}> ?value }}
}}"#
    )
}
