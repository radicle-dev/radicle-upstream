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

export PATH="$PATH:CARGO_HOME/bin"

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

echo "--- Building proxy"
(cd app && ELECTRON_ENABLE_LOGGING=1 yarn proxy:build)

echo "--- Saving proxy/target cache"
rm -rf "$target_cache"
cp -aTu proxy/target "$target_cache"
echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"

echo "--- Updateing submodules"
(cd app && git submodule update --init --recursive)

(cd app && git submodule foreach "git fetch --all")

echo "--- Starting proxy daemon and runing app tests"
(cd app && yarn test)

echo "--- Packaging and uploading app binaries"
(cd app && yarn ci:dist)
