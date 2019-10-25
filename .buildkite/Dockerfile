FROM rust:1.38

RUN apt-get -y update
# install cypress deps
RUN apt-get -y install xvfb libgtk-3-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2

# install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get -y update
RUN apt-get -y install yarn

RUN yarn --version
