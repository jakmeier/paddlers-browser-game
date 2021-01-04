FROM jakmeier/paddlers:builder-base as GameMasterBuilder
# Build only dependencies first to allow Docker's image caching to kick in
RUN \
# With selected nightly, there is a bug in cargo new, therefore cargo init is used here
mkdir paddlers-shared-lib; \
mkdir paddlers-game-master; \
mkdir specification-loader; \
USER=root cargo init --lib paddlers-shared-lib; \
USER=root cargo init --bin paddlers-game-master
COPY ./paddlers-game-master/Cargo.toml ./paddlers-game-master/
COPY ./specification-loader/Cargo.toml ./specification-loader/
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./Cargo.lock ./paddlers-game-master/
# Only one compilation since this is the docker file to run on releases, usually on Dockerhub
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./specification-loader/src ./specification-loader/src
COPY ./migrations ./migrations
COPY ./paddlers-game-master/src ./paddlers-game-master/src
RUN cargo build --manifest-path=paddlers-game-master/Cargo.toml --release
RUN cargo build --manifest-path=specification-loader/Cargo.toml --release


FROM buildpack-deps:stretch as GameMaster
WORKDIR /app
COPY --from=GameMasterBuilder ./paddlers-game-master/target/release/paddlers-game-master ./paddlers-game-master
COPY --from=GameMasterBuilder ./specification-loader/target/release/specification-loader ./specification-loader
COPY ./diesel.toml ./diesel.toml
COPY ./specification ./specification
RUN mkdir /opt/keycloak
COPY ./paddlers-keycloak/demo_pub_rsa.der /opt/keycloak/pub_rsa.der
# Customize env file later if you need to 
COPY ./local.env ./.env
CMD ["./paddlers-game-master"]