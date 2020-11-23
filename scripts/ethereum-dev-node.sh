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

sleep 4

echo "Deploying the Radicle Dev Contracts..."
./scripts/deploy-dev-contracts.js;
echo "Done"

echo "Adding funds to your account..."
ethers --rpc http://localhost:8545 --account-rpc 0 send $(< ./sandbox/.local-eth-account) 10
echo "Done"

fg %1
