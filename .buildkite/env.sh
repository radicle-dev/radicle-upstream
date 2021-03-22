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
export CARGO_HOME="$CACHE_FOLDER/cargo"

mkdir -p "$NPM_CONFIG_CACHE"
mkdir -p "$YARN_CACHE_FOLDER"
mkdir -p "$CARGO_HOME"

chmod -R a+w $CARGO_HOME

export PATH="$HOME/.cargo/bin:$PATH"

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false
