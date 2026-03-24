/**
 * Satisfies the synthetic `env` import emitted in `kittentts_wasm_bg.wasm` when
 * ort-sys leaves a placeholder import. If this stub is ever called at runtime,
 * ONNX Runtime was not linked correctly for this target.
 */
function ort_sys_placeholder_import() {
    console.warn(
        "[kittentts_wasm] ort-sys placeholder import invoked — check ORT wasm linking / features"
    );
}

// Named export must match the wasm import name exactly (see WebAssembly.Module.imports).
export {
    ort_sys_placeholder_import as "\n\nThe ort-sys crate could not link to ONNX Runtime because:\n\t- `libonnxruntime` is not configured via `pkg-config`\n\t- ort-sys was instructed not to download prebuilt binaries (`cargo build --offline`), or the `download-binaries` feature is not enabled\n\t- Neither `ORT_LIB_PATH` or `ORT_IOS_XCFWK_PATH` (for iOS) were set to link to custom binaries\n\nTo rectify this:\n\t- Compile ONNX Runtime from source and manually configure linking (see https://ort.pyke.io/setup/linking for more information)\n\t- Enable the `download-binaries` feature if the target is supported\n\t- Enable ort's `alternative-backend` feature if you intend to use a different backend (or ort-sys' `disable-linking` feature if you use this crate directly)\n",
};
