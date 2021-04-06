#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

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
export PATH="$HOME/.cargo/bin:$PATH"

log-group-start "Installing yarn dependencies"
yarn install --immutable
log-group-end

log-group-start "Loading proxy target cache"
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
log-group-end

log-group-start "Updating submodules"
./scripts/test-setup.sh
log-group-end

log-group-start "Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"
log-group-end

log-group-start "Run proxy docs"
(
  export RUSTDOCFLAGS="-D intra-doc-link-resolution-failure"
  time cargo doc --workspace --no-deps --all-features --document-private-items
)
log-group-end

log-group-start "Run proxy fmt"
time cargo fmt --all -- --check
log-group-end

log-group-start "Run proxy lints"
time cargo clippy --all --all-features --all-targets -Z unstable-options -- --deny warnings
log-group-end

log-group-start "Run app eslint checks"
time yarn lint
log-group-end

log-group-start "Run app prettier checks"
time yarn prettier:check
log-group-end

log-group-start "Check TypeScript"
time yarn typescript:check
log-group-end

log-group-start "Run proxy tests"
(
  export RUST_TEST_TIME_UNIT=2000,4000
  export RUST_TEST_TIME_INTEGRATION=2000,8000
  cargo build --tests --all --all-features --all-targets
  timeout 6m cargo test --all --all-features --all-targets -- -Z unstable-options --report-time
)
log-group-end

log-group-start "Bundle electron main files"
time yarn run webpack --config-name main
log-group-end

log-group-start "Starting proxy daemon and runing app tests"
time ELECTRON_ENABLE_LOGGING=1 yarn test
log-group-end

if [[ "${BUILDKITE_BRANCH:-}" == "master" || -n "${BUILDKITE_TAG:-}" ]]; then
  log-group-start "Packaging and uploading app binaries"
  time yarn dist
  log-group-end
fi
