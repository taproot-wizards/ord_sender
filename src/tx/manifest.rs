use anyhow::Result;
use bitcoin::OutPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Manifest {
    pub(crate) fee_rate: u64,
    pub(crate) funding_outpoint: Option<OutPoint>,
    /// address for change. there will always be at least 546 sats of change, so you can use it as an anchor for CPFP
    pub(crate) change_address: String,
    pub(crate) transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transfer {
    /// the inscripiton id of the ordinal to transfer. Will be resolved to an outpoint when the transaction is built
    pub(crate) inscription_id: String,
    /// The address to send the ordinal to
    pub(crate) address: String,
}

impl Manifest {
    pub(crate) fn from_json_file(file_name: &str) -> Result<Self> {
        let manifest = std::fs::read_to_string(file_name)?;
        let manifest: Self = serde_json::from_str(&manifest)?;
        Ok(manifest)
    }
    pub(crate) fn to_json_file(&self, file_name: &str) -> Result<()> {
        let manifest = serde_json::to_string_pretty(self)?;
        std::fs::write(file_name, manifest)?;
        Ok(())
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            fee_rate: 1,
            funding_outpoint: None,
            change_address: "".to_string(),
            transfers: vec![Transfer {
                inscription_id: "".to_string(),
                address: "".to_string(),
            }],
        }
    }
}
