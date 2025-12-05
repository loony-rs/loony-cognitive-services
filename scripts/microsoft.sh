#!/bin/bash

# Load .env safely and export its variables
if [ -f .env ]; then
  set -o allexport
  . .env
  set +o allexport
fi


export LD_LIBRARY_PATH=/home/sankar/.microsoft/speech-sdk/lib/x64:$LD_LIBRARY_PATH

cargo build --release

RUST_LOG=debug \
MSSubscriptionKey= \
MSServiceRegion=centralindia \
PORT=2011 \
./target/release/microsoft

