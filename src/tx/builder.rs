use crate::settings::Settings;
use crate::tx::inscription_id_resolver::InscriptionIdResolver;
use crate::tx::manifest::Manifest;
use anyhow::Result;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{Address, Amount, Network, Psbt, Sequence, Transaction, Witness};
use esplora_client::Builder;
use log::debug;
use std::str::FromStr;

pub(crate) fn create_psbt(manifest: &Manifest, settings: &Settings) -> Result<Psbt> {
    let transaction = make_transaction(manifest, settings)?;
    let mut psbt = Psbt::from_unsigned_tx(transaction)?;

    let url = match settings.network {
        Network::Bitcoin => "https://taprootwizards.mempool.space/api",
        Network::Testnet => "https://mempool.space/testnet/api",
        Network::Signet => "https://mempool.space/signet/api",
        Network::Regtest => "http:/localhost:6002/api",
        _ => panic!("Unsupported network"),
    };

    let client = Builder::new(url).build_blocking();
    for i in 0..psbt.inputs.len() {
        let txin = &psbt.unsigned_tx.input[i];
        let tx = client.get_tx_no_opt(&txin.previous_output.txid)
            .map_err(|e| anyhow::anyhow!("Error getting previous tx {txin:?}: {e}"))?;
        psbt.inputs[i].witness_utxo = Some(tx.output[txin.previous_output.vout as usize].clone());
    }

    Ok(psbt)
}

pub(crate) fn estimate_fee(manifest: &Manifest, settings: &Settings) -> Result<Amount> {
    let mut transaction = make_transaction(manifest, settings)?;
    if manifest.funding_outpoint.is_none() {
        // push a dummy input to estimate the fee
        transaction.input.push(bitcoin::TxIn {
            previous_output: bitcoin::OutPoint::default(),
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Default::default(),
        });
    }
    let dummy_witness = settings.wallet_type.dummy_witness();
    for input in &mut transaction.input {
        input.witness = Witness::from(dummy_witness.clone());
    }
    let vsize = transaction.vsize();
    let fee_rate = manifest.fee_rate;
    let fee = Amount::from_sat((vsize as u64 * fee_rate) + 1234);
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
        let inscription_info = id_resolver.resolve_inscription_id(&transfer.inscription_id)?;
        let input = bitcoin::TxIn {
            previous_output: inscription_info.outpoint,
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Default::default(),
        };
        let address = Address::from_str(&transfer.address)?.require_network(settings.network)?;
        let output = bitcoin::TxOut {
            value: Amount::from_sat(inscription_info.amount),
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
        value: Amount::from_sat(1234), // add a small output to allow for future fee bumping
        script_pubkey: Address::from_str(&manifest.change_address)?
            .require_network(settings.network)?
            .script_pubkey(),
    });

    transaction.input = inputs;
    transaction.output = outputs;

    if manifest.funding_outpoint.is_some() {
        assert_eq!(
            transaction.input.len(),
            transaction.output.len(),
            "must have the same number of inputs and outputs"
        );
    } else {
        assert_eq!(
            transaction.input.len(),
            transaction.output.len() - 1,
            "must have one less output than inputs"
        );
    }
    assert_eq!(
        manifest.transfers.len() + 1,
        transaction.output.len(),
        "must have the same number of transfers as outputs plus an anchor output for CPFP"
    );
    assert_eq!(
        transaction.output.last().unwrap().value,
        Amount::from_sat(1234),
        "last output must be a dust output for CPFP"
    );

    Ok(transaction)
}
