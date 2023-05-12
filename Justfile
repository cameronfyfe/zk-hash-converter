_:
    @just --list

build:
    cargo build --release

lint:
    cargo clippy

fmt-check:
    @just _rust-nightly cargo fmt --check

fmt:
    @just _rust-nightly cargo fmt

readme-update:
    present --in-place README.md

readme-check: _tmp
    present README.md > tmp/README.md
    diff README.md tmp/README.md

ci: fmt-check build lint

run-cli +ARGS='':
    target/release/zk-hash-converter-cli {{ARGS}}

_rust-nightly +CMD='':
    nix develop .#rust-nightly -c {{CMD}}

_tmp:
    mkdir -p tmp