use std::path::PathBuf;

use clap::{Parser, ValueHint};

/// zk-hash-converter
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(name = "zk-hash-converter")]
pub struct Args {
    /// Subcommand.
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

/// CLI Args top-level Subcommand
#[derive(Parser, Debug)]
pub enum Subcommand {
    Prove(Prove),
    Verify(Verify),
    GuestId(GuestId),
}

/// Prove a hash correlation
#[derive(Parser, Debug)]
pub struct Prove {
    /// Message to hash
    #[clap(short, long, value_parser)]
    pub message: Option<String>,
    /// File to hash
    #[clap(short, long, value_hint = ValueHint::FilePath, value_parser)]
    pub file: Option<PathBuf>,
    /// Proof file destination
    #[clap(short, long, default_value = "proof.bin", value_hint = ValueHint::AnyPath, value_parser)]
    pub proof: PathBuf,
}

/// Verify a hash correlation
#[derive(Parser, Debug)]
pub struct Verify {
    /// Proof file to verify
    #[clap(value_parser)]
    pub proof: PathBuf,
}

/// Print the Guest ID
#[derive(Parser, Debug)]
pub struct GuestId {}
