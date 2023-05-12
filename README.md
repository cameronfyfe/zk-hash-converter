# zk-hash-converter

zk-hash-converter is a tool for generating and verifying zero knowledge proofs that hash 2 results from different algorithms proven to be correlated to the same data without revealing information about the data.

# Getting Started

Enter nix shell dev environment

    nix develop

Build

    just build

# Running CLI

## Quick Examples

Generate proof for hashes of a message

    just run-cli prove --message 'abc' --journal ./journal.json --proof ./proof.json

Generate proof for hashes of a file

    just run-cli prove --file ./my-file --journal ./journal.json --proof ./proof.json

Verify a proof

    just run-cli verify --proof ./proof.json

## Usage

```present just run-cli help
CLI Args

Usage: zk-hash-converter-cli <COMMAND>

Commands:
  prove   Prove a hash correlation
  verify  Verify a hash correlation
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `prove`

```present just run-cli prove --help
Prove a hash correlation

Usage: zk-hash-converter-cli prove [OPTIONS] --journal <JOURNAL> --proof <PROOF>

Options:
  -m, --message <MESSAGE>  Message to hash
  -f, --file <FILE>        File to hash
  -j, --journal <JOURNAL>  Journal destination
  -p, --proof <PROOF>      Proof destination
  -h, --help               Print help
```

### `verify`

```present just run-cli verify --help
Verify a hash correlation

Usage: zk-hash-converter-cli verify --proof <PROOF>

Options:
  -p, --proof <PROOF>  Proof file to verify
  -h, --help           Print help
```