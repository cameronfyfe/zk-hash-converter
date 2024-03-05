# zk-hash-converter

zk-hash-converter is a tool for generating and verifying zero knowledge proofs proving multiple hashes to be correlated to the same data blob without revealing information about the data blob.

# Getting Started

Enter nix shell dev environment

    nix develop

Build

    just build

Build for CUDA

    just build-cuda

# Running CLI

## Quick Examples

Generate proof for hashes of a message

    zk-hash-converter prove --message 'abc'

Generate proof for hashes of a file

    zk-hash-converter prove --file my-file

Verify a proof

    zk-hash-converter verify --proof proof.bin

Show the Guest ID of the prover

    zk-hash-converter guest-id

## Usage

```
zk-hash-converter

Usage: zk-hash-converter <COMMAND>

Commands:
  prove     Prove a hash correlation
  verify    Verify a hash correlation
  guest-id  Print the Guest ID
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `prove`

```
Prove a hash correlation

Usage: zk-hash-converter prove [OPTIONS]

Options:
  -m, --message <MESSAGE>  Message to hash
  -f, --file <FILE>        File to hash
  -p, --proof <PROOF>      Proof file destination [default: proof.bin]
  -h, --help               Print help
```

### `verify`

```
Verify a hash correlation

Usage: zk-hash-converter verify <PROOF>

Arguments:
  <PROOF>  Proof file to verify

Options:
  -h, --help  Print help
```

### `guest-id`

```
Print the Guest ID

Usage: zk-hash-converter guest-id

Options:
  -h, --help  Print help
```
