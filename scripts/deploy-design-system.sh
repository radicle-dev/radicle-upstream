#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

echo "This will build and deploy the design system to Github pages"
echo

read -r -p "Are you sure you want to continue? [yes/no]: " confirm
echo

if [[ $(git worktree list) != *radicle-upstream/design-system/build* ]]; then
  echo "Setting up design-system-showcase worktree in design-system/build"
  echo
  git worktree add design-system/build design-system-showcase
fi

echo "Copying Twemoji assets…"
echo
cp public/twemoji/*.svg design-system/build/twemoji

echo "Building…"
echo
webpack build --config-name design-system --mode production

echo
echo "Deploying…"
echo
cd design-system/build &&
  git add --all
  git commit -m "Deploy design-system showcase" &&
  git push origin design-system-showcase

echo
echo "Design system deployed to https://radicle-dev.github.io/radicle-upstream"
echo
