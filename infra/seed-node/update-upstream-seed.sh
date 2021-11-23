#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

if [[ -n "${1:-}" ]]; then
  path="by-commit/${1}"
else
  path="main"
fi

systemctl stop upstream-seed
curl -fsSL \
  "https://storage.googleapis.com/radicle-upstream-build-artifacts/v1/${path}/x86_64-linux/upstream-seed" \
  -o /usr/local/bin/upstream-seed
chmod +x /usr/local/bin/upstream-seed

systemctl start upstream-seed
