//! Registered schema versions (design doc §6.2).
//!
//! A schema version is an immutable artifact with a canonical IRI, carrying
//! the compiled validation artifacts. In production these would be generated
//! from a LinkML source; the spike ships them as hand-authored fixtures.
//! Provisional choice for §20.4: registrations are a side registry here, not
//! claims themselves.

use std::collections::BTreeMap;

/// Immutable registered schema version with its compiled artifacts.
#[derive(Clone, Debug)]
pub struct SchemaVersion {
    pub iri: String,
    /// Pinned JSON-LD context (as-if LinkML-generated). Used by authoring
    /// tools; never fetched remotely by the engine (design doc §10).
    pub context: serde_json::Value,
    /// JSON Schema artifact (as-if LinkML-generated), used by L1.
    pub json_schema: serde_json::Value,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum RegistryError {
    #[error("schema version already registered (versions are immutable): {0}")]
    AlreadyRegistered(String),
}

/// Immutable-once-registered schema version store (§18 invariant 10).
#[derive(Debug, Default)]
pub struct SchemaRegistry {
    versions: BTreeMap<String, SchemaVersion>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, version: SchemaVersion) -> Result<(), RegistryError> {
        if self.versions.contains_key(&version.iri) {
            return Err(RegistryError::AlreadyRegistered(version.iri));
        }
        self.versions.insert(version.iri.clone(), version);
        Ok(())
    }

    pub fn get(&self, iri: &str) -> Option<&SchemaVersion> {
        self.versions.get(iri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(iri: &str) -> SchemaVersion {
        SchemaVersion {
            iri: iri.to_string(),
            context: serde_json::json!({}),
            json_schema: serde_json::json!({}),
        }
    }

    #[test]
    fn registered_versions_are_immutable() {
        let mut registry = SchemaRegistry::new();
        let iri = "https://claims.example/schema/test/1.0.0";

        registry.register(version(iri)).unwrap();
        let err = registry.register(version(iri)).unwrap_err();

        assert_eq!(err, RegistryError::AlreadyRegistered(iri.to_string()));
    }
}
