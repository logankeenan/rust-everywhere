use std::collections::HashMap;
use std::str::FromStr;
use axum::body::Body;
use axum::http;
use axum::response::Response;
use axum::http::{Method, Request, Uri};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use tower_service::Service;
use app::create_app;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WasmRequest {
    #[wasm_bindgen(skip)]
    pub method: String,
    #[wasm_bindgen(skip)]
    pub url: String,
    #[wasm_bindgen(skip)]
    pub headers: HashMap<String, String>,
    #[wasm_bindgen(skip)]
    pub body: Option<String>,
}

#[wasm_bindgen]
impl WasmRequest {
    #[wasm_bindgen(constructor)]
    pub fn new(method: String, url: String, headers_js_value: JsValue, body: Option<String>) -> WasmRequest {
        let headers: HashMap<String, String> = headers_js_value.into_serde().unwrap();

        WasmRequest { method, url, headers, body }
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.url.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.clone().unwrap().to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }

    pub fn headers_append(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

pub fn wasm_request_to_axum_request(wasm_request: &WasmRequest) -> Result<Request<Body>, Box<dyn std::error::Error>> {
    let method = Method::from_str(&wasm_request.method)?;

    let uri = Uri::try_from(&wasm_request.url)?;

    let mut request_builder = Request::builder()
        .method(method)
        .uri(uri);

    for (k, v) in &wasm_request.headers {
        let header_name = http::header::HeaderName::from_bytes(k.as_bytes())?;
        let header_value = http::header::HeaderValue::from_str(v)?;
        request_builder = request_builder.header(header_name, header_value);
    }

    let request = match &wasm_request.body {
        Some(body_str) => request_builder.body(Body::from(body_str.to_owned()))?,
        None => request_builder.body(Body::empty())?,
    };

    Ok(request)
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WasmResponse {
    #[wasm_bindgen(skip)]
    pub status_code: String,
    #[wasm_bindgen(skip)]
    pub headers: HashMap<String, String>,
    #[wasm_bindgen(skip)]
    pub body: Option<String>,
}

#[wasm_bindgen]
impl WasmResponse {
    #[wasm_bindgen(getter)]
    pub fn status_code(&self) -> String {
        self.status_code.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }
}

pub async fn axum_response_to_wasm_response(mut response: Response) -> Result<WasmResponse, Box<dyn std::error::Error>> {
    // Extract and convert status code
    let status_code = response.status().to_string();

    let mut headers = HashMap::new();
    for (name, value) in response.headers() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(name.as_str().to_owned(), value_str.to_owned());
        }
    }

    let bytes = match http_body::Body::data(response.body_mut()).await {
        None => vec![],
        Some(body_bytes) => match body_bytes {
            Ok(bytes) => bytes.to_vec(),
            Err(_) => vec![]
        },
    };
    let body_str = String::from_utf8(bytes)?;

    Ok(WasmResponse {
        status_code,
        headers,
        body: Some(body_str),
    })
}

#[wasm_bindgen]
pub async fn app(wasm_request: WasmRequest) -> WasmResponse {
    let mut router = create_app();

    let request = wasm_request_to_axum_request(&wasm_request).unwrap();

    let axum_response = router.call(request).await.unwrap();

    let response = axum_response_to_wasm_response(axum_response).await.unwrap();

    response
}
