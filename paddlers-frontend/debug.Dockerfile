# For local development, which doesn't build in a docker container but instead copies from the native target directory

# A lightweight image to host application
FROM nginx:latest as WebServer
COPY ./www/dist/* /usr/share/nginx/html/
COPY ./paddlers-frontend/static /usr/share/nginx/html
COPY ./paddlers-frontend/static/js/keycloak/player.local.json /usr/share/nginx/html/js/keycloak/player.json
COPY ./specification/dialogue /usr/share/nginx/html/dialogue_scenes
COPY ./paddlers-frontend/nginx/mime.types ./paddlers-frontend/nginx/nginx.conf /etc/nginx/
COPY ./paddlers-frontend/nginx/localhost.conf /etc/nginx/conf.d/paddlers.conf
COPY ./wait-for-it.sh ./wait-for-it.sh
RUN chmod +x ./wait-for-it.sh
CMD ["./wait-for-it.sh" , "db-interface:65432" , "--strict" , "--timeout=60" , "--" , "nginx", "-g daemon off;"]