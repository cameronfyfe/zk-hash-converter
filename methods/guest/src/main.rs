#![no_main]

use risc0_zkvm::{guest::env, sha::{Impl, Sha256}};

// include this file instead of handling as a crate because `core` library
// and risc5 based `guest` binary should use different `serde` configs
include!("../../../interface/interface.rs");

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let DataToHash { data } = env::read();

    let sha256 = Impl::hash_bytes(&data);
    let sha256: [u8; 32] = sha256.as_bytes().try_into().unwrap();

    let blake3 = blake3::hash(&data);
    let blake3: [u8; 32] = blake3.try_into().unwrap();

    env::commit(&HashResults { sha256, blake3 });
}
