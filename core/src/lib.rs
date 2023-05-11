use anyhow::Result;
use risc0_zkvm::{Executor, ExecutorEnv, SessionReceipt};
use serde_json::json;
use zk_hash_converter_methods::GUEST_ELF;

// include this file instead of handling as a crate because `core` library
// and risc5 based `guest` binary should use different `serde` configs
include!("../../interface/interface.rs");

pub struct ProveHashesResponse {
    pub journal: String,
    pub receipt: String,
}

pub fn prove_hashes(data: Vec<u8>) -> Result<ProveHashesResponse> {
    let data = DataToHash { data };
    let data = risc0_zkvm::serde::to_vec(&data)?;
    let guest_env = ExecutorEnv::builder()
        // default session limit is 64*1000*1000 cycles
        // default was exceed when running with >100kb input
        .session_limit(1024* 1024 * 1024 * 1024)
        .add_input(&data)
        .build();

    let session = Executor::from_elf(guest_env, GUEST_ELF)?.run()?;
    let receipt = session.prove()?;

    let HashResults { sha256, blake3 } = hash_data_from_receipt(&receipt)?;

    let response = ProveHashesResponse {
        journal: serde_json::to_string_pretty(&json!({
            "sha256": bytes_to_hex_string(&sha256),
            "blake3": bytes_to_hex_string(&blake3),
        }))?,
        receipt: serde_json::to_string(&receipt)?,
    };

    Ok(response)
}

fn hash_data_from_receipt(receipt: &SessionReceipt) -> Result<HashResults> {
    let hash_data = risc0_zkvm::serde::from_slice(receipt.journal.as_slice())?;

    Ok(hash_data)
}

// TODO: hex crate
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}
