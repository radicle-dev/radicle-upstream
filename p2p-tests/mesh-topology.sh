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

  ip netns add upstream-test-maintainer
  ip netns add upstream-test-seed
  ip netns add upstream-test-contributor
  ip netns add upstream-test-contributor2
  ip netns add upstream-test-contributor3
  ip netns add upstream-test-contributor4

  ip link add maint.host type veth peer name maint.bridge
  ip link add contr.host type veth peer name contr.bridge
  ip link add contr2.host type veth peer name contr2.bridge
  ip link add contr3.host type veth peer name contr3.bridge
  ip link add contr4.host type veth peer name contr4.bridge
  ip link add seed.host type veth peer name seed.bridge

  ip link add test-bridge type bridge

  ip link set maint.host netns upstream-test-maintainer
  ip link set contr.host netns upstream-test-contributor
  ip link set contr2.host netns upstream-test-contributor2
  ip link set contr3.host netns upstream-test-contributor3
  ip link set contr4.host netns upstream-test-contributor4
  ip link set seed.host netns upstream-test-seed

  ip addr add 10.0.0.254/24 dev test-bridge
  ip link set dev test-bridge up

  ip link set dev maint.bridge master test-bridge
  ip link set dev contr.bridge master test-bridge
  ip link set dev contr2.bridge master test-bridge
  ip link set dev contr3.bridge master test-bridge
  ip link set dev contr4.bridge master test-bridge
  ip link set dev seed.bridge master test-bridge

  ip link set dev maint.bridge up
  ip link set dev contr.bridge up
  ip link set dev contr2.bridge up
  ip link set dev contr3.bridge up
  ip link set dev contr4.bridge up
  ip link set dev seed.bridge up

  ip netns exec upstream-test-maintainer ip addr add 10.0.0.101/24 dev maint.host
  ip netns exec upstream-test-maintainer ip link set dev maint.host up

  ip netns exec upstream-test-contributor ip addr add 10.0.0.102/24 dev contr.host
  ip netns exec upstream-test-contributor ip link set dev contr.host up

  ip netns exec upstream-test-contributor2 ip addr add 10.0.0.103/24 dev contr2.host
  ip netns exec upstream-test-contributor2 ip link set dev contr2.host up

  ip netns exec upstream-test-contributor3 ip addr add 10.0.0.104/24 dev contr3.host
  ip netns exec upstream-test-contributor3 ip link set dev contr3.host up

  ip netns exec upstream-test-contributor4 ip addr add 10.0.0.105/24 dev contr4.host
  ip netns exec upstream-test-contributor4 ip link set dev contr4.host up

  ip netns exec upstream-test-seed ip addr add 10.0.0.1/24 dev seed.host
  ip netns exec upstream-test-seed ip link set dev seed.host up

  ip netns exec upstream-test-maintainer ip route flush all
  ip netns exec upstream-test-contributor ip route flush all
  ip netns exec upstream-test-contributor2 ip route flush all
  ip netns exec upstream-test-contributor3 ip route flush all
  ip netns exec upstream-test-contributor4 ip route flush all
  ip netns exec upstream-test-seed ip route flush all

  ip netns exec upstream-test-maintainer ip route add 10.0.0.0/24 dev maint.host
  ip netns exec upstream-test-contributor ip route add 10.0.0.0/24 dev contr.host
  ip netns exec upstream-test-contributor2 ip route add 10.0.0.0/24 dev contr2.host
  ip netns exec upstream-test-contributor3 ip route add 10.0.0.0/24 dev contr3.host
  ip netns exec upstream-test-contributor4 ip route add 10.0.0.0/24 dev contr4.host
  ip netns exec upstream-test-seed ip route add 10.0.0.0/24 dev seed.host
}

function clean_up_network {
  echo "tearing down network"
  set +e

  ip netns delete upstream-test-maintainer > /dev/null 2>&1
  ip netns delete upstream-test-seed > /dev/null 2>&1
  ip netns delete upstream-test-contributor > /dev/null 2>&1
  ip netns delete upstream-test-contributor2 > /dev/null 2>&1
  ip netns delete upstream-test-contributor3 > /dev/null 2>&1
  ip netns delete upstream-test-contributor4 > /dev/null 2>&1

  ip link delete maint.host > /dev/null 2>&1
  ip link delete maint.bridge > /dev/null 2>&1
  ip link delete contr.host > /dev/null 2>&1
  ip link delete contr.bridge > /dev/null 2>&1
  ip link delete contr2.host > /dev/null 2>&1
  ip link delete contr2.bridge > /dev/null 2>&1
  ip link delete contr3.host > /dev/null 2>&1
  ip link delete contr3.bridge > /dev/null 2>&1
  ip link delete contr4.host > /dev/null 2>&1
  ip link delete contr4.bridge > /dev/null 2>&1
  ip link delete seed.host > /dev/null 2>&1
  ip link delete seed.bridge > /dev/null 2>&1

  ip link delete test-bridge > /dev/null 2>&1

  set -e
  return 0
}

main ${1-"--help"}
