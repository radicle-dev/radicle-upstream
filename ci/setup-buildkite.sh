#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

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
  echo "--- ${1:-}"
}

function log-group-end () {
  true
}
