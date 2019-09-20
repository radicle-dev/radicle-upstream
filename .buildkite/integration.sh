#!/usr/bin/env bash
set -eou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn

trap 'kill %1' EXIT
yarn server&

export HOME=/root
export YARN_CACHE_FOLDER=/root/.cache
cypress run
