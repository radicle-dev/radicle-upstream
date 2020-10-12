#!/usr/bin/env bash
#
# Run a local ethereum node and deploy the latest contracts to it.
#

set -eumo pipefail

node_version=$(node --version)
if [[ ! ( ${node_version} =~ ^v12 ) ]]; then
  echo "Invalid node version ${node_version}. Please use v12"
  exit 1
fi

node_modules/.bin/ganache-cli \
  --mnemonic "image napkin cruise dentist name plunge crisp muscle nest floor vessel blush" \
  --defaultBalanceEther 1000 \
  "$@" &

function stop_ganache() {
  kill %1 2>/dev/null || true
  fg %1 2>/dev/null || true
}

trap stop_ganache SIGINT EXIT

sleep 2

./scripts/deploy-dev-contracts.js;

fg %1
