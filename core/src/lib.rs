use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use zk_hash_converter_core_methods::{GUEST_ELF, GUEST_ID};

// include this file instead of handling as a crate because `core` library
// and risc-v based `guest` binary should use different `serde` configs
include!("../methods/interface.rs");

use interface::{DataToHash, HashResults};

pub struct ProveResponse {
    pub hash_results: HashResults,
    pub receipt: Vec<u8>,
}

pub struct VerifyResponse {
    pub verified: bool,
    pub hash_results: HashResults,
}

pub fn prove_hashes(data: Vec<u8>) -> Result<ProveResponse> {
    let data = DataToHash { data };
    let data = risc0_zkvm::serde::to_vec(&data)?;

    let guest_env = ExecutorEnv::builder().write(&data)?.build()?;

    let receipt = default_prover().prove(guest_env, GUEST_ELF)?;

    let hash_results = hash_data_from_receipt(&receipt)?;

    let receipt = bincode::serialize(&receipt)?;

    Ok(ProveResponse {
        hash_results,
        receipt,
    })
}

pub fn verify_proof(proof: &[u8]) -> Result<VerifyResponse> {
    let receipt = bincode::deserialize::<Receipt>(proof)?;

    let hash_results = hash_data_from_receipt(&receipt)?;

    let result = receipt.verify(GUEST_ID);

    let verified = result.is_ok();

    Ok(VerifyResponse {
        verified,
        hash_results,
    })
}

pub fn guest_id() -> String {
    hex::encode(vec_u8_from_u32_slice_little_endian(&GUEST_ID))
}

fn hash_data_from_receipt(receipt: &Receipt) -> Result<HashResults> {
    let hash_data = receipt.journal.decode()?;

    Ok(hash_data)
}

fn vec_u8_from_u32_slice_little_endian(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect()
}
