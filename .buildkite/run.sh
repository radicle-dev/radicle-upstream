#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

source .buildkite/env.sh

echo "--- Removing old Yarn temp dir"
du -hs "$YARN_TEMPDIR"
rm -rf "$YARN_TEMPDIR"
mkdir -p "$YARN_TEMPDIR"

echo "--- Installing yarn dependencies"
time TMPDIR="$YARN_TEMPDIR" yarn install

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
time git submodule update --init --remote
time git submodule foreach "git fetch --all"
time git submodule foreach "git checkout -B dev -t origin/dev"
time git submodule foreach "git checkout master"
time git submodule foreach "git pull origin master"

echo "--- Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"

echo "--- Run proxy docs"
(cd proxy && time cargo doc --no-deps)

echo "--- Run proxy fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy lints"
(cd proxy && time cargo clippy --all --all-features --all-targets -Z unstable-options -- --deny warnings)

echo "--- Run app eslint checks"
time yarn lint

echo "--- Run app prettier checks"
time yarn prettier:check

echo "--- Run app svelte checks"
time yarn svelte:check

echo "--- Run proxy tests"
(cd proxy && time cargo test --all --all-features --all-targets)

echo "--- Starting proxy daemon and runing app tests"
time ELECTRON_ENABLE_LOGGING=1 yarn test

if [[ "${BUILDKITE_BRANCH:-}" == "master" ]]; then
  echo "--- Packaging and uploading app binaries"
  time yarn dist
fi
