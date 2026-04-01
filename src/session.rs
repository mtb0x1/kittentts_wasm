use anyhow::{Context, Result, anyhow};
use once_cell::sync::Lazy;
use ort::session::Session;
use ort::session::builder::SessionBuilder;
use std::io::Read;
// use std::sync::Arc;

// Use compressed ONNX file (decompress at runtime from zip)
static ONNX_MODEL_COMPRESSED: &[u8] = include_bytes!("../models/kitten_tts_mini_v0_8.onnx.zip");
static ONNX_MODEL_BYTES: Lazy<Vec<u8>> = Lazy::new(|| {
    tracing::info!(
        "Decompressing ONNX model zip ({} KB compressed)",
        ONNX_MODEL_COMPRESSED.len() / 1024
    );
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(ONNX_MODEL_COMPRESSED))
        .expect("Failed to open zip archive");
    let mut model_file = archive
        .by_name("model.onnx")
        .expect("Failed to find model.onnx in zip");
    let mut decompressed = Vec::new();
    model_file
        .read_to_end(&mut decompressed)
        .expect("Failed to read model from zip");
    tracing::info!("Decompressed to {} MB", decompressed.len() / 1_000_000);
    decompressed
});

#[inline(always)]
pub fn get_model_bytes() -> &'static [u8] {
    &ONNX_MODEL_BYTES
}

#[inline(always)]
pub fn preload_model() {
    let _ = ONNX_MODEL_BYTES.len();
    tracing::info!("Model buffer preloaded");
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = "WasmSession")]
pub struct KittenSession {
    // The actual ONNX session (inference engine)
    session: Session,
}

impl KittenSession {
    #[cfg(target_arch = "wasm32")]
    pub async fn load_async() -> Result<Self> {
        tracing::debug!("Entering KittenSession::load_async");
        tracing::info!("Loading embedded KittenTTS model");

        tracing::info!(
            "Using embedded model, size: {} MB",
            get_model_bytes().len() / 1_000_000
        );

        let mut session = SessionBuilder::new()
            .map_err(|e| anyhow!("Failed to create session builder: {e}"))
            .context("while creating session builder")?;

        /* can't use those for wasm
        let mut session = session
            .with_parallel_execution(true)
            .map_err(|e| anyhow!("Failed to set parallel exec for session builder: {e}"))
            .context("while setting parallel exec")?;
        let mut session = session
            .with_intra_threads(4)
            .map_err(|e| anyhow!("Failed to set parallel exec for session builder: {e}"))
            .context("while setting with_intra_threads to 4")?;
        let mut session = session
            .with_inter_threads(4)
            .map_err(|e| anyhow!("Failed to set parallel exec for session builder: {e}"))
            .context("while setting with_inter_threads to 4")?;
        let mut session = session
            .with_optimization_level(GraphOptimizationLevel::All)
            .map_err(|e| anyhow!("Failed to set GraphOptimizationLevel for session builder: {e}"))
            .context("while GraphOptimizationLevel")?;
        */

        let session = session
            .commit_from_memory(get_model_bytes())
            .await
            .map_err(|e| anyhow!("Failed to load ONNX session: {e}"))
            .context("while committing model from bytes")?;

        tracing::info!("ONNX session created and optimized");

        tracing::debug!("Exiting KittenSession::load_async successfully");
        Ok(KittenSession { session })
    }

    pub(crate) fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    #[allow(dead_code)]
    pub(crate) fn session(&self) -> &Session {
        &self.session
    }
}

// Convert anyhow errors to JsValue for the wasm boundary.
pub fn error_to_js(err: anyhow::Error) -> wasm_bindgen::JsValue {
    let msg = format!("{:?}", err);
    tracing::error!("Rust error: {}", msg);
    js_sys::Error::new(&msg).into()
}
