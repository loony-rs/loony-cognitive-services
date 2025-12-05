#!/bin/bash

# Load .env safely and export its variables
if [ -f .env ]; then
  set -o allexport
  . .env
  set +o allexport
fi

# Ensure updated PATH is used
hash -r

./target/release/speechmatics