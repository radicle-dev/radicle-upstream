FROM gcr.io/opensourcecoin/radicle-registry/ci-base@sha256:18d8cf1e39f4f42d871a2a9d3bcd8641bfda343a1fc7b9fbca4f8aea422bae6f

RUN apt-get -y update

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
RUN rustup component add clippy --toolchain nightly-2020-02-05-x86_64-unknown-linux-gnu
