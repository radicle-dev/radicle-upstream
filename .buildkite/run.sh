#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

source .buildkite/env.sh

echo "--- Removing old Yarn temp dir"
du -hs "$YARN_TEMPDIR"
rm -rf "$YARN_TEMPDIR"
mkdir -p "$YARN_TEMPDIR"

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
./scripts/test-setup.sh

echo "--- Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"

(
  cd proxy
  export RUST_LOG=coco=trace,librad=trace
  export RUST_TEST_TIME_UNIT=2000,4000
  export RUST_TEST_TIME_INTEGRATION=2000,8000
  timeout 8m cargo test -p coco --test gossip can_ask_and_clone_project -- -Z unstable-options --ensure-time --nocapture
)
