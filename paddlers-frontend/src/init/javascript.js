export function keycloak_token() {
    return window.keycloak.token;
}
export function keycloak_preferred_name() {
    return window.keycloak.tokenParsed.preferred_username;
}