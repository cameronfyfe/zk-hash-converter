use clap::{Parser, ValueHint};
use std::path::PathBuf;

/// CLI Args
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(name = "hash_converter")]
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
}

/// Prove a hash correlation
#[derive(Parser, Debug)]
pub struct Prove {
    /// Message to hash
    #[clap(short, long, value_parser)]
    pub message: Option<String>,
    /// File to hash
    #[clap(short, long, value_parser)]
    pub file: Option<PathBuf>,
    /// Journal destination
    #[clap(short, long, value_parser)]
    pub journal: PathBuf,
    /// Proof destination
    #[clap(short, long, value_hint = ValueHint::AnyPath, value_parser)]
    pub proof: PathBuf,
}

/// Verify a hash correlation
#[derive(Parser, Debug)]
pub struct Verify {
    /// Proof file to verify
    #[clap(short, long, value_parser)]
    pub proof: PathBuf,
}
