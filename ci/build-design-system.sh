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

version="$(node -e 'console.log(require("twemoji/package.json").version)')"

log-group-start "Installing Twemoji SVG assets v${version}"
curl -fsSL "https://github.com/twitter/twemoji/archive/refs/tags/v${version}.tar.gz" \
  | tar -x -z -C design-system/build/twemoji/ --strip-components=3 "twemoji-${version}/assets/svg"
log-group-end

log-group-start "Building static showcase assets…"
webpack build --config-name design-system --mode production
log-group-end
