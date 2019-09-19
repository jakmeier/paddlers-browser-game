FROM jakmeier/paddlers:builder-base as WasmBuilder
RUN USER=root cargo install cargo-web
RUN \
# With selected nightly, there is a bug in cargo new, therefore cargo init is used here
mkdir paddlers-shared-lib; \
mkdir paddlers-frontend; \
USER=root cargo init --bin paddlers-frontend; \
USER=root cargo init --lib paddlers-shared-lib
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./Cargo.lock ./paddlers-frontend/
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release
# Now replace shallow projects with actual source code and build again
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/api ./paddlers-frontend/api
RUN rm ./paddlers-frontend/target/deploy/paddlers-frontend.*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/deps/paddlers_frontend*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/deps/libpaddlers_shared*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/paddlers*
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release --features=dev_view

# A lightweight image to host application
FROM nginx:alpine as WebServer
COPY --from=WasmBuilder ./paddlers-frontend/target/deploy/paddlers-frontend.* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/localhost.conf /etc/nginx/conf.d/