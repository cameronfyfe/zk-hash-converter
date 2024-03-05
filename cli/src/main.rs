use std::fs;

use anyhow::{bail, Result};
use clap::Parser;
use zk_hash_converter::cli;
use zk_hash_converter_core::{prove_hashes, verify_proof, ProveResponse, VerifyResponse};

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.cmd {
        cli::Subcommand::Prove(args) => prove(args)?,
        cli::Subcommand::Verify(args) => verify(args)?,
        cli::Subcommand::GuestId(args) => guest_id(args)?,
    };

    Ok(())
}

fn prove(args: cli::Prove) -> Result<()> {
    let data = match (args.message, args.file) {
        (Some(message), None) => message.as_bytes().to_owned(),
        (None, Some(file)) => fs::read(file)?,
        (Some(_), Some(_)) => {
            bail!("Provide only a message (--message) or a file to hash (--file). Not both.")
        }
        (None, None) => bail!("Provide a message (--message) or file to hash (--file)."),
    };

    let ProveResponse {
        hash_results,
        receipt,
    } = prove_hashes(data)?;

    println!("Sha256: {}", hex::encode(hash_results.sha256));
    println!("Blake3: {}", hex::encode(hash_results.blake3));

    fs::write(args.proof, receipt)?;

    Ok(())
}

fn verify(args: cli::Verify) -> Result<()> {
    let proof = fs::read(args.proof)?;

    let VerifyResponse {
        verified,
        hash_results,
    } = verify_proof(&proof)?;

    println!("Sha256: {}", hex::encode(hash_results.sha256));
    println!("Blake3: {}", hex::encode(hash_results.blake3));

    if verified {
        println!("Proof verified!");
    } else {
        println!("Proof failed to verify.")
    }

    Ok(())
}

fn guest_id(args: cli::GuestId) -> Result<()> {
    let _ = args;

    println!("{}", zk_hash_converter_core::guest_id());

    Ok(())
}
