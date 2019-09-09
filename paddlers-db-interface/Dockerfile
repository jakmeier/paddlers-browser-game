FROM jakmeier/paddlers:builder-base as DbInterfaceBuilder
# Build only dependencies first to allow Docker's image caching to kick in
RUN \
# With selected nightly, there is a bug in cargo new, therefore cargo init is used here
mkdir paddlers-shared-lib; \
mkdir paddlers-db-interface; \
USER=root cargo init --bin paddlers-db-interface; \
USER=root cargo init --lib paddlers-shared-lib
COPY ./paddlers-db-interface/Cargo.toml ./paddlers-db-interface/
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./Cargo.lock ./paddlers-db-interface/
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml --release
# Now replace shallow projects with actual source code and build again
# First, the shared lib only to add another layer of caching
RUN rm ./paddlers-shared-lib/src/*.rs
RUN rm ./paddlers-db-interface/target/release/deps/paddlers_shared*
RUN rm ./paddlers-db-interface/target/release/deps/libpaddlers_shared*
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./migrations ./migrations
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml --release
# Second, the application binary
RUN rm ./paddlers-db-interface/src/*.rs
COPY ./paddlers-db-interface/src ./paddlers-db-interface/src
RUN rm ./paddlers-db-interface/target/release/deps/paddlers_db*
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml --release

FROM buildpack-deps:stretch as DbInterface
WORKDIR /app
COPY --from=DbInterfaceBuilder ./paddlers-db-interface/target/release/paddlers-db-interface ./paddlers-db-interface
COPY ./diesel.toml ./diesel.toml
# Customize env file later if you need to 
COPY ./local.env ./.env
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
# TODO: Check if this works for demo
CMD ["./wait-for-it.sh" , "game-master:8088" , "--strict" , "--timeout=60" , "--" , "./paddlers-db-interface"]
EXPOSE 65432