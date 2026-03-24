# kittentts wasm - TODO

## inference
- [x] load ONNX model in wasm
- [ ] Expose text input to inference function
- [ ] handle model inference from js-sys
- [ ] return inference results as structured data

## Audio processing
- [ ] generate wav file from inference output
- [ ] stream wav back to JavaScript
- [ ] handle audio buffer conversion

## web interface
- [ ] create single-page app (CSR)
- [ ] text input form
- [ ] display inference status/progress
- [ ] play/download audio output
- [ ] error handling and user feedback

## build & deploy
- [ ] optimize wasm bundle size
- [ ] test in browser
- [ ] handle CORS if needed
- [ ] document usage

## performance
- [ ] profile wasm bundle
- [ ] optimize hot paths
- [ ] explore using WebGPU for hardware acceleration
- [ ] test on different hardware
