#!/usr/bin/env bash
set -eou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn

trap 'kill %1' EXIT
yarn server&

unset YARN_CACHE_FOLDER
export CYPRESS_RUN_BINARY=/usr/local/bin/cypress
export CYPRESS_CACHE_FOLDER=/build/.cypress
cypress run
