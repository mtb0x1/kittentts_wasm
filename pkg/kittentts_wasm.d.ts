/* tslint:disable */
/* eslint-disable */

/**
 * WASM wrapper for the IPAPhonemizer
 */
export class Phonemizer {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Gets the last error message
     */
    get_error(): string;
    /**
     * Checks if the phonemizer is properly loaded
     */
    is_loaded(): boolean;
    /**
     * Creates a new phonemizer instance
     */
    constructor();
    /**
     * Creates a new phonemizer instance
     *
     * # Arguments
     * * `rules_path` - Path or identifier for rules data
     * * `list_path` - Path or identifier for list data
     * * `dialect` - dialect
     */
    static new_with_custom(rules_path: string, list_path: string, dialect: string): Phonemizer;
    /**
     * Phonemizes the given text
     *
     * # Arguments
     * * `text` - The text to phonemize
     *
     * # Returns
     * The phonemized text
     */
    phonemize_text(text: string): string;
}

export class WasmSession {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

/**
 * Gets the size of embedded list in bytes
 */
export function get_list_size(): number;

/**
 * Gets the size of embedded rules in bytes
 */
export function get_rules_size(): number;

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
    readonly __wbg_phonemizer_free: (a: number, b: number) => void;
    readonly get_list_size: () => number;
    readonly get_rules_size: () => number;
    readonly phonemizer_get_error: (a: number, b: number) => void;
    readonly phonemizer_is_loaded: (a: number) => number;
    readonly phonemizer_new: (a: number) => void;
    readonly phonemizer_new_with_custom: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly phonemizer_phonemize_text: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_518: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_715: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_533: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_533_1: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_720: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_1703: (a: number, b: number, c: number, d: number) => void;
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
