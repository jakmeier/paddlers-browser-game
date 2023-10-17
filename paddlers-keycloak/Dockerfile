FROM quay.io/keycloak/keycloak:22.0 as KeyCloak

COPY ./paddlers-keycloak/theme /opt/keycloak/themes/paddlers
RUN /opt/keycloak/bin/kc.sh build --db=postgres
COPY ./paddlers-keycloak/realm-export.json /opt/keycloak/import/
RUN /opt/keycloak/bin/kc.sh import --file /opt/keycloak/import/realm-export.json
ENTRYPOINT ["/opt/keycloak/bin/kc.sh", "start", "--proxy", "edge", "--hostname-strict", "true", "--hostname-path", "/auth"]
