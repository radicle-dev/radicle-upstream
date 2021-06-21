#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

# Download the Twemoji SVGs and put them into public/twemoji

set -Eeou pipefail

version="$(node -e 'console.log(require("twemoji/package.json").version)')"

echo "Installing Twemoji SVG assets v${version}"

curl -sSL "https://github.com/twitter/twemoji/archive/refs/tags/v${version}.tar.gz" \
  | tar -x -z -C public/twemoji/ --strip-components=3 "twemoji-${version}/assets/svg"
