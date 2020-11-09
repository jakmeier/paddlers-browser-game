# Take the latest build from docker hub
FROM jakmeier/paddlers:frontend-builder as WasmBuilder
COPY ./paddlers-frontend/Cargo.toml ./paddlers-frontend/
COPY ./paddlers-shared-lib/Cargo.toml ./paddlers-shared-lib/
COPY ./Cargo.lock ./paddlers-frontend/
COPY ./paddlers-shared-lib/src ./paddlers-shared-lib/src
COPY ./paddlers-frontend/src ./paddlers-frontend/src
COPY ./paddlers-frontend/api ./paddlers-frontend/api
COPY ./nuts ./nuts
COPY ./div-rs ./div-rs
COPY ./paddle ./paddle
COPY ./www ./www
RUN cd paddlers-frontend; wasm-pack build --debug
RUN cd www; npm run build

# A lightweight image to host application
FROM nginx:latest as WebServer
COPY --from=WasmBuilder ./www/dist/* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/js/keycloak/player.local.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/localhost.conf /etc/nginx/conf.d/paddlers.conf
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
CMD ["./wait-for-it.sh" , "db-interface:65432" , "--strict" , "--timeout=60" , "--" , "nginx", "-g daemon off;"]