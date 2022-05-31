# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

FROM debian:buster-slim

# System packages
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cmake \
      curl \
      git \
      jq \
      procps;
  rm -rf /var/lib/apt/lists/*;

# install node and yarn
RUN set -eux; \
  curl -sfLS https://deb.nodesource.com/setup_14.x | bash -; \
  apt-get -y update; \
  apt-get -y --no-install-recommends install nodejs; \
  apt-get autoremove; \
  rm -rf /var/lib/apt/lists/*; \
  npm install --global yarn;

# Rust toolchain
# Make sure this is in sync with rust-toolchain!
ENV RUST_VERSION=nightly-2021-06-17 \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_HOME=/usr/local/rustup \
    RUSTUP_VERSION=1.24.3 \
    RUSTUP_SHA256=3dc5ef50861ee18657f9db2eeb7392f9c2a6c95c90ab41e45ab4ca71476b4338

RUN set -eux; \
    curl -sfLSO "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init"; \
    echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    rustup component add clippy rustfmt; \
    cargo install cargo-deny; \
    rm -rf /usr/local/cargo/registry; \
    rm /usr/local/cargo/.package-cache;

VOLUME /cache
ENV CARGO_HOME=/cache/cargo
