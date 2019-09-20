#!/usr/bin/env bash
set -eou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn
yarn test
