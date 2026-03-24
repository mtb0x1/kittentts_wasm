build: build-release

clean:
    cargo clean

build-debug: clean
    wasm-pack build --target web --dev

build-release: clean
    wasm-pack build --target web --release

# Serve repo root — open http://localhost:8000/web/index.html (needs build-release first for pkg/)
serve:
    @echo "Open http://localhost:8000/web/index.html"
    python3 -m http.server 8000

