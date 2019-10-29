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

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

echo "--- Loading proxy/target cache"
target_cache=/cache/radicle-upstream-proxy-target-cache

if [[ -d "$target_cache" ]]; then
  time cp -aT "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- Installing yarn dependencies"
cd app
yarn install

echo "--- Building proxy"
yarn proxy:build

echo "--- Starting proxy daemon and runing app tests"
yarn run-p --race proxy:start ci:test

echo "--- Packaging and uploading app binaries"
yarn ci:dist

echo "--- Saving proxy/target cache"
cd ..
rm -rf "$target_cache"
time cp -aTu proxy/target "$target_cache"
echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
