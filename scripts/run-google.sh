#!/bin/bash

# Load .env safely and export its variables
if [ -f .env ]; then
  set -o allexport
  . .env
  set +o allexport
fi

# Ensure updated PATH is used
hash -r


MODEL="chirp_3" \
LANG_CODE="en-US" \
SRH=8000 \
ACC=1 \
./target/release/google