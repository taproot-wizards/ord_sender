mod tx;
mod settings;

use anyhow::Result;
use clap::Parser;
use crate::settings::Settings;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    settings: Option<String>,

    #[command(subcommand)]
    action: Action,
}

#[derive(Parser)]
enum Action {
    MakeTx { manifest: String },
    EstimateFee { manifest: String },
}


fn main() -> Result<()>{
    let cli = Cli::parse();
    let settings_path = cli.settings.clone().unwrap_or("settings.json".to_string());
    let settings = match settings::from_json_file(&settings_path) {
        Ok(settings) => settings,
        Err(_) => {
            let settings = Settings::default();
            println!("Writing default settings to {settings_path}");
            settings::to_json_file(&settings, &settings_path)?;
            settings
        }
    };
    
    match cli.action {
        Action::MakeTx { manifest } => {
            let manifest = tx::manifest::Manifest::from_json_file(&manifest)?;
            let tx = tx::builder::create_transaction(&manifest, &settings)?;
            println!("{}", tx.txid());
        }
        Action::EstimateFee { manifest } => {
            let manifest = tx::manifest::Manifest::from_json_file(&manifest)?;
            let fee = tx::builder::estimate_fee(&manifest, &settings)?;
            println!("{}", fee);
        }
    }

    Ok(())
}


