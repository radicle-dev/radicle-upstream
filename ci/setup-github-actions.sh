#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -Eeou pipefail

export CACHE_FOLDER="$HOME/cache"

function log-group-start () {
  echo "::group::${1:-}"
}

function log-group-end () {
  echo "::endgroup::"
}
