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
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml
# Now replace shallow projects with actual source code and build again
# First, the shared lib only to add another layer of caching
RUN rm ./paddlers-shared-lib/src/*.rs
RUN rm ./paddlers-db-interface/target/debug/deps/paddlers_shared*
RUN rm ./paddlers-db-interface/target/debug/deps/libpaddlers_shared*
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./migrations ./migrations
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml
# Second, the application binary
RUN rm ./paddlers-db-interface/src/*.rs
COPY ./paddlers-db-interface/src ./paddlers-db-interface/src
RUN rm ./paddlers-db-interface/target/debug/deps/paddlers_db*
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml --features=local

FROM buildpack-deps:stretch as DbInterface
WORKDIR /app
COPY --from=DbInterfaceBuilder ./paddlers-db-interface/target/debug/paddlers-db-interface ./paddlers-db-interface
COPY ./diesel.toml ./diesel.toml
# Customize env file later if you need to 
COPY ./local.env ./.env
# Local build also needs a RSA key (must match keycloak setup)
RUN mkdir /opt/keycloak
COPY ./paddlers-keycloak/debug_pub_rsa.der /opt/keycloak/pub_rsa.der
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
CMD ["./wait-for-it.sh" , "game-master:8088" , "--strict" , "--timeout=60" , "--" , "./paddlers-db-interface"]
EXPOSE 65432