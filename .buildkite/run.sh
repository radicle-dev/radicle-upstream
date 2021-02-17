#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

source .buildkite/env.sh

echo "--- Removing old Yarn temp dir"
du -hs "$YARN_TEMPDIR"
rm -rf "$YARN_TEMPDIR"
mkdir -p "$YARN_TEMPDIR"

echo "--- Installing yarn dependencies"
time TMPDIR="$YARN_TEMPDIR" yarn install --frozen-lockfile

echo "--- Loading proxy target cache"
declare -r target_cache="$CACHE_FOLDER/proxy-target"

mkdir -p "$target_cache"

if [[ -d "$target_cache" ]]; then
	ln -s "$target_cache" ./target
  echo "Size of $target_cache is $(du -sh "$target_cache" | cut -f 1)"
else
  echo "Cache $target_cache not available"
fi

echo "--- Updating submodules"
./scripts/test-setup.sh

echo "--- Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"

echo "--- Run proxy docs"
(
  export RUSTDOCFLAGS="-D intra-doc-link-resolution-failure"
  time cargo doc --workspace --no-deps --all-features --document-private-items
)

echo "--- Run proxy fmt"
time cargo fmt --all -- --check

echo "--- Run proxy lints"
time cargo clippy --all --all-features --all-targets -Z unstable-options -- --deny warnings

echo "--- Run app eslint checks"
time yarn lint

echo "--- Run app prettier checks"
time yarn prettier:check

echo "--- Check TypeScript"
time yarn typescript:check

echo "--- Run proxy tests"
(
  export RUST_TEST_TIME_UNIT=2000,4000
  export RUST_TEST_TIME_INTEGRATION=2000,8000
  cargo build --tests --all --all-features --all-targets
  timeout 6m cargo test --all --all-features --all-targets -- -Z unstable-options --report-time
)

echo "--- Starting proxy daemon and runing app tests"
time ELECTRON_ENABLE_LOGGING=1 yarn test

if [[ "${BUILDKITE_BRANCH:-}" == "master" || -n "${BUILDKITE_TAG:-}" ]]; then
  echo "--- Packaging and uploading app binaries"
  time yarn dist
fi
