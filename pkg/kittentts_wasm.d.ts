/* tslint:disable */
/* eslint-disable */

export class WasmSession {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export function infer(text: string, voice: string, speed: number): Promise<Blob>;

export function init(): void;

export function isModelLoaded(): boolean;

export function loadModel(feature?: string | null, backend?: string | null): Promise<void>;

export function loadModelForceReload(feature: string | null | undefined, backend: string | null | undefined, force_reload: boolean): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmsession_free: (a: number, b: number) => void;
    readonly infer: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly init: () => void;
    readonly isModelLoaded: () => number;
    readonly loadModel: (a: number, b: number, c: number, d: number) => number;
    readonly loadModelForceReload: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly rust_zstd_wasm_shim_calloc: (a: number, b: number) => number;
    readonly rust_zstd_wasm_shim_free: (a: number) => void;
    readonly rust_zstd_wasm_shim_malloc: (a: number) => number;
    readonly rust_zstd_wasm_shim_memcmp: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memcpy: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memmove: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_memset: (a: number, b: number, c: number) => number;
    readonly rust_zstd_wasm_shim_qsort: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_552: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_749: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_567: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_567_1: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_754: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_1654: (a: number, b: number, c: number, d: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
