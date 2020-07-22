#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

source .buildkite/env.sh

echo "--- Installing yarn dependencies"
time yarn install

echo "--- Loading proxy/target cache"
declare -r target_cache="$CACHE_FOLDER/proxy-target"

mkdir -p "$target_cache"

if [[ -d "$target_cache" ]]; then
	ln -s "$target_cache" proxy/target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- Updating submodules"
time git submodule update --remote --init --recursive
time git submodule foreach "git fetch --all"
time git submodule foreach "git checkout -B dev -t origin/dev"
time git submodule foreach "git checkout master"
time git submodule foreach "git pull origin master"

echo "--- Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"

echo "--- Run proxy tests"
(cd proxy && time cargo test --all --all-features --all-targets)

echo "--- Run cargo fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy lints"
(cd proxy && time cargo clippy --all --all-features --all-targets)

echo "--- Run proxy docs"
(cd proxy && time cargo doc --no-deps)

echo "--- Run app eslint checks"
time yarn lint

echo "--- Run app prettier checks"
time yarn prettier:check

echo "--- Run app svelte checks"
time yarn svelte:check

echo "--- Starting proxy daemon and runing app tests"
time ELECTRON_ENABLE_LOGGING=1 yarn test

if [[ "${BUILDKITE_BRANCH:-}" == "master" ]]; then
  echo "--- Packaging and uploading app binaries"
  time yarn dist
fi
