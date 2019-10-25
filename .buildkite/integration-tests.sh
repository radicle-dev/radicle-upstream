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

yarn install
yarn proxy:build
yarn proxy:start&

yarn wait-on -t 10000 http://localhost:8080 && yarn test
