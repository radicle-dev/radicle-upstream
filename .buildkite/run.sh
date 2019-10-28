#!/bin/bash
set -Eeou pipefail

export HOME=/cache

mkdir -p /cache/yarn
mkdir -p /cache/cargo
mkdir -p /cache/rustup

export YARN_CACHE_FOLDER=/cache/yarn
export CARGO_HOME=/cache/cargo
export RUSTUP_HOME=/cache/rustup

export PATH="$PATH:CARGO_HOME/bin"

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

echo "--- Load proxy/target cache"
target_cache=/cache/radicle-upstream-proxy-target-cache

if [[ -d "$target_cache" ]]; then
  time cp -aT "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- install yarn deps"
cd app
yarn install

echo "--- build proxy"
yarn proxy:build

echo "--- start proxy in background and run app tests"
yarn run-p --race proxy:start ci:test

echo "--- package and upload app binaries"
yarn ci:dist

echo "--- Save proxy/target cache"
cd ..
rm -rf "$target_cache"
time cp -aTu proxy/target "$target_cache"
echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"


echo "--- show debug info"
echo "yarn --version"
yarn --version
echo "yarn cypress --version"
yarn cypress --version
echo "rustup --version"
rustup --version
echo "ls -la /cache"
ls -la /cache
echo "ls -la /cache/yarn"
ls -la /cache/yarn
echo "ls -la /cache/cargo"
ls -la /cache/cargo
echo "ls -la /cache/rustup"
ls -la /cache/rustup
echo "ls -la dist"
ls -la dist
