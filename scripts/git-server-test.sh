# Copyright © 2022 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

#!/usr/bin/env bash

radicle_client_services_commit=0cbc2212494f9e26e227729a584838f5e9772305
exec docker run \
  --init \
  --publish 8778:8778 \
  --rm \
  --name upstream-git-server-test \
  "$@" \
  "gcr.io/radicle-services/git-server:$radicle_client_services_commit" \
  --allow-unauthorized-keys
