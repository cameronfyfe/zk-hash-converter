_:
    @just --list

build +args='':
    cargo build {{args}}

build-cuda +args='':
    cargo build --features cuda --target-dir target-cuda {{args}} 

docker-build +args='': _tmp build-pyzero-builder 
    docker run --rm \
        -v `pwd`:/build \
        -w /build \
        --env CARGO_HOME=./tmp/.cargo-docker \
        pyzero-builder \
        cargo build --target-dir target-docker {{args}}

docker-build-cuda +args='': _tmp build-pyzero-builder-cuda
    docker run --rm \
        -v `pwd`:/build \
        -w /build \
        --env CARGO_HOME=./tmp/.cargo-docker \
        pyzero-builder-cuda \
        cargo build --features cuda --target-dir target-docker-cuda {{args}}

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

ci: fmt-check build readme-check

build-pyzero-builder:
    docker build -t pyzero-builder -f docker/base.dockerfile .

build-pyzero-builder-cuda:
    docker build -t pyzero-builder-cuda -f docker/cuda.dockerfile .

enter-pyzero-builder:
    docker run --rm -it \
        -v `pwd`:/build \
        -w /build \
        pyzero-builder \
        bash

_tmp:
    mkdir -p tmp