FROM jakmeier/paddlers:builder-base as GameMasterBuilder
# Install diesel CLI
RUN cargo install diesel_cli
RUN mkdir -p /out && cp /usr/local/cargo/bin/diesel /out/
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
RUN cargo build --manifest-path=paddlers-game-master/Cargo.toml
# Now replace shallow projects with actual source code and build again
# First, the shared lib only to add another layer of caching
RUN rm ./paddlers-shared-lib/src/*.rs
RUN rm ./paddlers-game-master/target/debug/deps/paddlers_shared*
RUN rm ./paddlers-game-master/target/debug/deps/libpaddlers_shared*
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./migrations ./migrations
RUN cargo build --manifest-path=paddlers-game-master/Cargo.toml
# Second, the application binary
RUN rm ./paddlers-game-master/src/*.rs
COPY ./paddlers-game-master/src ./paddlers-game-master/src
COPY ./specification-loader/src ./specification-loader/src
RUN rm ./paddlers-game-master/target/debug/deps/paddlers_game*
RUN cargo build --manifest-path=paddlers-game-master/Cargo.toml --features=local_test
RUN cargo build --manifest-path=specification-loader/Cargo.toml


FROM buildpack-deps:stretch as GameMaster
WORKDIR /app
COPY --from=GameMasterBuilder ./paddlers-game-master/target/debug/paddlers-game-master ./paddlers-game-master
COPY --from=GameMasterBuilder ./specification-loader/target/debug/specification-loader ./specification-loader
COPY ./diesel.toml ./diesel.toml
COPY ./specification ./specification
# Customize env file later if you need to 
COPY ./local.env ./.env
# Local build also needs a RSA key (must match keycloak setup)
RUN mkdir /opt/keycloak
COPY ./paddlers-keycloak/debug_pub_rsa.der /opt/keycloak/pub_rsa.der
# Copy diesel CLI binary
COPY --from=GameMasterBuilder /out/diesel /bin/
COPY --from=GameMasterBuilder ./migrations ./migrations
# If RESET_DB has been defiend, rerun diesel migrations before starting paddlers-game-master
CMD [ ! -z "$RESET_DB" ] && while diesel migration revert; do :; done; diesel migration run; ./paddlers-game-master