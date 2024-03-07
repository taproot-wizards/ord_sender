use bitcoin::opcodes::all::OP_CHECKMULTISIG;
use bitcoin::script::Builder;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref FAKE_2_OF_3_WITNESS: Vec<Vec<u8>> = {
        let fake_signature = [0u8; 64];
        let fake_pubkey = [0u8; 33];
        let mut builder = Builder::new();
        builder = builder
            .push_int(2)
            .push_slice(&fake_pubkey)
            .push_slice(&fake_pubkey)
            .push_slice(&fake_pubkey)
            .push_int(3)
            .push_opcode(OP_CHECKMULTISIG);
        let script = builder.into_script();
        let mut witness = Vec::new();
        witness.push(fake_signature.to_vec());
        witness.push(fake_signature.to_vec());
        witness.push(script.to_bytes());
        witness
    };
    static ref FAKE_SINGLESIG_TAPROOT_WITNESS: Vec<Vec<u8>> = {
        let fake_signature = [0u8; 64];
        let mut witness = Vec::new();
        witness.push(fake_signature.to_vec());
        witness
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum WalletType {
    SingleSigTaproot,
    MultiSigSegwit { threshold: u8, max: u8 },
}

impl WalletType {
    pub(crate) fn dummy_witness(&self) -> Vec<Vec<u8>> {
        match self {
            WalletType::SingleSigTaproot => FAKE_SINGLESIG_TAPROOT_WITNESS.clone(),
            WalletType::MultiSigSegwit { threshold, max } => {
                if *threshold != 2 || *max != 3 {
                    // TODO: support other multisig configurations
                    panic!("Only 2-of-3 multisig is supported");
                }
                FAKE_2_OF_3_WITNESS.clone()
            }
        }
    }
}
