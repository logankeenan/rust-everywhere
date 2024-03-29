use axum_browser_adapter::{axum_response_to_wasm_response, wasm_request_to_axum_request, WasmResponse};
use wasm_bindgen::prelude::wasm_bindgen;
use tower_service::Service;
use app::create_app;

pub use axum_browser_adapter::WasmRequest;

#[wasm_bindgen]
pub async fn app(wasm_request: WasmRequest) -> WasmResponse {
    let mut router = create_app();
    let request = wasm_request_to_axum_request(&wasm_request).unwrap();

    let axum_response = router.call(request).await.unwrap();
    let response = axum_response_to_wasm_response(axum_response).await.unwrap();

    response
}
