// import * as wasm from "../paddlers-frontend/pkg/paddlers_frontend_bg.js";
import ("../paddlers-frontend/pkg/paddlers_frontend_bg.js").then(
    wasm => {

        window.keycloak = Keycloak('js/keycloak/player.json');

        window.keycloak.init({
                onLoad: 'login-required'
            })
            .success(() => {
                wasm.main();
                wasm.start_network_thread();
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
    }
)