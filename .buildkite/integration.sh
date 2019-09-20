#!/bin/bash
set -Eeou pipefail

export HOME=/cache
export YARN_CACHE_FOLDER=/cache

cd app
yarn

yarn clean
yarn build
yarn server&

yarn wait-on -t 10000 http://localhost:8000 && echo "run the tests now"
#yarn test:cypress:run
