FROM jakmeier/paddlers:frontend-builder as WasmBuilder
# frontend-builder already contains a full build of a previous version (with precompiled dependencies)
# Now update source code with newest version
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./paddlers-frontend/api ./paddlers-frontend/api
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
COPY ./Cargo.lock ./paddlers-frontend/
# Build
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

# Install Paddlers app
COPY --from=WasmBuilder ./paddlers-frontend/target/deploy/paddlers-frontend.* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/js/keycloak/player.demo.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/demo.conf /etc/nginx/conf.d/paddlers_ssl.conf
COPY ./paddlers-frontend/nginx/demo_no_ssl.conf /etc/nginx/conf.d/paddlers.conf