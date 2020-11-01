export class JsBrowserInfo {
    constructor() {
        this.user_agent = navigator.userAgent;
        this.inner_width = window.inner_width();
        this.inner_height = window.inner_height();
        this.outer_width = window.outer_width();
        this.outer_height = window.outer_height();
    }
}