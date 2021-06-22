FROM debian:buster-slim

# System packages
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      curl \
      git \
      jq \
      procps \
      # Cypress deps
      # See https://github.com/cypress-io/cypress-docker-images/blob/422d5f00ca2bfcaa3b31b347f536817809500f96/base/14.16.0/Dockerfile#L10
      libgtk2.0-0 \
      libgtk-3-0 \
      libnotify-dev \
      libgconf-2-4 \
      libgbm-dev \
      libnss3 \
      libxss1 \
      libasound2 \
      libxtst6 \
      xauth \
      xvfb \
      fonts-noto-color-emoji \
      fonts-arphic-bkai00mp \
      fonts-arphic-bsmi00lp \
      fonts-arphic-gbsn00lp \
      fonts-arphic-gkai00mp \
      fonts-arphic-ukai \
      fonts-arphic-uming \
      ttf-wqy-zenhei \
      ttf-wqy-microhei \
      xfonts-wqy; \
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
ENV RUST_VERSION=nightly-2021-05-09 \
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
