use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use zk_hash_converter_cli::cli;
use zk_hash_converter_core::{prove_hashes, verify_proof, Proof, ProveHashesResponse};

fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.cmd {
        cli::Subcommand::Prove(args) => prove(args)?,
        cli::Subcommand::Verify(args) => verify(args)?,
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

    let ProveHashesResponse { journal, proof } = prove_hashes(data)?;

    fs::write(args.journal, journal)?;
    fs::write(args.proof, proof)?;

    Ok(())
}

fn verify(args: cli::Verify) -> Result<()> {
    let proof = fs::read_to_string(args.proof)?;
    let proof = serde_json::from_str::<Proof>(&proof)?;

    let verified = verify_proof(&proof)?;

    if verified {
        println!("Proof verified!");
    } else {
        println!("Proof failed to verify.")
    }

    Ok(())
}
