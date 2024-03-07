use anyhow::Result;
use bitcoin::OutPoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InscriptionInfo {
    pub(crate) outpoint: OutPoint,
    pub(crate) amount: u64,
}

pub(crate) trait InscriptionIdResolver {
    /// Given an inscription_id, returns info about the inscription including the outpoint and amount
    fn resolve_inscription_id(&self, inscription_id: &str) -> Result<InscriptionInfo>;
}

/// Resolves inscription ids by looking them up in a static map. Can be read/written to a file.
pub(crate) struct StaticInscriptionIdResolver {
    map: HashMap<String, InscriptionInfo>,
}

impl StaticInscriptionIdResolver {
    pub fn from_json_file(path: &str) -> Result<Self> {
        let map = std::fs::read_to_string(path)?;
        let map: HashMap<String, InscriptionInfo> = serde_json::from_str(&map)?;
        Ok(Self { map })
    }
}

impl InscriptionIdResolver for StaticInscriptionIdResolver {
    fn resolve_inscription_id(&self, inscription_id: &str) -> Result<InscriptionInfo> {
        self.map
            .get(inscription_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Inscription id not found"))
    }
}

pub(crate) struct OrdServerInscriptionIdResolver {
    url: String,
}

impl OrdServerInscriptionIdResolver {
    pub(crate) fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

impl InscriptionIdResolver for OrdServerInscriptionIdResolver {
    fn resolve_inscription_id(&self, inscription_id: &str) -> Result<InscriptionInfo> {
        let response: Value =
            reqwest::blocking::get(format!("{}/inscriptions/{}", self.url, inscription_id))?
                .json()?;

        // Extract the output_value and satpoint fields
        let output_value = response["output_value"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("output_value not found"))?;
        let satpoint = response["satpoint"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("satpoint not found"))?;

        let outpoint = satpoint_to_outpoint(satpoint)?;

        Ok(InscriptionInfo {
            outpoint,
            amount: output_value,
        })
    }
}

fn satpoint_to_outpoint(satpoint: &str) -> Result<OutPoint> {
    let satpoint = satpoint.split(":").take(2).collect::<Vec<&str>>().join(":");
    let outpoint = OutPoint::from_str(&satpoint)?;
    Ok(outpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::OutPoint;

    #[test]
    fn test_satpoint_split() {
        let satpoint = "5fddcbdc3eb21a93e8dd1dd3f9087c3677f422b82d5ba39a6b1ec37338154af6:0:0";
        let expected = OutPoint::from_str(
            "5fddcbdc3eb21a93e8dd1dd3f9087c3677f422b82d5ba39a6b1ec37338154af6:0",
        )
        .unwrap();

        let actual = satpoint_to_outpoint(satpoint).unwrap();

        assert_eq!(
            actual, expected,
            "satpoint_to_outpoint did not produce expected result"
        );
    }
}
