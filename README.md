# KittenTTS WASM

Experimental WebAssembly (WASM) inference app of [KittenTTS](https://github.com/KittenML/KittenTTS), a high-quality, lightweight Text-to-Speech engine. This project enables high-performance, private, on-device speech synthesis directly in the browser using ONNX Runtime Web.

## Prerequisites

Before building, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Just](https://github.com/casey/just) (optional, but recommended for shortcut commands)
- Python 3 (for serving the web interface locally)

## How to Build

Using `just` (recommended):
```bash
just build-release
```

Or using `wasm-pack` directly:
```bash
wasm-pack build --target web --release --out-dir=web/pkg
```

This will generate the WASM binary and JavaScript glue code in the `web/pkg` directory.

## How to Use

Using `just`:
```bash
just serve
```

Or using Python:
```bash
python3 -m http.server 8000
```

Then, open your browser and navigate to:
`http://localhost:8000/web/index.html`

## Advanced: Tracing/Debugging
You can enable detailed logging in the browser console by appending a `tracing` parameter to the URL:
- `?tracing=on`: Enables full tracing.
- `?tracing=trace`: Sets log level to trace (supports `debug`, `warn`, `error`, etc.).

## License

TBD (See `Cargo.toml`)

---
*Note: This is an experimental project and some changes might occur time to time.*
