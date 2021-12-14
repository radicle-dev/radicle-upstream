#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail
source "$(dirname "$0")/lib/topology.sh"

function setup_network {
  echo "setting up network: every node can reach any other node"

  create_bridge

  create_peer seed 10.0.0.1/24
  create_peer maintainer 10.0.0.101/24
  create_peer contributor 10.0.0.102/24
  create_peer contributor2 10.0.0.103/24
  create_peer contributor3 10.0.0.104/24
  create_peer contributor4 10.0.0.105/24
}

function clean_up_network {
  echo "tearing down network"

  clean_up_bridge

  set +e
  ip netns delete upstream-test-maintainer > /dev/null 2>&1
  ip netns delete upstream-test-seed > /dev/null 2>&1
  ip netns delete upstream-test-contributor > /dev/null 2>&1
  ip netns delete upstream-test-contributor2 > /dev/null 2>&1
  ip netns delete upstream-test-contributor3 > /dev/null 2>&1
  ip netns delete upstream-test-contributor4 > /dev/null 2>&1
  set -e

  return 0
}

main ${1-"--help"}
