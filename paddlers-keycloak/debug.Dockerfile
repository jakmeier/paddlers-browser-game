FROM quay.io/keycloak/keycloak:22.0 as KeyCloak
# Load custom theme
COPY ./paddlers-keycloak/theme /opt/keycloak/themes/paddlers

# Copy realm info to specific folder for import configuration
COPY ./paddlers-keycloak/realm-export.json /opt/keycloak/data/import/
RUN /opt/keycloak/bin/kc.sh build --db=postgres

ENTRYPOINT [ "/opt/keycloak/bin/kc.sh", "start-dev", "--import-realm", "--proxy", "edge", "--hostname-strict", "false", "--hostname-path", "/auth" ]
