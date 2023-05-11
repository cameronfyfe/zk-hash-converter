_:
    @just --list

build:
    cargo build --release

run-cli +ARGS='':
    target/release/zk-hash-converter-cli {{ARGS}}