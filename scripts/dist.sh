#!/bin/bash

# Copyright © 2022 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

rm -rf ./dist
mkdir ./dist

yarn run webpack build --mode production
cargo build --release --bins
yarn run electron-builder --publish never
