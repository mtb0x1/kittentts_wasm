build: build-release

clean:
    cargo clean

build-debug: clean
    wasm-pack build --target web --dev

build-release: clean
    wasm-pack build --target web --release

