#!/bin/bash
set -Eeou pipefail

export HOME=/cache

mkdir -p /cache/yarn
mkdir -p /cache/cargo
mkdir -p /cache/rustup

export YARN_CACHE_FOLDER=/cache/yarn
export CARGO_HOME=/cache/cargo
export RUSTUP_HOME=/cache/rustup

chmod -R a+w $CARGO_HOME $RUSTUP_HOME

export PATH="$PATH:$CARGO_HOME/bin"

echo "--- Install clippy"
(cd proxy && rustup component add clippy --toolchain nightly-2019-11-17-x86_64-unknown-linux-gnu)

echo "--- Installing yarn dependencies"
(cd app && yarn install)

echo "--- Loading proxy/target cache"
target_cache=/cache/radicle-upstream-proxy-target-cache

if [[ -d "$target_cache" ]]; then
  cp -aT "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- Updateing submodules"
(cd app && git submodule update --init --recursive)
(cd app && git submodule foreach "git fetch --all")

echo "--- Set custom git config"
(cp .buildkite/.gitconfig /cache/)

echo "--- Build proxy"
(cd app && yarn proxy:build)

echo "--- Build proxy release"
(cd app && yarn proxy:build:release)

echo "--- Build proxy test"
(cd app && yarn proxy:build:test)

echo "--- Saving proxy/target cache"
rm -rf "$target_cache"
cp -aTu proxy/target "$target_cache"
echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"

echo "--- Run proxy tests"
(cd proxy && cargo test --all-features --all-targets)

echo "--- Run proxy lints"
(cd proxy && cargo clippy --all-features --all-targets)

echo "--- Starting proxy daemon and runing app tests"
(cd app && ELECTRON_ENABLE_LOGGING=1 yarn test)

echo "--- Packaging and uploading app binaries"
(cd app && yarn ci:dist)
(cd app && yarn ci:dist)
