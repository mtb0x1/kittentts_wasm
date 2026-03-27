import wasmInit, { loadModel, isModelLoaded, infer } from "./pkg/kittentts_wasm.js";
import { encodeWav } from "./wav.mjs";

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
const downloadLink = document.getElementById('download-link');

let currentBlobUrl = null;

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
        let currentVoices = [];
        try {
            const resp = await fetch('voices.json');
            if (resp.ok) {
                currentVoices = await resp.json();
                log("voices", `fetched ${currentVoices.length} voices from voices.json`);
                voiceSelect.innerHTML = '';
                currentVoices.forEach(v => {
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
            const voiceTechnical = voiceSelect.value;
            const speed = parseFloat(speedSlider.value);
            const useWebGPU = webgpuCheckbox.checked;

            if (!text) {
                showError("Please enter some text to synthesize.");
                return;
            }

            const voiceData = currentVoices.find(v => v.technical === voiceTechnical);
            if (!voiceData) {
                showError("Selected voice not found in metadata.");
                return;
            }
            const voiceOffset = voiceData.offset;

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
                log("infer", `Calling infer with text: "${text}", voice: "${voiceTechnical}" (offset: ${voiceOffset}), speed: ${speed}`);
                const result = await infer(text, voiceOffset, speed);

                const inferTimeMs = (performance.now() - infer_t0).toFixed(0);
                log("infer", `Inference returned ${result.length} samples (${inferTimeMs}ms)`);

                // Encode raw PCM f32 samples → WAV blob
                const wavBlob = encodeWav(result);
                log("wav", `encoded ${wavBlob.size} bytes`);

                // Revoke previous blob URL to free memory
                if (currentBlobUrl) URL.revokeObjectURL(currentBlobUrl);
                currentBlobUrl = URL.createObjectURL(wavBlob);

                player.src = currentBlobUrl;
                downloadLink.href = currentBlobUrl;
                downloadLink.download = `kittentts_${Date.now()}.wav`;
                audioOutput.classList.remove('hidden');
                updateStatus(`Generation complete! (${inferTimeMs}ms, ${result.length} samples)`, "success");
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
