FROM debian:buster-slim

# System packages
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    clang \
    curl \
    gcc \
    git \
    jq \
    libc6-dev \
    libssl-dev \
    make \
    pkg-config

# install cypress deps
RUN apt-get -y install autoconf git nettle-dev m4 gnupg xvfb libgtk-3-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2

# install npm
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get -y update
RUN apt-get -y install build-essential nodejs

# install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get -y update
RUN apt-get -y install yarn

RUN apt-get autoremove; \
    rm -rf /var/lib/apt/lists/*

# Rust toolchain
# Make sure this is in sync with rust-toolchain!
ENV RUST_VERSION=nightly-2020-10-05 \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_HOME=/usr/local/rustup \
    RUSTUP_VERSION=1.21.1 \
    RUSTUP_SHA256=ad1f8b5199b3b9e231472ed7aa08d2e5d1d539198a15c5b1e53c746aad81d27b

RUN set -eux; \
    curl -LOf "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init"; \
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
