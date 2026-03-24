use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use wasm_bindgen::prelude::*;

mod session;
use session::KittenSession;

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
