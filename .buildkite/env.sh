#!/bin/bash
set -Eeou pipefail

if [[ "${BUILDKITE_AGENT_META_DATA_PLATFORM:-}" == "macos" ]]; then
  echo "--- Setting up macOS environment"

  export HOME=/Users/buildkite
  export CACHE_FOLDER="$HOME/buildkite-cache"

  echo "CACHE_FOLDER=$CACHE_FOLDER"
  echo "HOME=$HOME"
else
  echo "--- Setting up Linux environment"

  export CACHE_FOLDER="/cache"
  export HOME="$CACHE_FOLDER"

  echo "CACHE_FOLDER=$CACHE_FOLDER"
  echo "HOME=$HOME"
fi

export NPM_CONFIG_CACHE="$CACHE_FOLDER/npm"
export YARN_CACHE_FOLDER="$CACHE_FOLDER/yarn"
export YARN_TEMPDIR="$CACHE_FOLDER/yarn-tmp"
export CARGO_HOME="$CACHE_FOLDER/cargo"
export RUSTUP_HOME="$CACHE_FOLDER/rustup"

mkdir -p "$NPM_CONFIG_CACHE"
mkdir -p "$YARN_CACHE_FOLDER"
mkdir -p "$YARN_TEMPDIR"
mkdir -p "$CARGO_HOME"
mkdir -p "$RUSTUP_HOME"


chmod -R a+w $CARGO_HOME $RUSTUP_HOME

export PATH="$HOME/.cargo/bin:$PATH"

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false

# Most of the caching is done through caching ./target
export SCCACHE_CACHE_SIZE="1G"
