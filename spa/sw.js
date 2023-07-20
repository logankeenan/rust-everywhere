importScripts("/dist/wasm/spa.js");

self.addEventListener('install', (event) => {
    event.waitUntil(loadWasmModule());
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

async function wasmResponseToJsResponse(wasmResponse) {
    const body = wasmResponse.body;
    const status = parseInt(wasmResponse.status_code);
    const jsHeaders = new Headers();
    const headers = wasmResponse.headers;
    for (let [key, value] of headers) {
        jsHeaders.append(key, value);
    }
    return new Response(body, {status: status, headers: jsHeaders});
}

async function requestToWasmRequest(request) {
    const {WasmRequest} = wasm_bindgen;
    const method = request.method;
    const url = request.url;
    const headers = Object.fromEntries(request.headers.entries());


    const cookie = await getCookie();
    if (cookie) {
        headers["Cookie"] = cookie;
    }

    let body = null;
    if (request.body !== null) {
        body = await request.text();
    }
    return new WasmRequest(method, url, headers, body);
}

self.addEventListener('fetch', async event => {
    const url = new URL(event.request.url);
    if (url.host === "localhost:4000") {
        event.respondWith((async () => {
            try {
                const {app} = wasm_bindgen;
                const request = event.request;
                const wasmRequest = await requestToWasmRequest(request);
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
