#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

export HOME=/cache

mkdir -p /cache/yarn
mkdir -p /cache/cargo
mkdir -p /cache/rustup

export YARN_CACHE_FOLDER=/cache/yarn
export CARGO_HOME=/cache/cargo
export RUSTUP_HOME=/cache/rustup

chmod -R a+w $CARGO_HOME $RUSTUP_HOME

export PATH="$PATH:$CARGO_HOME/bin"

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false
# Most of the caching is done through caching ./target
export SCCACHE_CACHE_SIZE="1G"

echo "--- Installing yarn dependencies"
(cd app && time yarn install)

echo "--- Loading proxy/target cache"
declare -r target_cache=/cache/proxy-target

mkdir -p "$target_cache"

if [[ -d "$target_cache" ]]; then
	ln -s "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- Updateing submodules"
(cd app && time git submodule update --init --recursive)
(cd app && time git submodule foreach "git fetch --all")

echo "--- Set custom git config"
(cp .buildkite/.gitconfig /cache/)

echo "--- Run cargo fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy tests"
(cd proxy && time cargo test --all --all-features --all-targets)

echo "--- Run proxy lints"
(cd proxy && time cargo check --all --all-features --all-targets)
(cd proxy && time cargo clippy --all --all-features --all-targets)

echo "--- Starting proxy daemon and runing app tests"
(cd app && time ELECTRON_ENABLE_LOGGING=1 yarn test)

echo "--- Build proxy release"
(cd app && time yarn proxy:build:release)

echo "--- Packaging and uploading app binaries"
(cd app && time yarn ci:dist)
