#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

export HOME=/Users/buildkite
export PATH="$HOME/.cargo/bin:$PATH"

export BUILDKITE_CACHE="$HOME/buildkite-cache"

export YARN_CACHE_FOLDER="$BUILDKITE_CACHE/yarn"
export CARGO_HOME="$BUILDKITE_CACHE/cargo"
export RUSTUP_HOME="$BUILDKITE_CACHE/rustup"

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false
# Most of the caching is done through caching ./target
export SCCACHE_CACHE_SIZE="1G"

echo "--- Installing yarn dependencies"
(cd app && time yarn install)

echo "--- Loading proxy/target cache"
declare -r target_cache="$BUILDKITE_CACHE/proxy-target"

mkdir -p "$target_cache"

if [[ -d "$target_cache" ]]; then
	ln -s "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

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
