FROM debian:buster-slim

# System packages
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    clang \
    curl \
    git \
    jq \
    libssl-dev \
    pkg-config \
    procps

# install cypress deps
RUN apt-get -y install --no-install-recommends autoconf git nettle-dev m4 gnupg xvfb libgtk-3-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2

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
ENV RUST_VERSION=nightly-2021-01-21 \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_HOME=/usr/local/rustup \
    RUSTUP_VERSION=1.23.1 \
    RUSTUP_SHA256=ed7773edaf1d289656bdec2aacad12413b38ad0193fff54b2231f5140a4b07c5

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
    rustup component add clippy rustfmt;

VOLUME /cache
ENV CARGO_HOME=/cache/cargo
