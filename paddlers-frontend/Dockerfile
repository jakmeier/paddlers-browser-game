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
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
# Only one compilation since this is the docker file to run on releases, usually on Dockerhub
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/api ./paddlers-frontend/api
RUN cd paddlers-frontend; cargo web deploy --target=wasm32-unknown-unknown --release

# A lightweight image to host application
FROM nginx:alpine as WebServer

# Install SSL certificate tools
RUN apk add netcat-openbsd bc curl wget git bash libressl
RUN cd /tmp/; \
git clone https://github.com/Neilpang/acme.sh.git; \
cd acme.sh/; \
./acme.sh --install; \
D=/usr/share/nginx/html; \
mkdir -vp ${D}/.well-known/acme-challenge/; \
chmod -R 0555 ${D}/.well-known/acme-challenge/; \
mkdir -p /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/; \
cd /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/; \
openssl dhparam -dsaparam -out dhparams.pem 4096;

# Later on the machine, to actually install (for now, manually only):
# D=/usr/share/nginx/html;
# /tmp/acme.sh/acme.sh --issue -w $D -d demo.paddlers.ch -k 4096;
# /tmp/acme.sh/acme.sh  --installcert -d demo.paddlers.ch \
# --keypath /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/demo.paddlers.ch.key \
# --fullchainpath /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/demo.paddlers.ch.cer \
# --reloadcmd '/etc/init.d/nginx restart'

# Install Paddlers app
COPY --from=WasmBuilder ./paddlers-frontend/target/deploy/paddlers-frontend.* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/js/keycloak/player.demo.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/demo.conf /etc/nginx/conf.d/paddlers.conf