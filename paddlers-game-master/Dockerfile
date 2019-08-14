FROM rust:nightly as GameMasterBuilder
# Build only dependencies first to allow Docker's image caching to kick in
RUN \
# With selected nightly, there is a bug in cargo new, therefore cargo init is used here
mkdir paddlers-shared-lib; \
mkdir paddlers-game-master; \
USER=root cargo init --lib paddlers-shared-lib; \
USER=root cargo init --bin paddlers-game-master
COPY ./paddlers-game-master/Cargo.toml ./paddlers-game-master/
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
RUN rm ./paddlers-game-master/target/debug/deps/paddlers_game*
RUN cargo build --manifest-path=paddlers-game-master/Cargo.toml


FROM buildpack-deps:stretch as GameMaster
WORKDIR /app
COPY --from=GameMasterBuilder ./paddlers-game-master/target/debug/paddlers-game-master ./paddlers-game-master
COPY ./diesel.toml ./diesel.toml
COPY ./.env ./.env
CMD ["./paddlers-game-master"]