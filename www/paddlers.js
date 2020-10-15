window.keycloak = Keycloak('js/keycloak/player.json');

window.keycloak.init({
        onLoad: 'login-required'
    })
    .success(() => {
        Rust.paddlers_frontend.then(function(wasm) {
            wasm.start_network_thread();
        });
    })
    .error(function(errorData) {
        console.error("Login Failed: ", errorData);
    });
window.keycloak.onTokenExpired = () => {
    window.keycloak.updateToken(300)
        .success(() => {})
        .error(() => {
            console.error('User token refresh failed')
        });
}