#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

source .buildkite/env.sh

echo "--- Installing yarn dependencies"
yarn install --immutable

echo "--- Loading proxy target cache"

declare -r rust_target_cache="$CACHE_FOLDER/proxy-target"
mkdir -p "$rust_target_cache"
ln -s "${rust_target_cache}" ./target

if [[ "${BUILDKITE_AGENT_META_DATA_PLATFORM:-}" != "macos" ]]; then
  free_cache_space_kb=$(df --output=avail /cache | sed -n 2p)
  min_free_cache_kb=$(( 2 * 1024 * 1024 )) # 2GiB is 25%
  echo "$(( free_cache_space_kb / 1024 )) MiB free space on /cache"
  if [[ $free_cache_space_kb -le $min_free_cache_kb ]]; then
    echo "Not enough free space on /cache. Deleting ${rust_target_cache}"
    du -sh /cache/*
    rm -r "${rust_target_cache}"
    mkdir -p "${rust_target_cache}"
  fi
fi

echo "--- Updating submodules"
./scripts/test-setup.sh

echo "--- Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"

echo "--- Run proxy docs"
(
  export RUSTDOCFLAGS="-D intra-doc-link-resolution-failure"
  time cargo doc --workspace --no-deps --all-features --document-private-items
)

echo "--- Run proxy fmt"
time cargo fmt --all -- --check

echo "--- Run proxy lints"
time cargo clippy --all --all-features --all-targets -Z unstable-options -- --deny warnings

echo "--- Run app eslint checks"
time yarn lint

echo "--- Run app prettier checks"
time yarn prettier:check

echo "--- Check TypeScript"
time yarn typescript:check

echo "--- Run proxy tests"
(
  export RUST_TEST_TIME_UNIT=2000,4000
  export RUST_TEST_TIME_INTEGRATION=2000,8000
  cargo build --tests --all --all-features --all-targets
  timeout 6m cargo test --all --all-features --all-targets -- -Z unstable-options --report-time
)

echo "--- Bundle electron main files"
time yarn run webpack --config-name main

echo "--- Starting proxy daemon and runing app tests"
time ELECTRON_ENABLE_LOGGING=1 yarn test

if [[ "${BUILDKITE_BRANCH:-}" == "master" || -n "${BUILDKITE_TAG:-}" ]]; then
  echo "--- Packaging and uploading app binaries"
  time yarn dist
fi
