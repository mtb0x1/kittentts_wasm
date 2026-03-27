# kittentts wasm - TODO

## inference
- [x] load ONNX model in wasm
- [x] Expose text, voice and speed inputs to inference function
- [x] handle model inference from js-sys
- [x] implement actual phonemizer for tokenization 
- [x] return inference results as structured data

## Audio processing
- [x] generate wav file from inference output
- [x] stream wav back to JavaScript
- [x] handle audio buffer download/playback
- [x] move wav generation to wasm (it's currently in js)


## web interface
- [x] create single-page app (CSR)
- [x] text input form
- [x] display inference status/progress
- [x] play/download audio output
- [x] error handling and user feedback

## build & deploy
- [ ] optimize wasm bundle size
    - [x] using wasm-opt we shave only 100kb on 82Mb 
        `wasm-opt --all-features -O4 kittentts_wasm_bg.wasm -o kittentts_wasm_bg_opt.wasm`
    - [ ] zip onnx and bin files to reduce size, and unzip in memory at _start
- [x] test in browser
- [x] handle CORS if needed
- [x] document usage

## performance
- [ ] profile wasm bundle
- [ ] optimize hot paths
- [ ] explore using WebGPU for hardware acceleration
- [ ] explore using WebNN (browser API for ml inference) for better perfs
- [x] activate simd feature for wasm and more
- [ ] test on different hardware
- [x] voices names need to be aligned (todo in build.rs)
- [ ] clean cargo dep : remove useless crates.
