#!/bin/bash

# Load .env safely and export its variables
if [ -f .env ]; then
  set -o allexport
  . .env
  set +o allexport
fi

# Ensure updated PATH is used
hash -r


MODEL="latest_long" \
LANGUAGE="en-US" \
SAMPLE_RATE_HERTZ=16000 \
AUDIO_CHANNEL_COUNT=1 \
./target/release/google