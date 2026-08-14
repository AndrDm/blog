declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen_dba88447e252d001___convert__closures_____invoke___wasm_bindgen_dba88447e252d001___JsValue__core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_dba88447e252d001___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_dba88447e252d001___convert__closures_____invoke___wasm_bindgen_dba88447e252d001___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_dba88447e252d001___convert__closures_____invoke___js_sys_c341332848a1a840___Array______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_dba88447e252d001___convert__closures_____invoke___js_sys_c341332848a1a840___Array______true__3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_dba88447e252d001___convert__closures_____invoke___core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_dba88447e252d001___JsValue___true_: (a: number, b: number) => [number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
