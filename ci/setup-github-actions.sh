#!/bin/bash
set -Eeou pipefail

export CACHE_FOLDER="$HOME/cache"

function log-group-start () {
  echo "::group::$1"
}

function log-group-end () {
  echo "::endgroup::"
}
