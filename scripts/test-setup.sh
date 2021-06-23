#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

git submodule update --init --remote
git submodule foreach "git fetch --all"
git submodule foreach "git checkout -B dev -t origin/dev"
git submodule foreach "git checkout master"
git submodule foreach "git pull origin master"
