#!/bin/bash
set -Eeou pipefail

TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'

echo "--- Run cargo fmt"
(cd proxy && time cargo fmt --all -- --check)

echo "--- Run proxy lints"
(cd proxy && time cargo clippy --all --all-features --all-targets)

echo "--- Run proxy docs"
(cd proxy && time cargo doc --no-deps)
