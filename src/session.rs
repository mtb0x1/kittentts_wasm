use anyhow::{Context, Result, anyhow};
use ort::session::builder::SessionBuilder;
use ort::session::{self, Session};
// use std::sync::Arc;

pub const ONNX_MODEL_BYTES: &[u8] = include_bytes!("../models/kitten_tts_mini_v0_8.onnx");

#[wasm_bindgen::prelude::wasm_bindgen(js_name = "WasmSession")]
pub struct KittenSession {
    // The actual ONNX session (inference engine)
    session: Session,
}

impl KittenSession {
    pub async fn load_async() -> Result<Self> {
        tracing::debug!("Entering KittenSession::load_async");
        tracing::info!("Loading embedded KittenTTS model");

        tracing::info!(
            "Using embedded model, size: {} MB",
            ONNX_MODEL_BYTES.len() / 1_000_000
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
            .commit_from_memory(ONNX_MODEL_BYTES)
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
