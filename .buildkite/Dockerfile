FROM gcr.io/opensourcecoin/radicle-registry/ci-base

RUN apt-get -y update

# install cypress deps
RUN apt-get -y install autoconf git nettle-dev m4 gnupg xvfb libgtk-3-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2

# install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get -y update
RUN apt-get -y install yarn
RUN rustup component add clippy --toolchain nightly-2019-11-17-x86_64-unknown-linux-gnu
