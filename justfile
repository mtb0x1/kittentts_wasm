build: build-release

clean:
    cargo clean

build-debug: clean
    wasm-pack build --target web --dev --out-dir=web/pkg

build-release:
    wasm-pack build --target web --release --out-dir=web/pkg

# Serve repo root — open http://localhost:8000/web/index.html (needs build-release first for pkg/)
serve: build-release
    @echo "Open http://localhost:8000/web/index.html"
    python3 -m http.server 8000

