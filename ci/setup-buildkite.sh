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
  export HOME="/tmp/home"
  mkdir $HOME

  echo "CACHE_FOLDER=$CACHE_FOLDER"
  echo "HOME=$HOME"
fi

function log-group-start () {
  echo "--- $1"
}

function log-group-end () {
  true
}
