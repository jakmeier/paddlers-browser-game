FROM jboss/keycloak:7.0.0 as KeyCloak
COPY ./paddlers-keycloak/realm-export.json /opt/jboss/keycloak/
CMD ["-b", "0.0.0.0", "-Dkeycloak.import=/opt/jboss/keycloak/realm-export.json"]
