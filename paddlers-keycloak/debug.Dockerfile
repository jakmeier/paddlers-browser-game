FROM jboss/keycloak:7.0.0 as KeyCloak
# Import configuration
COPY ./paddlers-keycloak/realm-export.json /opt/jboss/keycloak/
COPY ./paddlers-keycloak/debug.standalone.xml /opt/jboss/keycloak/standalone/configuration/standalone.xml
# Load custom theme
COPY ./paddlers-keycloak/theme /opt/jboss/keycloak/themes/paddlers
CMD ["-b", "0.0.0.0", "-Dkeycloak.import=/opt/jboss/keycloak/realm-export.json"]
