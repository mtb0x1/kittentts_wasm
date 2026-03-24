import wasmInit, { loadModel, isModelLoaded, infer } from "../pkg/kittentts_wasm.js";

const textInput = document.getElementById('text-input');
const voiceSelect = document.getElementById('voice-select');
const speedSlider = document.getElementById('speed-slider');
const speedValue = document.getElementById('speed-value');
const webgpuCheckbox = document.getElementById('webgpu-checkbox');
const generateBtn = document.getElementById('generate-btn');
const btnText = generateBtn.querySelector('.btn-text');
const btnLoader = generateBtn.querySelector('.loader');
const statusText = document.getElementById('status-text');
const statusIndicator = document.querySelector('.status-indicator');
const errorBanner = document.getElementById('error-banner');
const audioOutput = document.getElementById('audio-output');
const player = document.getElementById('player');

const log = (phase, detail) => {
    const msg = detail !== undefined ? `[kittentts] ${phase}: ${detail}` : `[kittentts] ${phase}`;
    console.log(msg);
};

function updateStatus(message, state = 'normal') {
    statusText.textContent = message;
    statusIndicator.className = 'status-indicator';
    if (state === 'processing') statusIndicator.classList.add('processing');
    if (state === 'success') statusIndicator.classList.add('success');
    if (state === 'error') statusIndicator.classList.add('error');
}

function showError(errMessage) {
    errorBanner.textContent = errMessage;
    errorBanner.classList.remove('hidden');
    updateStatus('Error occurred', 'error');
}

function hideError() {
    errorBanner.classList.add('hidden');
    errorBanner.textContent = '';
}

async function main() {
    const t0 = performance.now();
    log("boot", "starting wasm init");
    updateStatus("Loading WASM environment...", 'processing');

    try {
        await wasmInit();
        log("wasm", `instantiated (+${(performance.now() - t0).toFixed(0)} ms)`);

        updateStatus("Loading ONNX model...", 'processing');
        log("model", "loadModel() starting (embedded ONNX in wasm)");
        const t1 = performance.now();

        await loadModel();

        const loadTimeMs = (performance.now() - t1).toFixed(0);

        log("model", `loadModel() finished (+${loadTimeMs} ms)`);
        log("state", `isModelLoaded() = ${isModelLoaded()}`);
        log("done", `total ${(performance.now() - t0).toFixed(0)} ms`);

        // Populate voices
        try {
            const resp = await fetch('voices.json');
            if (resp.ok) {
                const voices = await resp.json();
                log("voices", `fetched ${voices.length} voices from voices.json`);
                voiceSelect.innerHTML = '';
                voices.forEach(v => {
                    const opt = document.createElement('option');
                    opt.value = v.technical;
                    opt.textContent = v.colloquial;
                    voiceSelect.appendChild(opt);
                });
            } else {
                console.warn("[kittentts] voices.json not found or failed to load");
            }
        } catch (e) {
            console.error("[kittentts] failed to fetch voices", e);
        }

        // Update speed label on input
        speedSlider.addEventListener('input', (e) => {
            speedValue.textContent = parseFloat(e.target.value).toFixed(1);
        });

        // Ready
        updateStatus(`Ready (Loaded in ${loadTimeMs}ms)`, "success");
        textInput.disabled = false;
        voiceSelect.disabled = false;
        speedSlider.disabled = false;
        webgpuCheckbox.disabled = false;
        generateBtn.disabled = false;
        btnText.textContent = "Generate Audio";

        // Generate click handler
        generateBtn.addEventListener('click', async () => {
            const text = textInput.value.trim();
            const voice = voiceSelect.value;
            const speed = parseFloat(speedSlider.value);
            const useWebGPU = webgpuCheckbox.checked;

            if (!text) {
                showError("Please enter some text to synthesize.");
                return;
            }

            hideError();
            audioOutput.classList.add('hidden');

            // Set UI to processing
            textInput.disabled = true;
            voiceSelect.disabled = true;
            speedSlider.disabled = true;
            webgpuCheckbox.disabled = true;
            generateBtn.disabled = true;
            btnText.textContent = "Synthesizing...";
            btnLoader.classList.remove('hidden');
            updateStatus("Synthesizing audio...", "processing");

            try {
                const infer_t0 = performance.now();
                log("infer", `Calling infer with text: "${text}", voice: "${voice}", speed: ${speed}`);
                const result = infer(text, voice, speed);

                const inferTimeMs = (performance.now() - infer_t0).toFixed(0);
                // If it succeeds (which it shouldn't right now due to todo!)
                console.log("Inference result:", result);
                updateStatus(`Generation complete! (Took ${inferTimeMs}ms)`, "success");

                // TODO: Wire up actual audio blob when Rust returns it
                // const blobUrl = URL.createObjectURL(blob);
                // player.src = blobUrl;
                // audioOutput.classList.remove('hidden');
            } catch (e) {
                console.error("[kittentts] inference failed", e);
                showError(`Inference failed: ${e.toString()}`);
            } finally {
                // Restore UI
                textInput.disabled = false;
                voiceSelect.disabled = false;
                speedSlider.disabled = false;
                webgpuCheckbox.disabled = false;
                generateBtn.disabled = false;
                btnText.textContent = "Generate Audio";
                btnLoader.classList.add('hidden');

                // If status wasn't set to "error" by catch, it will stay processing/success
                if (!errorBanner.classList.contains('hidden')) {
                    updateStatus("Ready (Failed last run)", "error");
                }
            }
        });

    } catch (e) {
        console.error("[kittentts] fatal initialization error", e);
        showError(`Initialization failed: ${e.toString()}`);
        updateStatus("Failed to initialize", "error");
        btnText.textContent = "Unavailable";
    }
}

main().catch((e) => {
    console.error("[kittentts] fatal", e);
});
