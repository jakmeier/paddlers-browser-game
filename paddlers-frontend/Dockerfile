FROM jakmeier/paddlers:frontend-builder as WasmBuilder
# frontend-builder already contains a full build of a previous version (with precompiled dependencies)
# Now update source code with newest version
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./paddlers-frontend/api ./paddlers-frontend/api
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
# For releases, these should be published to crates.io
# COPY ./div-rs ./div-rs
# COPY ./nuts ./nuts
COPY ./paddle ./paddle
COPY ./Cargo.lock ./paddlers-frontend/
# Build
RUN cd paddlers-frontend; wasm-pack build
RUN cd www; npm run release

# A lightweight image to host application
FROM nginx:alpine as WebServer

# Install SSL certificate tools
RUN apk add --update openssl netcat-openbsd bc curl wget git bash libressl socat
RUN cd /tmp/; \
git clone https://github.com/Neilpang/acme.sh.git; \
cd acme.sh/; \
./acme.sh --install; \
D=/usr/share/nginx/html; \
mkdir -vp ${D}/.well-known/acme-challenge/; \
chmod -R 0555 ${D}/.well-known/acme-challenge/; \
mkdir -p /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/; \
cd /etc/nginx/ssl/letsencrypt/demo.paddlers.ch/;

# Install Paddlers app
COPY --from=WasmBuilder ./www/dist/* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/js/keycloak/player.demo.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./specification/dialogue /usr/share/nginx/html/dialogue_scenes
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/demo.conf /etc/nginx/conf.d/paddlers_ssl.conf
COPY ./paddlers-frontend/nginx/demo_no_ssl.conf /etc/nginx/conf.d/paddlers.conf
# Link other hosted sites, currently they need to be uploaded manually, though
COPY ./paddlers-frontend/nginx/demo_no_ssl.conf /etc/nginx/conf.d/paddlers.conf
