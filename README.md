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

    zk-hash-converter-cli prove --message 'abc' --journal ./journal.json --proof ./proof.json

Generate proof for hashes of a file

    zk-hash-converter-cli prove --file ./my-file --journal ./journal.json --proof ./proof.json

Verify a proof

    zk-hash-converter-cli verify --proof ./proof.json

Example `journal.json`:
```json
{
  "blake3": "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
  "sha256": "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
}
```

## Usage

```
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

```
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

```
Verify a hash correlation

Usage: zk-hash-converter-cli verify --proof <PROOF>

Options:
  -p, --proof <PROOF>  Proof file to verify
  -h, --help           Print help
```