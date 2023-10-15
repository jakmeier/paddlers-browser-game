import Keycloak from 'keycloak-js';
import { main, start_network_thread } from "../paddlers-frontend/pkg/paddlers_frontend.js";

await init_kc();

async function init_kc() {
    window.keycloak = new Keycloak('js/keycloak/player.json');

    try {
        await window.keycloak.init({
            onLoad: 'login-required'
        });
        if (window.keycloak.tokenParsed && window.keycloak.tokenParsed.locale && window.history.replaceState) {
            const lang = window.keycloak.tokenParsed.locale;
            const url = new URL(window.location.href);
            if (!url.searchParams.has("lang")) {
                url.searchParams.set("lang", lang);
                window.history.replaceState(history.state, "Paddlers", url);
            }
        }
        main();
        start_network_thread();
    } catch (errorData) {
        console.error("Login Failed: ", errorData);
    };


    window.keycloak.onTokenExpired = () => {
        window.keycloak.updateToken(300)
            .success(() => { })
            .error(() => {
                console.error('User token refresh failed')
            });
    }
}
