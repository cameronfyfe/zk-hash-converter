#!/usr/bin/env bash

KB="$1"

TMP=./tmp
mkdir -p $TMP

FILE_TO_HASH="$TMP/file-to-hash_$KB-kb"

dd \
    if=/dev/urandom \
    of=$FILE_TO_HASH \
    bs=1K \
    count=$KB \
;

start_time=$(date +%s.%N)

just run-cli prove \
    --file $FILE_TO_HASH \
    --journal $FILE_TO_HASH.journal.json \
    --receipt $FILE_TO_HASH.receipt.json \
;

end_time=$(date +%s.%N)
elapsed=$(echo "$end_time - $start_time" | bc)

printf "$(date) $KB Kb: \t%.6f seconds\n" "$elapsed"
