build: build-release

lint:
    cargo fmt
    cargo clippy --target wasm32-unknown-unknown -- -D warnings

test: 
    cargo test --target x86_64-unknown-linux-gnu -- --nocapture
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

