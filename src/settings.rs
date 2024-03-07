use crate::tx::dummy_witness::WalletType;
use crate::tx::inscription_id_resolver::{InscriptionIdResolver, StaticInscriptionIdResolver};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) network: bitcoin::Network,
    pub(crate) wallet_type: WalletType,
    pub(crate) id_resolver: IdResolverConfiguration,
}

pub fn from_toml_file<T: serde::de::DeserializeOwned>(path: &str) -> Result<T> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = toml::from_str(&contents)?;
    Ok(data)
}

pub fn to_toml_file<T: serde::ser::Serialize>(data: &T, path: &str) -> Result<()> {
    let toml = toml::to_string(data)?;
    let mut file = File::create(path)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            network: bitcoin::Network::Regtest,
            wallet_type: WalletType::SingleSigTaproot,
            id_resolver: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) enum IdResolverConfiguration {
    Static { filename: String },
    OrdServer { url: Option<String> },
}

impl Default for IdResolverConfiguration {
    fn default() -> Self {
        Self::Static {
            filename: "inscription_id_map.json".to_string(),
        }
    }
}

impl From<&IdResolverConfiguration> for Box<dyn InscriptionIdResolver> {
    fn from(config: &IdResolverConfiguration) -> Box<dyn InscriptionIdResolver> {
        match config {
            IdResolverConfiguration::Static { filename } => Box::new(
                StaticInscriptionIdResolver::from_json_file(&filename)
                    .expect("Failed to load id resolver"),
            ),
            IdResolverConfiguration::OrdServer { url } => {
                unimplemented!()
            }
        }
    }
}
