# -*- mode: ruby -*-
# vi: set ft=ruby :

# Use this Vagrantfile to run p2p-tests on an Intel macOS host.
# This requires VirtualBox to be installed.
#
#   brew install vagrant
#   vagrant up
#   vagrant ssh
#     cd /vagrant
#     cargo build
#     cd p2p-tests
#     sudo -E ./contributor-fork-replication-1-test.ts

Vagrant.configure("2") do |config|
  config.vm.provider "virtualbox" do |v|
    v.memory = 8192
    v.cpus = 4
  end

  config.vm.box = "ubuntu/focal64"
  config.vm.provision "shell", inline: <<~SHELL
    export DEBIAN_FRONTEND=noninteractive
    apt-get -y update
    apt-get -y upgrade
    apt-get -y --no-install-recommends install \
      build-essential \
      ca-certificates \
      cmake \
      curl \
      git \
      jq \
      procps; \

    curl -sfLS https://deb.nodesource.com/setup_16.x | bash -; \
    apt-get -y update; \
    apt-get -y --no-install-recommends install nodejs; \
    apt-get autoremove; \
    npm install --global yarn;

    curl https://sh.rustup.rs -sSfL | sudo -u vagrant sh -s -- -y --profile minimal

    # Setting cargo target directory to point outside of this repository, so
    # the Linux and macOS binaries don't end up in the same target folder.
    # This also prevents cargo warnings about hardlinking in the guest.
    echo "export CARGO_TARGET_DIR=/home/vagrant/target" >> /home/vagrant/.bashrc
  SHELL
end
