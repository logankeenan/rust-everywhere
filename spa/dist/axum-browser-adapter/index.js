function wasmResponseToJsResponse(wasmResponse) {
    const body = wasmResponse.body;
    const status = parseInt(wasmResponse.status_code);
    const jsHeaders = new Headers();
    const headers = wasmResponse.headers;
    for (let [key, value] of headers) {
        jsHeaders.append(key, value);
    }
    return new Response(body, {status: status, headers: jsHeaders});
}
async function requestToWasmRequest(request, WasmRequest) {
    const method = request.method;
    const url = request.url;
    const headers = Object.fromEntries(request.headers.entries());

    let body = null;
    if (request.body !== null) {
        body = await request.text();
    }
    return new WasmRequest(method, url, headers, body);
}