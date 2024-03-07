use crate::settings::Settings;
use crate::tx::inscription_id_resolver::InscriptionIdResolver;
use crate::tx::manifest::Manifest;
use anyhow::Result;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{Address, Amount, Psbt, Sequence, Transaction, Witness};
use log::debug;
use std::str::FromStr;

pub(crate) fn create_psbt(manifest: &Manifest, settings: &Settings) -> Result<Psbt> {
    let transaction = make_transaction(manifest, settings)?;
    let psbt = Psbt::from_unsigned_tx(transaction)?;
    Ok(psbt)
}

pub(crate) fn estimate_fee(manifest: &Manifest, settings: &Settings) -> Result<Amount> {
    let mut transaction = make_transaction(manifest, settings)?;
    let dummy_witness = settings.wallet_type.dummy_witness();
    for input in &mut transaction.input {
        input.witness = Witness::from(dummy_witness.clone());
    }
    let vsize = transaction.vsize();
    let fee_rate = manifest.fee_rate;
    let fee = Amount::from_sat((vsize as u64 * fee_rate) + 546);
    Ok(fee)
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
    let id_resolver: Box<dyn InscriptionIdResolver> = (&settings.id_resolver).into();
    for transfer in &manifest.transfers {
        let outpoint = match transfer.outpoint {
            Some(outpoint) => outpoint,
            None => match &transfer.inscription_id {
                Some(inscription_id) => id_resolver.resolve_inscription_id(inscription_id)?,
                None => {
                    return Err(anyhow::anyhow!(
                        "You must provide either an outpoint or an inscription id"
                    ))
                }
            },
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
        debug!(
            "adding input: {:?} and output: {:?} for transfer: {:?}",
            input, output, transfer
        );
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
        }
        None => (),
    }

    outputs.push(bitcoin::TxOut {
        value: Amount::from_sat(546), // add a dust-sized output to allow for future fee bumping
        script_pubkey: Address::from_str(&manifest.anchor_address)?
            .require_network(settings.network)?
            .script_pubkey(),
    });

    transaction.input = inputs;
    transaction.output = outputs;

    Ok(transaction)
}
