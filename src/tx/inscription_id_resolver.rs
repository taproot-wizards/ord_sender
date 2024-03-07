use anyhow::Result;
use bitcoin::OutPoint;
use std::collections::HashMap;

pub(crate) trait InscriptionIdResolver {
    /// Given an inscription_id, returns the outpoint where it currently resides.
    fn resolve_inscription_id(&self, inscription_id: &str) -> Result<OutPoint>;
}

/// Resolves inscription ids by looking them up in a static map. Can be read/written to a file.
pub(crate) struct StaticInscriptionIdResolver {
    map: HashMap<String, OutPoint>,
}

impl StaticInscriptionIdResolver {
    pub fn from_json_file(path: &str) -> Result<Self> {
        let map = std::fs::read_to_string(path)?;
        let map: HashMap<String, OutPoint> = serde_json::from_str(&map)?;
        Ok(Self { map })
    }
}

impl InscriptionIdResolver for StaticInscriptionIdResolver {
    fn resolve_inscription_id(&self, inscription_id: &str) -> Result<OutPoint> {
        self.map
            .get(inscription_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Inscription id not found"))
    }
}
