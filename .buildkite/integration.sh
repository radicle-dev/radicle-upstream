#!/bin/bash
set -Eeou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn

yarn clean
yarn build
yarn server&

exit 1
#yarn test:cypress:run
