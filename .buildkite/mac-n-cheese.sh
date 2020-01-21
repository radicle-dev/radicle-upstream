#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

env

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false
# Most of the caching is done through caching ./target
export SCCACHE_CACHE_SIZE="1G"

echo "--- Installing yarn dependencies"
(cd app && time yarn install)

echo "--- Updating submodules"
(cd app && time git submodule update --init --recursive)
(cd app && time git submodule foreach "git fetch --all")

echo "--- Run cargo fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy tests"
(cd proxy && time cargo test --all --all-features --all-targets)

echo "--- Run proxy lints"
(cd proxy && time cargo check --all --all-features --all-targets)
(cd proxy && time cargo clippy --all --all-features --all-targets)

echo "--- Starting proxy daemon and runing app tests"
(cd app && time ELECTRON_ENABLE_LOGGING=1 yarn test)

echo "--- Packaging and uploading app binaries"
(cd app && time yarn dist)
