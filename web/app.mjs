import wasmInit, { loadModel, isModelLoaded } from "../pkg/kittentts_wasm.js";

const log = (phase, detail) => {
    const msg = detail !== undefined ? `[kittentts] ${phase}: ${detail}` : `[kittentts] ${phase}`;
    console.log(msg);
};

async function main() {
    const t0 = performance.now();
    log("boot", "starting wasm init");

    await wasmInit();
    log("wasm", `instantiated (+${(performance.now() - t0).toFixed(0)} ms)`);

    log("model", "loadModel() starting (embedded ONNX in wasm)");
    const t1 = performance.now();

    try {
        await loadModel();
    } catch (e) {
        console.error("[kittentts] loadModel failed", e);
        throw e;
    }

    log("model", `loadModel() finished (+${(performance.now() - t1).toFixed(0)} ms)`);
    log("state", `isModelLoaded() = ${isModelLoaded()}`);
    log("done", `total ${(performance.now() - t0).toFixed(0)} ms`);
}

main().catch((e) => {
    console.error("[kittentts] fatal", e);
});
