!function(t){var e={};function n(o){if(e[o])return e[o].exports;var r=e[o]={i:o,l:!1,exports:{}};return t[o].call(r.exports,r,r.exports,n),r.l=!0,r.exports}n.m=t,n.c=e,n.d=function(t,e,o){n.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:o})},n.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},n.t=function(t,e){if(1&e&&(t=n(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var o=Object.create(null);if(n.r(o),Object.defineProperty(o,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var r in t)n.d(o,r,function(e){return t[e]}.bind(null,r));return o},n.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return n.d(e,"a",e),e},n.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},n.p="",n(n.s=0)}([function(t,e,n){"use strict";function o(){}n.r(e);function r(t){return t()}function u(){return Object.create(null)}function l(t){t.forEach(r)}function c(t){return"function"==typeof t}function i(t,e){return t!=t?e==e:t!==e||t&&"object"==typeof t||"function"==typeof t}function a(t){return 0===Object.keys(t).length}new Set;function s(t,e,n){t.insertBefore(e,n||null)}function f(t){t.parentNode.removeChild(t)}function d(t){return document.createTextNode(t)}new Set;let p;function h(t){p=t}const m=[],g=[],$=[],y=[],b=Promise.resolve();let _=!1;function x(){_||(_=!0,b.then(j))}function w(t){$.push(t)}let v=!1;const k=new Set;function j(){if(!v){v=!0;do{for(let t=0;t<m.length;t+=1){const e=m[t];h(e),S(e.$$)}for(h(null),m.length=0;g.length;)g.pop()();for(let t=0;t<$.length;t+=1){const e=$[t];k.has(e)||(k.add(e),e())}$.length=0}while(m.length);for(;y.length;)y.pop()();_=!1,v=!1,k.clear()}}function S(t){if(null!==t.fragment){t.update(),l(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(w)}}const O=new Set;function E(t,e){t&&t.i&&(O.delete(t),t.i(e))}"undefined"!=typeof window?window:"undefined"!=typeof globalThis?globalThis:global;new Set(["allowfullscreen","allowpaymentrequest","async","autofocus","autoplay","checked","controls","default","defer","disabled","formnovalidate","hidden","ismap","loop","multiple","muted","nomodule","novalidate","open","playsinline","readonly","required","reversed","selected"]);let M;function T(t,e){const n=t.$$;null!==n.fragment&&(l(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function P(t,e,n,i,a,s,d=[-1]){const g=p;h(t);const $=e.props||{},y=t.$$={fragment:null,ctx:null,props:s,update:o,not_equal:a,bound:u(),on_mount:[],on_destroy:[],before_update:[],after_update:[],context:new Map(g?g.$$.context:[]),callbacks:u(),dirty:d,skip_bound:!1};let b=!1;if(y.ctx=n?n(t,$,(e,n,...o)=>{const r=o.length?o[0]:n;return y.ctx&&a(y.ctx[e],y.ctx[e]=r)&&(!y.skip_bound&&y.bound[e]&&y.bound[e](r),b&&function(t,e){-1===t.$$.dirty[0]&&(m.push(t),x(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}(t,e)),n}):[],y.update(),b=!0,l(y.before_update),y.fragment=!!i&&i(y.ctx),e.target){if(e.hydrate){const t=(_=e.target,Array.from(_.childNodes));y.fragment&&y.fragment.l(t),t.forEach(f)}else y.fragment&&y.fragment.c();e.intro&&E(t.$$.fragment),function(t,e,n){const{fragment:o,on_mount:u,on_destroy:i,after_update:a}=t.$$;o&&o.m(e,n),w(()=>{const e=u.map(r).filter(c);i?i.push(...e):l(e),t.$$.on_mount=[]}),a.forEach(w)}(t,e.target,e.anchor),j()}var _;h(g)}"function"==typeof HTMLElement&&(M=class extends HTMLElement{constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){for(const t in this.$$.slotted)this.appendChild(this.$$.slotted[t])}attributeChangedCallback(t,e,n){this[t]=n}$destroy(){T(this,1),this.$destroy=o}$on(t,e){const n=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return n.push(e),()=>{const t=n.indexOf(e);-1!==t&&n.splice(t,1)}}$set(t){this.$$set&&!a(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}});function C(t){let e;return{c(){e=d("/home/jakmeier/fun/gamedev/paddlers/paddlers-frontend/src/game/town/svelte"),this.c=o},m(t,n){s(t,e,n)},p:o,i:o,o:o,d(t){t&&f(e)}}}class q extends M{constructor(t){super(),P(this,{target:this.shadowRoot},null,C,i,{}),t&&t.target&&s(t.target,this,t.anchor)}}customElements.define("test-component",q);e.default=q}]);