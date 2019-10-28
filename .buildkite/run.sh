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

cd app

echo "--- install yarn deps"
yarn install

echo "--- build proxy"
yarn proxy:build

echo "--- start proxy in background and run app tests"
yarn run-p --race proxy:start ci:test

echo "--- package and upload app binaries"
yarn ci:dist

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
