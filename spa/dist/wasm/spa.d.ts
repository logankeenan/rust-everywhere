declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {WasmRequest} wasm_request
	* @returns {Promise<WasmResponse>}
	*/
	export function app(wasm_request: WasmRequest): Promise<WasmResponse>;
	/**
	*/
	export class WasmRequest {
	  free(): void;
	/**
	* @param {string} method
	* @param {string} url
	* @param {any} headers_js_value
	* @param {string | undefined} body
	*/
	  constructor(method: string, url: string, headers_js_value: any, body?: string);
	/**
	* @param {string} key
	* @param {string} value
	*/
	  append_header(key: string, value: string): void;
	}
	/**
	*/
	export class WasmResponse {
	  free(): void;
	/**
	*/
	  readonly body: string | undefined;
	/**
	*/
	  readonly headers: any;
	/**
	*/
	  readonly status_code: string;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly app: (a: number) => number;
  readonly __wbg_wasmrequest_free: (a: number) => void;
  readonly wasmrequest_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly wasmrequest_append_header: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_wasmresponse_free: (a: number) => void;
  readonly wasmresponse_status_code: (a: number, b: number) => void;
  readonly wasmresponse_body: (a: number, b: number) => void;
  readonly wasmresponse_headers: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0764daaaab8f6d19: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h5d128595b8812717: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
