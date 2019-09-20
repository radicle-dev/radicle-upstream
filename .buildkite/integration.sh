#!/bin/bash
set -Eeou pipefail
trap 'kill %1' EXIT

exit 1

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn

yarn clean
yarn build
yarn server&

#yarn test:cypress:run
