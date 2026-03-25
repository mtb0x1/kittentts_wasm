use once_cell::sync::Lazy;
use ort::session::RunOptions;
use ort::session::Session;
use ort::value::Tensor;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use wasm_bindgen::prelude::*;

mod session;
mod text_cleaner;
use session::KittenSession;
use text_cleaner::TextCleaner;

static GLOBAL_TRACING: Lazy<Mutex<()>> = Lazy::new(|| {
    let stdout = tracing_subscriber::fmt::layer().with_filter(EnvFilter::new("ort=debug"));
    tracing_subscriber::registry().with(stdout).init();
    Mutex::new(())
});

static GLOBAL_SESSION: Lazy<Arc<Mutex<Option<KittenSession>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[wasm_bindgen(start)]
pub fn init() {
    // Initialize tracing subsystem once at wasm startup
    let _unused = GLOBAL_TRACING.try_lock();
    console_error_panic_hook::set_once();
    tracing::info!("KittenTTS WASM initialized");
}

#[wasm_bindgen(js_name = "loadModel")]
pub async fn load_model() -> Result<(), JsValue> {
    tracing::info!("Initializing ORT Web API");
    let api = ort_web::api(ort_web::FEATURE_NONE)
        .await
        .map_err(|e| JsValue::from(format!("Failed to initialize ORT Web API: {}", e)))?;
    ort::set_api(api);

    tracing::info!("Loading embedded KittenTTS model");

    match KittenSession::load_async().await {
        Ok(session) => {
            let mut global = GLOBAL_SESSION
                .lock()
                .map_err(|e| JsValue::from(format!("Failed to acquire session lock: {e}")))?;
            *global = Some(session);
            tracing::info!("Model loaded successfully");
            Ok(())
        }
        Err(e) => Err(session::error_to_js(e)),
    }
}

// Check if the model is currently loaded in memory.
#[wasm_bindgen(js_name = "isModelLoaded")]
pub fn is_model_loaded() -> bool {
    GLOBAL_SESSION.lock().map(|s| s.is_some()).unwrap_or(false)
}

#[wasm_bindgen(js_name = "infer")]
pub async fn infer_on_cpu_with_params(
    text: &str,
    voice_offset: usize,
    speed: f32,
) -> Result<js_sys::Float32Array, JsValue> {
    let mut global_session = GLOBAL_SESSION
        .lock()
        .map_err(|e| JsValue::from(format!("Lock error: {e}")))?;
    let mut session_wrapper = global_session
        .as_mut()
        .ok_or_else(|| JsValue::from("Model not loaded yet"))?;
    let session: &mut Session = session_wrapper.session_mut();

    let cleaner = TextCleaner::new();
    let tokens = cleaner.tokenize_for_model(text);
    let text_len = tokens.len();

    let speed_array = ndarray::Array1::<f32>::from_elem((1usize,), speed);

    let input_ids_val = Tensor::from_array((vec![1, text_len], tokens))
        .map_err(|e| JsValue::from(format!("Failed to create input_ids: {e}")))?;

    let ref_id = text.len().min(400 - 1);
    let f32_offset = voice_offset + ref_id * 256;
    let byte_offset = f32_offset * 4;

    let mut style_vec = Vec::with_capacity(256);
    let bytes_slice = session::VOICES_BIN;
    if byte_offset + 256 * 4 > bytes_slice.len() {
        return Err(JsValue::from("VOICES_BIN out of bounds"));
    }
    for i in 0..256 {
        let start = byte_offset + i * 4;
        let mut b = [0u8; 4];
        b.copy_from_slice(&bytes_slice[start..start + 4]);
        style_vec.push(f32::from_le_bytes(b));
    }

    let style_val = Tensor::from_array((vec![1, 256], style_vec))
        .map_err(|e| JsValue::from(format!("Failed to create style: {e}")))?;

    let speed_vec = speed_array.into_raw_vec();
    let speed_val = Tensor::from_array((vec![1], speed_vec))
        .map_err(|e| JsValue::from(format!("Failed to create speed: {e}")))?;

    let inputs = ort::inputs![
        "input_ids" => input_ids_val,
        "style" => style_val,
        "speed" => speed_val,
    ];

    //sync version .run_with_options(inputs, None)
    let run_options: RunOptions =
        RunOptions::new().map_err(|e| JsValue::from(format!("RunOptions error: {e}")))?;
    let outputs = session
        .run_async(inputs, &run_options)
        .await
        .map_err(|e| JsValue::from(format!("Inference failed: {e}")))?;

    let waveform_out = outputs
        .get("waveform")
        .ok_or_else(|| JsValue::from("No waveform output found"))?;

    let waveform_tensor = waveform_out
        .try_extract_tensor::<f32>()
        .map_err(|e| JsValue::from(format!("Waveform output not f32 tensor: {e}")))?;

    let (_shape, slice) = waveform_tensor;

    let len = slice.len();
    tracing::info!(
        "Inference success! Output size: {}, values: {:?}",
        len,
        &slice[..len]
    );

    // Return to JS
    let js_array = js_sys::Float32Array::new_with_length(len as u32);
    js_array.copy_from(&slice[..len]);

    Ok(js_array)
}

#[wasm_bindgen(js_name = "infer_webgpu")]
pub async fn infer_on_webgpu_with_params(
    text: &str,
    voice_offset: usize,
    speed: f32,
) -> Result<js_sys::Float32Array, JsValue> {
    todo!("WebGPU inference not implemented yet")
}
