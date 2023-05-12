#!/usr/bin/env bash

set -xeu

GEN=scripts/generate-hash-proof-for-file.sh

function run() {

    KB="$1"
    bash scripts/generate-hash-proof-for-file.sh $KB
}


run 500

# # 1kb - 100kb
# run 1
# run 10
# run 100
# # 1Mb - 100Mb
run 1000
run 10000
run 100000
# 1Gb - 10Gb
run 1000000
run 10000000
