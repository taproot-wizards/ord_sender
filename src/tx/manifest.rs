use bitcoin::{OutPoint};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Manifest {
    pub(crate) fee_rate: u64,
    pub(crate) funding_outpoint: Option<OutPoint>,
    pub(crate) anchor_address: String,
    pub(crate) transfers: Vec<Transfer>,
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transfer {
    /// the inscripiton id of the ordinal to transfer. If not provided, you must provide the outpoint
    pub(crate) inscription_id: Option<String>,
    /// the outpoint of the ordinal to transfer. If not provided, you must provide an inscription id
    pub(crate) outpoint: Option<OutPoint>,
    /// The address to send the ordinal to
    pub(crate) address: String,
    pub(crate) amount: u64,
}

impl Manifest {
    pub(crate) fn from_json_file(file_name: &str) -> Result<Self> {
        let manifest = std::fs::read_to_string(file_name)?;
        let manifest: Self = serde_json::from_str(&manifest)?;
        Ok(manifest)
    }
}