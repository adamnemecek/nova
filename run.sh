#!/bin/bash

CARGO_OPTS=--quiet

if [[ "$1" = "release" ]]; then
  CARGO_OPTS="$CARGO_OPTS --release"
fi

exec cargo run $CARGO_OPTS -- \
  --vfs-mount / data \
  --vfs-mount / data/local
