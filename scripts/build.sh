#!/bin/bash

# Load .env safely and export its variables
if [ -f .env ]; then
  set -o allexport
  . .env
  set +o allexport
fi

# Apply path overrides from .env
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH_PREPEND}:${SPEECHSDK_LIB}:${LD_LIBRARY_PATH}"
export PATH="${PATH_PREPEND}:${PATH}"

# Ensure updated PATH is used
hash -r

cargo build --release --features "audio-buffering,stt-google-async-streams,stt-google-beta"