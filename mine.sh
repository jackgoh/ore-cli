#!/bin/bash

# Get the RPC endpoint from the execution parameter
while [[ "$1" != "" ]]; do
    case $1 in
        --rpc)
            rpc_endpoint=$2
            shift
        ;;
        *)
        ;;
    esac
    shift
done

while true; do
    echo "Running iteration $i"
    ore \
    --rpc "$rpc_endpoint" \
    --keypair ~/.config/solana/id.json \
    --priority-fee 500000 \
    mine \
    --threads 4
    echo "Iteration $i complete"
done