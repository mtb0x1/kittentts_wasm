use once_cell::sync::Lazy;
use ort::session::RunOptions;
use ort::value::Tensor;
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use tracing_subscriber::EnvFilter;
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::fmt::{
    format::{FmtSpan, Pretty},
    time::UtcTime,
};
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Blob;

mod session;

#[cfg(target_arch = "wasm32")]
mod voices;
#[cfg(target_arch = "wasm32")]
mod wav;

#[cfg(target_arch = "wasm32")]
use phonemica::wasm::Phonemizer;

#[cfg(not(target_arch = "wasm32"))]
use phonemica::IPAPhonemizer as Phonemizer;
use session::KittenSession;

#[cfg(target_arch = "wasm32")]
use session::preload_model;
#[cfg(target_arch = "wasm32")]
use wav::process_and_get_blob;

static GLOBAL_TRACING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

static GLOBAL_SESSION: Lazy<Arc<Mutex<Option<KittenSession>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[cfg(target_arch = "wasm32")]
static GLOBAL_PHONEMIZER: Lazy<Mutex<Option<Phonemizer>>> = Lazy::new(|| Mutex::new(None));

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let search = location.search().unwrap_or_default();
    let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
    let tracing_param = params.get("tracing");
    let tracing_level_param = params.get("level");

    if let Some(tracing_val) = tracing_param {
        let mut initialized = GLOBAL_TRACING.lock().unwrap();
        if !*initialized {
            let enabled = matches!(tracing_val.to_lowercase().as_str(), "on" | "1" | "true");

            if enabled {
                let level_str = tracing_level_param.unwrap_or_else(|| "trace".to_string());
                let filter = match level_str.to_lowercase().as_str() {
                    "debug" | "info" | "warn" | "error" | "trace" => {
                        EnvFilter::new(format!("ort={level_str},kittentts_wasm={level_str}"))
                    }
                    _ => EnvFilter::new("ort=trace,kittentts_wasm=trace"),
                };

                let fmt_layer = tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(tracing_web::MakeConsoleWriter)
                    .with_span_events(FmtSpan::ACTIVE)
                    .with_timer(UtcTime::rfc_3339())
                    .with_filter(filter);

                let perf_layer =
                    tracing_web::performance_layer().with_details_from_fields(Pretty::default());
                tracing_subscriber::registry()
                    .with(fmt_layer)
                    .with(perf_layer)
                    .try_init()
                    .unwrap();
            }
            *initialized = true;
        }
    }
    tracing::info!("KittenTTS WASM initialized");
    preload_model();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "loadModel")]
pub async fn load_model(feature: Option<String>, backend: Option<String>) -> Result<(), JsValue> {
    load_model_with_force_reload(feature, backend, false).await
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "loadModelForceReload")]
pub async fn load_model_with_force_reload(
    feature: Option<String>,
    backend: Option<String>,
    force_reload: bool,
) -> Result<(), JsValue> {
    if is_model_loaded() && !force_reload {
        tracing::info!("Model already loaded");
        return Ok(());
    }

    // If forcing reload, clear the existing session
    if force_reload {
        tracing::info!("Force reloading model, clearing existing session");
        let mut global = GLOBAL_SESSION.lock().unwrap();
        *global = None;
    }

    let feature_str = feature.as_deref().unwrap_or("cpu").to_lowercase();
    let backend_str = backend.as_deref().unwrap_or("ort-web").to_lowercase();

    tracing::info!(
        "Loading model with feature: {}, backend: {}",
        feature_str,
        backend_str
    );

    if backend_str != "ort-web" {
        tracing::warn!("Backend {} not supported yet, using ort-web", backend_str);
    }

    let (api_features, select_feature) = match feature_str.as_str() {
        "webgl" => (ort_web::FEATURE_WEBGL, "WEBGL"),
        "webgpu" => (ort_web::FEATURE_WEBGPU, "WEBGPU"),
        "webnn" => (ort_web::FEATURE_WEBNN, "WEBNN"),
        "cpu" | "none" | "ort-web" => (ort_web::FEATURE_NONE, "CPU"),
        _ => (ort_web::FEATURE_NONE, "CPU"),
    };

    tracing::info!("Initializing ORT Web API with feature {}", select_feature);
    let api = match ort_web::api(api_features).await {
        Ok(api) => api,
        Err(e) => {
            tracing::warn!(
                "Failed to initialize ORT Web API for {}: {}",
                select_feature,
                e
            );
            if api_features != ort_web::FEATURE_NONE {
                tracing::info!("Falling back to CPU feature");
                ort_web::api(ort_web::FEATURE_NONE).await.map_err(|e| {
                    JsValue::from(format!(
                        "Failed to initialize ORT Web API (CPU fallback): {}",
                        e
                    ))
                })?
            } else {
                return Err(JsValue::from(format!(
                    "Failed to initialize ORT Web API: {}",
                    e
                )));
            }
        }
    };

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
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "isModelLoaded")]
pub fn is_model_loaded() -> bool {
    let loaded = GLOBAL_SESSION.lock().map(|s| s.is_some()).unwrap_or(false);
    tracing::info!("Model loaded: {}", loaded);
    loaded
}

#[cfg(target_arch = "wasm32")]
fn get_or_init_phonemizer() -> Result<std::sync::MutexGuard<'static, Option<Phonemizer>>, JsValue> {
    tracing::debug!("Entering get_or_init_phonemizer");
    let mut guard = GLOBAL_PHONEMIZER.lock().map_err(|e| {
        tracing::error!("Failed to acquire phonemizer lock: {}", e);
        JsValue::from(format!("Failed to acquire phonemizer lock: {}", e))
    })?;
    if guard.is_none() {
        tracing::info!("Initializing global phonemizer");
        *guard = Some(Phonemizer::new().map_err(|e| {
            tracing::error!("Failed to create phonemizer: {:?}", e);
            JsValue::from(format!("Failed to create phonemizer: {:?}", e))
        })?);
        tracing::info!("Phonemizer initialized successfully");
    } else {
        tracing::debug!("Using cached phonemizer");
    }
    tracing::debug!("Exiting get_or_init_phonemizer");
    Ok(guard)
}

pub fn phonemize(text: &str, phonemizer: &Phonemizer) -> String {
    tracing::trace!("Phonemizing text: {}", text);
    let result = phonemizer.phonemize_text(text);
    tracing::trace!("Phonemization result: {}", result);
    result
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "infer")]
#[allow(clippy::await_holding_lock)]
pub async fn infer_on_cpu_with_params(
    text: &str,
    voice: &str,
    speed: f32,
) -> Result<Blob, JsValue> {
    tracing::info!(
        "Inference start: text_len={}, voice={}, speed={}",
        text.len(),
        voice,
        speed
    );

    let phonemizer_guard = get_or_init_phonemizer()?;
    let phonemizer = phonemizer_guard.as_ref().expect("phonemizer initialized");

    let tokens: Vec<i64> = phonemize(text, phonemizer).chars().map(|c| c as i64).collect();

    let tokens_len = tokens.len();
    tracing::debug!("Phonemization complete: {} tokens", tokens_len);

    let speed_array = ndarray::Array1::<f32>::from_elem((1usize,), speed);

    let input_ids_val = Tensor::from_array((vec![1, tokens_len], tokens))
        .map_err(|e| JsValue::from(format!("Failed to create input_ids: {e}")))?;

    let ref_id = text.len().min(400 - 1);

    let voice_raw = voices::VOICE_MAP
        .get(voice)
        .ok_or_else(|| JsValue::from("Voice not found"))?;
    let voice_f32: Vec<f32> = voice_raw
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let voice_array = ndarray::Array2::from_shape_vec((400, 256), voice_f32)
        .map_err(|e| JsValue::from(format!("Failed to create voice array: {e}")))?;
    let style_vec = voice_array.row(ref_id).to_owned();
    let style_input = style_vec.insert_axis(ndarray::Axis(0));
    let style_val = Tensor::from_array((vec![1, 256], style_input.into_raw_vec()))
        .map_err(|e| JsValue::from(format!("Failed to create style: {e}")))?;

    let speed_vec = speed_array.into_raw_vec();
    let speed_val = Tensor::from_array((vec![1], speed_vec))
        .map_err(|e| JsValue::from(format!("Failed to create speed: {e}")))?;

    tracing::debug!("Input tensors prepared (input_ids, style, speed)");

    let inputs = ort::inputs![
        "input_ids" => input_ids_val,
        "style" => style_val,
        "speed" => speed_val,
    ];

    let run_options: RunOptions =
        RunOptions::new().map_err(|e| JsValue::from(format!("RunOptions error: {e}")))?;

    tracing::info!("Starting ORT inference run");

    let session_ptr = GLOBAL_SESSION
        .lock()
        .map_err(|e| JsValue::from(format!("Lock error: {e}")))?
        .as_mut()
        .ok_or_else(|| JsValue::from("Model not loaded yet"))?
        as *mut KittenSession;

    let mut outputs = unsafe { &mut *session_ptr }
        .session_mut()
        .run_async(inputs, &run_options)
        .await
        .map_err(|e| JsValue::from(format!("Inference failed: {}", e)))?;

    tracing::info!("Inference complete, synchronizing outputs");

    ort_web::sync_outputs(&mut outputs)
        .await
        .map_err(|e| JsValue::from(format!("Failed to sync outputs: {e}")))?;

    let waveform_out = outputs
        .get("waveform")
        .ok_or_else(|| JsValue::from("No waveform output found"))?;

    let waveform_tensor = waveform_out
        .try_extract_tensor::<f32>()
        .map_err(|e| JsValue::from(format!("Waveform output not f32 tensor: {e}")))?;

    let (_shape, slice) = waveform_tensor;

    let len = slice.len();
    if len == 0 {
        return Err(JsValue::from("Inference failed: Output size is 0"));
    }
    tracing::debug!("Waveform extracted: {} samples", len);

    tracing::trace!(
        "Inference success! First 10 values: {:?}",
        &slice[..std::cmp::min(len, 10)]
    );

    tracing::info!("Processing audio and generating WAV blob");

    process_and_get_blob(&slice[..len], len, None)
}
