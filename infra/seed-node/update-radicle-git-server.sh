#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

systemctl stop radicle-git-server

curl -fsSL \
  "https://storage.googleapis.com/radicle-client-services/radicle-git-server" \
  -o /usr/local/bin/radicle-git-server
chmod +x /usr/local/bin/radicle-git-server

systemctl start radicle-git-server
