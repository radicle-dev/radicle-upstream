#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

source ci/env.sh

log-group-start "install toolcahin"
time rustup show active-toolchain
log-group-start "install toolcahin"

log-group-start "yarn install"
# Unsetting GITHUB_ACTIONS because yarn tries to add log groups in a buggy way.
env -u GITHUB_ACTIONS yarn install --immutable
log-group-end

log-group-start "Building and packaging binaries"
if [[ "${RUNNER_OS:-}" == "macOS" ]]; then
  :
  # TODO setup notarization
  # export NOTARIZE=true
  # export APPLE_ID="rudolfs@monadic.xyz"
  # export APPLE_ID_PASSWORD="@keychain:AC_PASSWORD"
  # export CSC_NAME="Monadic GmbH (35C27H9VL2)"
fi

time yarn dist
target="$(uname -m)-$(uname -s | tr "[:upper:]" "[:lower:]")"
mkdir "dist/${target}"
cp target/release/upstream-seed "dist/${target}/upstream-seed"
log-group-end
