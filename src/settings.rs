use crate::tx::dummy_witness::WalletType;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) network: bitcoin::Network,
    pub(crate) wallet_type: WalletType,
}

pub fn from_json_file(file_name: &str) -> Result<Settings> {
    let settings = std::fs::read_to_string(file_name)?;
    let settings: Settings = serde_json::from_str(&settings)?;
    Ok(settings)
}

pub fn to_json_file(settings: &Settings, file_name: &str) -> Result<()> {
    let settings = serde_json::to_string_pretty(settings)?;
    std::fs::write(file_name, settings)?;
    Ok(())
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            network: bitcoin::Network::Regtest,
            wallet_type: WalletType::SingleSigTaproot,
        }
    }
}