
#
# Setup a local ethereum node and deploy the latest contracts to it.
#

set -m

# Start the eth dev node in the background
./scripts/ethereum-dev-node.sh &

# Give enough time for the eth node to start up
sleep 2

# Deploy the dev contracts to the running eth node
./scripts/deploy-dev-contracts.js

# Bring the running eth dev node process to the foreground.
fg %1