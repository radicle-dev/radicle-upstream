#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

if [[ "${BUILDKITE_AGENT_META_DATA_PLATFORM}" == "macos" ]]; then
  echo "--- Setting up macOS environment"

  export HOME=/Users/buildkite
  export CACHE_FOLDER="$HOME/buildkite-cache"

  echo "CACHE_FOLDER=$CACHE_FOLDER"
  echo "HOME=$HOME"
else
  echo "--- Setting up Linux environment"

  export CACHE_FOLDER="/cache"
  export HOME="$CACHE_FOLDER"

  echo "CACHE_FOLDER=$CACHE_FOLDER"
  echo "HOME=$HOME"
fi

mkdir -p "$CACHE_FOLDER/yarn"
mkdir -p "$CACHE_FOLDER/cargo"
mkdir -p "$CACHE_FOLDER/rustup"

export YARN_CACHE_FOLDER="$CACHE_FOLDER/yarn"
export CARGO_HOME="$CACHE_FOLDER/cargo"
export RUSTUP_HOME="$CACHE_FOLDER/rustup"

chmod -R a+w $CARGO_HOME $RUSTUP_HOME

export PATH="$HOME/.cargo/bin:$PATH"

# Incremental builds use timestamps of local code. Since we always
# check it out fresh we can never use incremental builds.
export CARGO_BUILD_INCREMENTAL=false

# Most of the caching is done through caching ./target
export SCCACHE_CACHE_SIZE="1G"

echo "--- Installing yarn dependencies"
(cd app && time yarn install)

echo "--- Loading proxy/target cache"
declare -r target_cache="$CACHE_FOLDER/proxy-target"

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

echo "--- Set custom git config"
(cp .buildkite/.gitconfig "$CACHE_FOLDER/")
cat "$CACHE_FOLDER/.gitconfig"

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
