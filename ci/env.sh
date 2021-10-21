#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -Eeou pipefail

if [[ "${BUILDKITE:-}" = "true" ]]; then
  source ci/setup-buildkite.sh
elif [[ "${GITHUB_ACTIONS:-}" = "true" ]]; then
  source ci/setup-github-actions.sh
else
  echo "Unknown CI platform"
  exit 1
fi

export YARN_CACHE_FOLDER="$CACHE_FOLDER/yarn"
export CARGO_HOME="$CACHE_FOLDER/cargo"
export CYPRESS_CACHE_FOLDER="$CACHE_FOLDER/cypress"
export PATH="$HOME/.cargo/bin:$CARGO_HOME/bin:$PATH"

export TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'
