#!/usr/bin/env bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -euo pipefail

function setup_network {
  echo "setting up network: maintainer <==> seed <==> contributor"

  ip netns add upstream-test-maintainer
  ip netns add upstream-test-seed
  ip netns add upstream-test-contributor

  ip link add maint.host type veth peer name maint.bridge
  ip link add contr.host type veth peer name contr.bridge
  ip link add seed.host type veth peer name seed.bridge

  ip link add test-bridge type bridge

  ip link set maint.host netns upstream-test-maintainer
  ip link set contr.host netns upstream-test-contributor
  ip link set seed.host netns upstream-test-seed

  ip addr add 10.0.0.254/24 dev test-bridge
  ip link set dev test-bridge up

  ip link set dev maint.bridge master test-bridge
  ip link set dev contr.bridge master test-bridge
  ip link set dev seed.bridge master test-bridge

  ip link set dev maint.bridge up
  ip link set dev contr.bridge up
  ip link set dev seed.bridge up

  ip netns exec upstream-test-maintainer ip addr add 10.0.0.101/32 dev maint.host
  ip netns exec upstream-test-maintainer ip link set dev maint.host up

  ip netns exec upstream-test-contributor ip addr add 10.0.0.102/32 dev contr.host
  ip netns exec upstream-test-contributor ip link set dev contr.host up

  ip netns exec upstream-test-seed ip addr add 10.0.0.1/32 dev seed.host
  ip netns exec upstream-test-seed ip link set dev seed.host up

  ip netns exec upstream-test-maintainer ip route flush all
  ip netns exec upstream-test-contributor ip route flush all
  ip netns exec upstream-test-seed ip route flush all

  ip netns exec upstream-test-maintainer ip route add 10.0.0.1/32 dev maint.host
  ip netns exec upstream-test-maintainer ip route add 10.0.0.254/32 dev maint.host
  ip netns exec upstream-test-contributor ip route add 10.0.0.1/32 dev contr.host
  ip netns exec upstream-test-contributor ip route add 10.0.0.254/32 dev contr.host
  ip netns exec upstream-test-seed ip route add 10.0.0.101/32 dev seed.host
  ip netns exec upstream-test-seed ip route add 10.0.0.102/32 dev seed.host
  ip netns exec upstream-test-seed ip route add 10.0.0.254/32 dev seed.host
}

function clean_up_network {
  echo "tearing down network"
  set +e

  ip netns delete upstream-test-maintainer > /dev/null 2>&1
  ip netns delete upstream-test-seed > /dev/null 2>&1
  ip netns delete upstream-test-contributor > /dev/null 2>&1

  ip link delete maint.host > /dev/null 2>&1
  ip link delete maint.bridge > /dev/null 2>&1
  ip link delete contr.host > /dev/null 2>&1
  ip link delete contr.bridge > /dev/null 2>&1
  ip link delete seed.host > /dev/null 2>&1
  ip link delete seed.bridge > /dev/null 2>&1

  ip link delete test-bridge > /dev/null 2>&1

  set -e
  return 0
}

# Docker manipulates iptables in a way that prevents our network topology to
# work, this function resets all iptables rules to the defaults.
function reset_iptables {
  echo "resetting iptables to defaults"
  set +e

  iptables -P INPUT ACCEPT
  iptables -P FORWARD ACCEPT
  iptables -P OUTPUT ACCEPT
  iptables -t filter -F
  iptables -t filter -X
  iptables -t nat -F
  iptables -t nat -X
  iptables -t mangle -F
  iptables -t mangle -X
  iptables -t raw -F
  iptables -t raw -X

  set -e
  return 0
}

function status {
  echo "current networking status"
  set +e

  iptables -L -t filter
  iptables -L -t nat
  iptables -L -t mangle
  iptables -L -t raw
  iptables -L -t security

  ip addr list
  ip link list
  ip netns list

  ip route list
  ip netns exec upstream-test-maintainer ip route list
  ip netns exec upstream-test-contributor ip route list
  ip netns exec upstream-test-seed ip route list

  set -e
  return 0
}

function usage {
  cat <<EOF
Usage: sudo $(basename "${BASH_SOURCE[0]}") start | stop

This script sets up a star network topology.
It only works on Linux and _has to be run as root_.

EOF
  exit
}

function start {
  if [[ "${GITHUB_ACTIONS:-}" = "true" ]]; then
    reset_iptables
  fi
  clean_up_network
  setup_network
}

echo

if [[ "$OSTYPE" != "linux-gnu"* ]]; then
  echo -e "this script only works on Linux\n"
  exit 1
fi

if [ "$EUID" -ne 0 ]
  then usage;
  exit 1
fi

case ${1-"--help"} in
    status) status ;;
    start) start ;;
    stop) clean_up_network ;;
    --help) usage ;;
    *) usage; exit 1 ;;
esac
