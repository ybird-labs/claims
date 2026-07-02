//! Visualization artifacts: the L0 projection as N-Quads and as a
//! self-contained interactive HTML graph.
//!
//! Like every projection, these are disposable views rebuilt from the
//! authoritative claim record (design doc §17) — `cargo run` regenerates
//! them under `spike/out/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sophia::api::ns::{rdf, Namespace};
use sophia::api::prelude::*;
use sophia::api::term::SimpleTerm;
use sophia::turtle::serializer::nq::NqSerializer;

use crate::l0::L0Store;
use crate::l1::VALIDATION_RESULT_SCHEMA_IRI;
use crate::projection::{l0_projection, ProjectionError, VALIDATION_RESULT_NS};

const HTML_TEMPLATE: &str = include_str!("../assets/graph_template.html");
const METADATA_GRAPH_LABEL: &str = "engine metadata (default graph)";

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error(transparent)]
    Projection(#[from] ProjectionError),
    #[error("failed to serialize projection: {0}")]
    Serialize(String),
    #[error("failed to write artifact: {0}")]
    Io(#[from] std::io::Error),
}

/// Shorten an IRI for display: content-addressed IRIs become `claim:<hash6>`
/// (keeping any skolem fragment), others reduce to their last segment(s).
fn short_iri(iri: &str) -> String {
    if let Some(rest) = iri.strip_prefix("https://claims.example/claim/sha256/") {
        let (hash, fragment) = match rest.split_once('#') {
            Some((hash, fragment)) => (hash, Some(fragment)),
            None => (rest, None),
        };
        let hash = &hash[..hash.len().min(6)];
        return match fragment {
            Some(fragment) => format!("claim:{hash}#{fragment}"),
            None => format!("claim:{hash}"),
        };
    }
    // Schema *version* IRIs (…/schema/<name>/<version>) keep both segments;
    // schema-namespaced predicates fall through to the last segment.
    if let Some(rest) = iri.strip_prefix("https://claims.example/schema/") {
        if rest.matches('/').count() == 1 {
            return rest.to_string();
        }
    }
    let tail = iri.rsplit(['/', '#']).next().unwrap_or(iri);
    if tail.is_empty() {
        iri.to_string()
    } else {
        tail.to_string()
    }
}

/// Render a term for the inspector: IRIs shortened, literals with a short
/// datatype tag.
fn render_object(term: &SimpleTerm<'_>) -> (String, String) {
    if let Some(iri) = term.iri() {
        return (short_iri(iri.as_str()), iri.as_str().to_string());
    }
    let lexical = term
        .lexical_form()
        .map(|l| l.to_string())
        .unwrap_or_default();
    let datatype = term
        .datatype()
        .map(|dt| short_iri(dt.as_str()))
        .unwrap_or_default();
    let rendered = if datatype.is_empty() || datatype == "string" {
        format!("\"{lexical}\"")
    } else {
        format!("\"{lexical}\" ({datatype})")
    };
    let full = format!("\"{lexical}\"^^{datatype}");
    (rendered, full)
}

#[derive(Debug, Default)]
struct NodeBuild {
    kind: &'static str,
    status: Option<String>,
    type_label: Option<String>,
    triples: Vec<Value>,
}

/// Build the nodes/edges structure embedded in `graph.html`.
///
/// Node kinds are structural, verdict statuses are read from validation-claim
/// content — nothing is keyed to hardcoded claim IRIs.
pub fn graph_export(store: &L0Store) -> Result<Value, ExportError> {
    let projection = l0_projection(store)?;

    let vr = Namespace::new_unchecked(VALIDATION_RESULT_NS);
    let vr_type = vr.get("ValidationResult").unwrap();
    let vr_outcome = vr.get("outcome").unwrap();
    let vr_target = vr.get("target_claim").unwrap();

    // Verdict statuses per target claim, read from judgment content.
    let mut statuses: BTreeMap<String, String> = BTreeMap::new();
    let judgments: Vec<SimpleTerm<'static>> = projection
        .quads_matching(Any, [rdf::type_], [vr_type], Any)
        .filter_map(Result::ok)
        .map(|quad| quad.s().clone().into_term())
        .collect();
    for judgment in &judgments {
        let outcome = projection
            .quads_matching([judgment], [vr_outcome], Any, Any)
            .filter_map(Result::ok)
            .find_map(|quad| quad.o().lexical_form().map(|l| l.to_string()));
        let target = projection
            .quads_matching([judgment], [vr_target], Any, Any)
            .filter_map(Result::ok)
            .find_map(|quad| quad.o().iri().map(|iri| iri.as_str().to_string()));
        if let (Some(outcome), Some(target)) = (outcome, target) {
            statuses.insert(target, outcome);
        }
    }

    let mut builds: BTreeMap<String, NodeBuild> = BTreeMap::new();
    let mut edges: Vec<Value> = Vec::new();

    // Claim nodes: kind from the declared schema, status from verdicts.
    for claim in store.claims() {
        let is_verdict = claim
            .declared_schemas()
            .contains(VALIDATION_RESULT_SCHEMA_IRI);
        let build = builds.entry(claim.iri().to_string()).or_default();
        build.kind = if is_verdict { "verdict" } else { "claim" };
        if !is_verdict {
            build.status = Some(
                statuses
                    .get(claim.iri())
                    .cloned()
                    .unwrap_or_else(|| "unjudged".to_string()),
            );
        }
        for schema_iri in claim.declared_schemas() {
            builds.entry(schema_iri.clone()).or_default().kind = "schema";
            edges.push(json!({
                "source": claim.iri(),
                "target": schema_iri,
                "label": "declaresSchema",
                "kind": "metadata",
            }));
        }
    }

    // Content nodes, edges, and inspector triples from the named graphs;
    // metadata triples from the default graph.
    let mut objects_in_graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut subjects_in_graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for quad in projection.quads() {
        let quad = quad.map_err(|e| ExportError::Serialize(e.to_string()))?;
        let ([s, p, o], g) = quad.spog();
        let Some(subject_iri) = s.iri().map(|iri| iri.as_str().to_string()) else {
            continue;
        };
        let predicate_iri = p
            .iri()
            .map(|iri| iri.as_str().to_string())
            .unwrap_or_default();
        let (graph_label, graph_full, in_content) = match g {
            Some(graph) => {
                let iri = graph
                    .iri()
                    .map(|iri| iri.as_str().to_string())
                    .unwrap_or_default();
                (short_iri(&iri), iri, true)
            }
            None => (
                "metadata".to_string(),
                METADATA_GRAPH_LABEL.to_string(),
                false,
            ),
        };

        if in_content {
            let graph_iri = graph_full.clone();
            let build = builds.entry(subject_iri.clone()).or_default();
            if build.kind.is_empty() {
                // Structural rule: skolemized nodes live under a claim IRI
                // fragment; everything else is an external entity.
                build.kind = if subject_iri.contains("/claim/sha256/") && subject_iri.contains('#')
                {
                    "statement"
                } else {
                    "entity"
                };
            }
            subjects_in_graph
                .entry(graph_iri.clone())
                .or_default()
                .insert(subject_iri.clone());

            if rdf::type_ == p {
                if let Some(class_iri) = o.iri() {
                    builds.entry(subject_iri.clone()).or_default().type_label =
                        Some(short_iri(class_iri.as_str()));
                }
            } else if let Some(object_iri) = o.iri() {
                let object_iri = object_iri.as_str().to_string();
                let object_build = builds.entry(object_iri.clone()).or_default();
                if object_build.kind.is_empty() {
                    object_build.kind =
                        if object_iri.contains("/claim/sha256/") && object_iri.contains('#') {
                            "statement"
                        } else {
                            "entity"
                        };
                }
                objects_in_graph
                    .entry(graph_iri.clone())
                    .or_default()
                    .insert(object_iri.clone());
                edges.push(json!({
                    "source": subject_iri,
                    "target": object_iri,
                    "label": short_iri(&predicate_iri),
                    "kind": "content",
                }));
            }
        }

        let (object_rendered, object_full) = render_object(o);
        builds.entry(subject_iri).or_default().triples.push(json!({
            "p": short_iri(&predicate_iri),
            "pFull": predicate_iri,
            "o": object_rendered,
            "oFull": object_full,
            "g": graph_label,
            "gFull": graph_full,
        }));
    }

    // Synthetic "asserts" edges: each claim points at the root subjects of
    // its own named graph, so a claim connects visually to its content.
    for (graph_iri, subjects) in &subjects_in_graph {
        let objects = objects_in_graph.get(graph_iri);
        for subject in subjects {
            let is_root = objects.is_none_or(|set| !set.contains(subject));
            if is_root && subject != graph_iri {
                edges.push(json!({
                    "source": graph_iri,
                    "target": subject,
                    "label": "asserts",
                    "kind": "asserts",
                }));
            }
        }
    }

    let nodes: Vec<Value> = builds
        .into_iter()
        .map(|(id, build)| {
            let kind = if build.kind.is_empty() {
                "entity"
            } else {
                build.kind
            };
            let label = match kind {
                "claim" => format!("Claim {}", &short_iri(&id)["claim:".len()..]),
                "verdict" => format!("Verdict {}", &short_iri(&id)["claim:".len()..]),
                _ => build.type_label.clone().unwrap_or_else(|| short_iri(&id)),
            };
            json!({
                "id": id,
                "label": label,
                "kind": kind,
                "status": build.status.unwrap_or_else(|| "none".to_string()),
                "triples": build.triples,
            })
        })
        .collect();

    Ok(json!({ "nodes": nodes, "edges": edges }))
}

/// The self-contained interactive HTML document.
pub fn graph_html(store: &L0Store) -> Result<String, ExportError> {
    let data = graph_export(store)?;
    Ok(HTML_TEMPLATE.replace("__GRAPH_DATA__", &data.to_string()))
}

/// The full projection serialized as N-Quads.
pub fn projection_nquads(store: &L0Store) -> Result<String, ExportError> {
    let projection = l0_projection(store)?;
    let mut serializer = NqSerializer::new_stringifier();
    serializer
        .serialize_dataset(&projection)
        .map_err(|e| ExportError::Serialize(e.to_string()))?;
    Ok(serializer.as_str().to_string())
}

/// Write `projection.nq` and `graph.html` under `out_dir`, returning their
/// paths.
pub fn write_artifacts(store: &L0Store, out_dir: &Path) -> Result<(PathBuf, PathBuf), ExportError> {
    std::fs::create_dir_all(out_dir)?;

    let nquads_path = out_dir.join("projection.nq");
    std::fs::write(&nquads_path, projection_nquads(store)?)?;

    let html_path = out_dir.join("graph.html");
    std::fs::write(&html_path, graph_html(store)?)?;

    Ok((nquads_path, html_path))
}
