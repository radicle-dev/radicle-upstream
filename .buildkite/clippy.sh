#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

echo "--- Setting up Linux environment"

export CACHE_FOLDER="/cache"
export HOME="$CACHE_FOLDER"

echo "CACHE_FOLDER=$CACHE_FOLDER"
echo "HOME=$HOME"

mkdir -p "$CACHE_FOLDER/cargo"
mkdir -p "$CACHE_FOLDER/rustup"

export CARGO_HOME="$CACHE_FOLDER/cargo"
export RUSTUP_HOME="$CACHE_FOLDER/rustup"

chmod -R a+w $CARGO_HOME $RUSTUP_HOME

export PATH="$HOME/.cargo/bin:$PATH"

echo "--- Run cargo fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy lints"
(cd proxy && time cargo clippy --all --all-features --all-targets)

echo "--- Run proxy docs"
(cd proxy && time cargo doc --no-deps)
