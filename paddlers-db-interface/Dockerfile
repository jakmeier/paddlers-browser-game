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
# Only one compilation since this is the docker file to run on releases, usually on Dockerhub
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./migrations ./migrations
COPY ./paddlers-db-interface/src ./paddlers-db-interface/src
RUN cargo build --manifest-path=paddlers-db-interface/Cargo.toml --release

FROM buildpack-deps:stretch as DbInterface
WORKDIR /app
COPY --from=DbInterfaceBuilder ./paddlers-db-interface/target/release/paddlers-db-interface ./paddlers-db-interface
COPY ./diesel.toml ./diesel.toml
# Customize env file later if you need to 
COPY ./local.env ./.env
RUN mkdir /opt/keycloak
COPY ./paddlers-keycloak/demo_pub_rsa.der /opt/keycloak/pub_rsa.der
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
CMD ["./wait-for-it.sh" , "$GAME_MASTER_SERVICE_NAME" , "--strict" , "--timeout=60" , "--" , "./paddlers-db-interface"]
EXPOSE 65432