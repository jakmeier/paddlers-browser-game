// import * as wasm from "../paddlers-frontend/pkg/paddlers_frontend_bg.js";
import("../paddlers-frontend/pkg/paddlers_frontend_bg.js").then(
    wasm => {

        window.keycloak = Keycloak('js/keycloak/player.json');

        window.keycloak.init({
            onLoad: 'login-required'
        })
            .success(() => {
                if (window.keycloak.tokenParsed && window.keycloak.tokenParsed.locale && window.history.replaceState) {
                    const lang = window.keycloak.tokenParsed.locale;
                    const url = new URL(window.location.href);
                    if (!url.searchParams.has("lang")) {
                        url.searchParams.set("lang", lang);
                        window.history.replaceState(history.state, "Paddlers", url);
                    }
                }
                wasm.main();
                wasm.start_network_thread();
            })
            .error(function (errorData) {
                console.error("Login Failed: ", errorData);
            });
        window.keycloak.onTokenExpired = () => {
            window.keycloak.updateToken(300)
                .success(() => { })
                .error(() => {
                    console.error('User token refresh failed')
                });
        }
    }
)