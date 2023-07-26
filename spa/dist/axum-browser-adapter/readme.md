# axum-browser-adapter

[![Crates.io](https://img.shields.io/crates/v/axum-browser-adapter)](https://crates.io/crates/axum-browser-adapter) ![npm](https://img.shields.io/npm/v/axum-browser-adapter)

A collection of tools to make integrating Axum with the browser easier

[Documentation](https://docs.rs/axum-browser-adapter/latest/axum_browser_adapter/)

## Example

```rust
use axum_browser_adapter::{
    wasm_request_to_axum_request,
    axum_response_to_wasm_response,
    wasm_compat,
    WasmRequest,
    WasmResponse
};
use axum::Router;
use axum::routing::get;
use wasm_bindgen::prelude::wasm_bindgen;
use tower_service::Service;

#[wasm_compat]
pub async fn index() -> &'static str {
    "Hello World"
}

#[wasm_bindgen]
pub async fn wasm_app(wasm_request: WasmRequest) -> WasmResponse {
   let mut router: Router = Router::new().route("/", get(index));

   let request = wasm_request_to_axum_request(&wasm_request).unwrap();

   let axum_response = router.call(request).await.unwrap();

   let response = axum_response_to_wasm_response(axum_response).await.unwrap();

   response
}
```
Integrating w/ the browser

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title></title>
</head>
<body>
<script type="module">
    import init, {wasm_app, WasmRequest} from './dist/example.js';

    (async function () {
        await init();

        const wasmRequest = new WasmRequest("GET", "/", {}, undefined);
        let response = await wasm_app(wasmRequest);

        document.write(response.body)
    }())
</script>
</body>
</html>
```

Service worker 
```js
importScripts("/node_modules/axum-browser-adapter/index.js");

// load the WASM app 

self.addEventListener('fetch', (event) => {
    event.respondWith((async () => {
        const {wasm_app, WasmRequest} = wasm_bindgen;
        const request = event.request;
        const wasmRequest = await requestToWasmRequest(request, WasmRequest);

        const wasmResponse = await wasm_app(wasmRequest);

        return wasmResponseToJsResponse(wasmResponse);
    })());
});

```


## Running the Example

An example lives in `/example`
1. Compile the rust app to WASM: `. ./build.sh`
2. Serve `index.html` via [basic-http-server](https://github.com/brson/basic-http-server) or your favorite web server  