#!/bin/bash

# Download the Twemoji SVGs and put them into public/twemoji

set -Eeou pipefail

version="$(yarn --silent info twemoji version)"

echo "Installing Twemoji SVG assets v${version}"

curl -sSL "https://github.com/twitter/twemoji/archive/refs/tags/v${version}.tar.gz" \
  | tar -x -z -C public/twemoji/ --strip-components=3 "twemoji-${version}/assets/svg"
