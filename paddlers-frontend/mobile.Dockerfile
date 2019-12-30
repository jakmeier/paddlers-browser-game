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
# COPY ./quicksilver ./quicksilver
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release
# Now replace shallow projects with actual source code and build again
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/api ./paddlers-frontend/api
RUN rm ./paddlers-frontend/target/deploy/paddlers-frontend.*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/deps/paddlers_frontend*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/deps/libpaddlers_shared*
RUN rm ./paddlers-frontend/target/wasm32-unknown-unknown/release/paddlers*
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release --features=dev_view,mobile_debug

# A lightweight image to host application
FROM nginx:latest as WebServer
COPY --from=WasmBuilder ./paddlers-frontend/target/deploy/paddlers-frontend.* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/nologin.index.html /usr/share/nginx/html/index.html
COPY ./paddlers-frontend/static/js/keycloak/player.mobile.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/nologin.conf /etc/nginx/conf.d/paddlers.conf
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
CMD ["./wait-for-it.sh" , "db-interface:65432" , "--strict" , "--timeout=60" , "--" , "nginx", "-g daemon off;"]
# CMD ["./wait-for-it.sh" , "db-interface:65432" , "--strict" , "--timeout=60" , "--" , "nginx", "g"]