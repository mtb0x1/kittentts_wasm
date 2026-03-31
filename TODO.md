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
- [ ] sound low quality : review style process ? samples?
    - [x] kinda better by avoiding runtime sheninags on voice offset
    - [x] compare to kitten tts python outputed wav file for quality (hint: some gaps still)


## web interface
- [x] create single-page app (CSR)
- [x] text input form
- [x] display inference status/progress
- [x] play/download audio output
- [x] error handling and user feedback
- [ ] rework options selection on ui
    - [x] correct backends names and prefill features
    - [ ] TBC

## build & deploy
- [ ] optimize wasm bundle size
    - [x] using wasm-opt we shave only 100kb on 82Mb 
        `wasm-opt --all-features -O4 kittentts_wasm_bg.wasm -o kittentts_wasm_bg_opt.wasm`
    - [ ] zip onnx and bin files to reduce size, and unzip in memory at _start
- [x] test in browser
- [x] handle CORS if needed
- [x] document usage

## performance
- [ ] inference time is too long :
    - [x] ort-web 24s : 99% of time is allocated to : `session.run_async(inputs, &run_options).await`
    - [ ] explore alternative backends (https://ort.pyke.io/backends)
        - [X] tried ort-tract and ort-candle, can't even compile (getrandom dep with 2 version so can't select at runtime so need to be a feature + can't compile due to some trait errors)
        - [ ] plug wrapper on our own for tract, burn, candle ...
    - [ ] gets worst with time (mem leak ?)
    - [ ] gets worst(10x factor) with input size (10 chars vs 100 chars)
- [x] Tokenizer fails to process words like (it's, end of sentence., end-of-things ... etc)
- [ ] optimize hot paths
- [ ] explore using WebGPU for hardware acceleration
- [ ] explore using WebNN (browser API for ml inference) for better perfs
- [x] activate simd feature for wasm and more
- [ ] test on different hardware
- [x] voices names need to be aligned (todo in build.rs)
- [ ] clean cargo dep : remove useless crates.
- [ ] ci wasm-pack takes forever to install
