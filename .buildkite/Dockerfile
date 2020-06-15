FROM gcr.io/opensourcecoin/radicle-registry/ci-base@sha256:4cf947bf233156ea67b149e7a28e318941897371cb462d3c1d6b5d424d0838a5

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
