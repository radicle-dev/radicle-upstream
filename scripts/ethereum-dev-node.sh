#!/usr/bin/env bash
#
# Run a local ethereum node and deploy the latest contracts to it.
#

set -eumo pipefail

yarn run ganache-cli \
  --mnemonic "image napkin cruise dentist name plunge crisp muscle nest floor vessel blush" \
  --defaultBalanceEther 1000 \
  "$@" &

function stop_ganache() {
  kill %1 2>/dev/null || true
  fg %1 2>/dev/null || true
}

trap stop_ganache SIGINT EXIT SIGTERM

sleep 4

echo "Deploying the Radicle Dev Contracts..."
./scripts/deploy-dev-contracts.js;
echo "Done"

echo "Adding funds to your account..."
yarn run ethers --rpc http://localhost:8545 --account-rpc 0 --yes send $(< ./sandbox/.local-eth-account) 10
echo "Done"

fg %1
