#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail
source "$(dirname "$0")/lib/topology.sh"

function setup_network {
  echo "setting up network: maintainer <==> seed <==> contributor"

  create_bridge

  create_peer seed 10.0.0.1/24
  create_peer maintainer 10.0.0.101/32
  create_peer contributor 10.0.0.102/32

  ip netns exec upstream-test-maintainer ip route add 10.0.0.1/32 dev macv
  ip netns exec upstream-test-maintainer ip route add 10.0.0.254/32 dev macv

  ip netns exec upstream-test-contributor ip route add 10.0.0.1/32 dev macv
  ip netns exec upstream-test-contributor ip route add 10.0.0.254/32 dev macv
}

function clean_up_network {
  set +e
  ip netns delete upstream-test-maintainer > /dev/null 2>&1
  ip netns delete upstream-test-seed > /dev/null 2>&1
  ip netns delete upstream-test-contributor > /dev/null 2>&1
  set -e

  clean_up_bridge
  return 0
}

main ${1-"--help"}
