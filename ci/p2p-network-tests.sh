#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

source ci/env.sh

log-group-start "yarn install"
# Unsetting GITHUB_ACTIONS because yarn tries to add log groups in a buggy way.
env -u GITHUB_ACTIONS yarn install --immutable
env -u GITHUB_ACTIONS yarn dedupe --check
log-group-end

log-group-start "cargo build"
cargo build --all --all-features --all-targets
log-group-end

log-group-start "maintainer-update-propagation-1-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/maintainer-update-propagation-1-test.ts
log-group-end

log-group-start "maintainer-update-propagation-2-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/maintainer-update-propagation-2-test.ts
log-group-end

log-group-start "maintainer-update-propagation-3-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/maintainer-update-propagation-3-test.ts
log-group-end

log-group-start "active-sets-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/active-sets-test.ts
log-group-end

log-group-start "contributor-fork-replication-1-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/contributor-fork-replication-1-test.ts
log-group-end

log-group-start "contributor-fork-replication-2-test.ts"
time sudo -E FORCE_COLOR=1 ./p2p-tests/contributor-fork-replication-2-test.ts
log-group-end
