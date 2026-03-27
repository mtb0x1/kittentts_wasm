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


## web interface
- [x] create single-page app (CSR)
- [x] text input form
- [x] display inference status/progress
- [x] play/download audio output
- [x] error handling and user feedback

## build & deploy
- [ ] optimize wasm bundle size
- [x] test in browser
- [x] handle CORS if needed
- [x] document usage

## performance
- [ ] profile wasm bundle
- [ ] optimize hot paths
- [ ] explore using WebGPU for hardware acceleration
- [ ] explore using WebNN (browser API for ml inference) for better perfs
- [ ] activate simd feature for wasm
- [ ] test on different hardware
- [ ] test on different hardware
- [x] voices names need to be aligned (todo in build.rs)
- [ ] clean cargo dep : remove useless crates.
