#!/usr/bin/env bash
set -Eeou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn
yarn test
