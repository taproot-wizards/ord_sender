mod settings;
mod tx;

use crate::settings::Settings;
use crate::tx::manifest::Manifest;
use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine;
use clap::Parser;
use std::default;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    settings: Option<String>,

    #[command(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    Blank,
    MakeTx { manifest: String },
    EstimateFee { manifest: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let settings_path = cli.settings.clone().unwrap_or("settings.toml".to_string());
    let settings = match settings::from_toml_file(&settings_path) {
        Ok(settings) => settings,
        Err(_) => {
            let settings = Settings::default();
            println!("Writing default settings to {settings_path}");
            settings::to_toml_file(&settings, &settings_path)?;
            settings
        }
    };

    match cli.action {
        Action::Blank => {
            // make a default manifest and write it to a file
            let manifest: Manifest = Default::default();
            let manifest_filename = "manifest.json";
            manifest.to_json_file(manifest_filename)?;
            println!("Wrote default manifest to {}", manifest_filename);
        }
        Action::MakeTx {
            manifest: manifest_filename,
        } => {
            let manifest = tx::manifest::Manifest::from_json_file(&manifest_filename)?;
            let psbt = tx::builder::create_psbt(&manifest, &settings)?;
            let psbt_filename = manifest_filename.replace(".json", ".psbt");
            let mut file = std::fs::File::create(&psbt_filename)?;
            let base64_psbt = general_purpose::STANDARD.encode(psbt.serialize());
            file.write_all(base64_psbt.as_bytes())?;
            println!("Wrote psbt to {}", psbt_filename);
        }
        Action::EstimateFee {
            manifest: manifest_filename,
        } => {
            let manifest = tx::manifest::Manifest::from_json_file(&manifest_filename)?;
            let fee = tx::builder::estimate_fee(&manifest, &settings)?;
            println!("{}", fee);
        }
    }

    Ok(())
}
