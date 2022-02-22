#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

systemctl stop radicle-http-api

curl -fsSL \
  "https://storage.googleapis.com/radicle-client-services/radicle-http-api" \
  -o /usr/local/bin/radicle-http-api
chmod +x /usr/local/bin/radicle-http-api

systemctl start radicle-http-api
