importScripts("/dist/wasm/spa.js", "/dist/axum-browser-adapter/index.js");

const version = 1;

self.addEventListener('install', async (event) => {
    event.waitUntil(loadWasmModule());
    await self.skipWaiting();
});

self.addEventListener('activate', event => {
    event.waitUntil(self.clients.claim());
});

function setCookie(value) {
    return caches.open('my-cache').then((cache) => cache.put('cookie', new Response(value)));
}

function getCookie() {
    return caches.open('my-cache').then((cache) => {
        return cache.match('cookie').then((response) => {
            return response ? response.text() : null;
        });
    });
}


async function loadWasmModule() {
    return wasm_bindgen("/dist/wasm/spa_bg.wasm");
}

self.addEventListener('fetch', async event => {
    const url = new URL(event.request.url);
    if (["rust-everywhere-spa.pages.dev", "localhost:4000", "localhost:3002", "rust-everywhere-spa-server.logankeenan.com"].includes(url.host)) {
        event.respondWith((async () => {
            try {
                const {app, WasmRequest} = wasm_bindgen;
                const request = event.request;
                const wasmRequest = await requestToWasmRequest(request, WasmRequest);

                const cookie = await getCookie();
                if (cookie) {
                    wasmRequest.append_header("Cookie", cookie);
                }

                const wasmResponse = await app(wasmRequest);

                // The response has a set-cookie header. However, you can't construct
                // a Response clients-side with set-cookie and expect the cookie to be
                // set in the browser.  We'll pull off the value and set the Cookie
                // on subsequent requests
                const cookieValue = wasmResponse.headers.get('set-cookie');
                if (cookieValue) {
                    await setCookie(cookieValue)
                }

                return wasmResponseToJsResponse(wasmResponse);

            } catch (error) {
                console.error("error querying wasm app for result", {error, event});
            }
        })());
    }
});
