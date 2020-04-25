FROM jboss/keycloak:7.0.0 as KeyCloak
# Import configuration
COPY ./paddlers-keycloak/mobile.realm-export.json /opt/jboss/keycloak/realm-export.json
COPY ./paddlers-keycloak/mobile-standalone.xml /opt/jboss/keycloak/standalone/configuration/standalone.xml
# Load custom theme
COPY ./paddlers-keycloak/theme /opt/jboss/keycloak/themes/paddlers
COPY ./paddlers-keycloak/theme/login/mobile_login.ftl /opt/jboss/keycloak/themes/paddlers/login/login.ftl
CMD ["-b", "0.0.0.0", "-Dkeycloak.import=/opt/jboss/keycloak/realm-export.json"]
