_:
    @just --list

build:
    cargo build --release

lint:
    cargo clippy

fmt-check:
    nixpkgs-fmt . --check
    cargo fmt --check

fmt:
    nixpkgs-fmt .
    cargo fmt

readme-update:
    present --in-place README.md

readme-check: _tmp
    present README.md > tmp/README.md
    diff README.md tmp/README.md

ci: fmt-check build lint

run-cli +ARGS='':
    target/release/zk-hash-converter-cli {{ARGS}}

_tmp:
    mkdir -p tmp