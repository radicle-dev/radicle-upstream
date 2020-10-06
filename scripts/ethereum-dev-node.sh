#!/usr/bin/env bash

set -euo pipefail

node_version=$(node --version)
if [[ ! ( ${node_version} =~ ^v12 ) ]]; then
  echo "Invalid node version ${node_version}. Please use v12"
  exit 1
fi

exec node_modules/.bin/ganache-cli \
  --mnemonic "image napkin cruise dentist name plunge crisp muscle nest floor vessel blush" \
  --defaultBalanceEther 1000

