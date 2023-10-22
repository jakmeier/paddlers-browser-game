# Dockerfile for
# jakmeier/paddlers:frontend-builder
#
# A separate docker image for the fully built frontend which only needs to be copied to the webserver image.
# This image only exists to reduce local compilation time by permanently caching a full build of a recent version
# with all dependencies already compiled.
# Extending this image and compiling the project again takes full advantage of incremental builds.
#
# Note that this image does not have to be updated for every update of jakmeier/paddlers:frontend-snapshot
# which will build the project again from the newest source anyway.
#
FROM jakmeier/paddlers:builder-base

# Install npm (as traken from https://github.com/nodejs/docker-node/blob/7b11db1cab459beb96448e18ec421ec952fa0491/14/stretch/Dockerfile)
RUN groupadd --gid 1000 node \
  && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

ENV NODE_VERSION 16.6.0

RUN ARCH= && dpkgArch="$(dpkg --print-architecture)" \
  && case "${dpkgArch##*-}" in \
    amd64) ARCH='x64';; \
    ppc64el) ARCH='ppc64le';; \
    s390x) ARCH='s390x';; \
    arm64) ARCH='arm64';; \
    armhf) ARCH='armv7l';; \
    i386) ARCH='x86';; \
    *) echo "unsupported architecture"; exit 1 ;; \
  esac \
  # gpg keys listed at https://github.com/nodejs/node#release-keys
  && set -ex \
  && for key in \
    4ED778F539E3634C779C87C6D7062848A1AB005C \
    141F07595B7B3FFE74309A937405533BE57C7D57 \
    74F12602B6F1C4E913FAA37AD3A89613643B6201 \
    DD792F5973C6DE52C432CBDAC77ABFA00DDBF2B7 \
    8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600 \
    C4F0DFFF4E8C1A8236409D08E73BC641CC11F4C8 \
    C82FA3AE1CBEDC6BE46B9360C43CEC45C17AB93C \
    890C08DB8579162FEE0DF9DB8BEAB4DFCF555EF4 \
    108F52B48DB57BB0CC439B2997B01419BD92F80A \
  ; do \
    gpg --batch --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys "$key" || \
    gpg --batch --keyserver hkp://ipv4.pool.sks-keyservers.net --recv-keys "$key" || \
    gpg --batch --keyserver hkp://pgp.mit.edu:80 --recv-keys "$key" ; \
  done \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \
  && grep " node-v$NODE_VERSION-linux-$ARCH.tar.xz\$" SHASUMS256.txt | sha256sum -c - \
  && tar -xJf "node-v$NODE_VERSION-linux-$ARCH.tar.xz" -C /usr/local --strip-components=1 --no-same-owner \
  && rm "node-v$NODE_VERSION-linux-$ARCH.tar.xz" SHASUMS256.txt.asc SHASUMS256.txt \
  && ln -s /usr/local/bin/node /usr/local/bin/nodejs \
  # smoke tests
  && node --version \
  && npm --version

# Install wasm-pack
RUN USER=root cargo install wasm-pack
# Create all folders
RUN mkdir www; mkdir paddlers-shared-lib; mkdir paddlers-frontend
# Build npm package
COPY ./www ./www
RUN cd www; npm install 
# Copy all othersource code
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./paddlers-frontend/api ./paddlers-frontend/api
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
COPY ./div-rs ./div-rs
COPY ./nuts ./nuts
COPY ./paddle ./paddle
COPY ./Cargo.lock ./paddlers-frontend/
# Build project
RUN cd paddlers-frontend; wasm-pack build