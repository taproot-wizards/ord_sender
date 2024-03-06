use std::str::FromStr;
use bitcoin::{Address, Amount, Sequence, Transaction};
use anyhow::Result;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use log::debug;
use crate::settings::Settings;
use crate::tx::manifest::Manifest;

pub(crate) fn create_transaction(manifest: &Manifest, settings: &Settings) -> Result<Transaction> {
    todo!()
}

pub(crate) fn estimate_fee(manifest: &Manifest, settings: &Settings) -> Result<Amount> {
    todo!()
}

fn make_transaction(manifest: &Manifest, settings: &Settings) -> Result<Transaction> {
    let mut transaction = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: Vec::new(),
        output: Vec::new(),
    };
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for transfer in &manifest.transfers {
        let outpoint = match transfer.outpoint {
            Some(outpoint) => outpoint,
            None => {
                match &transfer.inscription_id {
                    Some(inscription_id) => resolve_inscription_id(&inscription_id)?,
                    None => return Err(anyhow::anyhow!("You must provide either an outpoint or an inscription id")),
                }
            }
        };
        let input = bitcoin::TxIn {
            previous_output: outpoint,
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Default::default(),
        };
        let address = Address::from_str(&transfer.address)?.require_network(settings.network)?;
        let output = bitcoin::TxOut {
            value: Amount::from_sat(transfer.amount),
            script_pubkey: address.script_pubkey(),
        };
        debug!("adding input: {:?} and output: {:?} for transfer: {:?}", input, output, transfer);
        inputs.push(input);
        outputs.push(output);
    }
    
    match &manifest.funding_outpoint {
        Some(outpoint) => {
            let input = bitcoin::TxIn {
                previous_output: *outpoint,
                script_sig: Default::default(),
                sequence: Sequence::MAX,
                witness: Default::default(),
            };
            inputs.push(input);
        },
        None => (),
    }
    
    outputs.push(bitcoin::TxOut {
        value: Amount::from_sat(546), // add a dust-sized output to allow for future fee bumping
        script_pubkey: Address::from_str(&manifest.anchor_address)?.require_network(settings.network)?.script_pubkey(),
    });
    
    transaction.input = inputs;
    transaction.output = outputs;
    
    Ok(transaction)
}

fn resolve_inscription_id(inscription_id: &str) -> Result<bitcoin::OutPoint> {
    todo!()
}