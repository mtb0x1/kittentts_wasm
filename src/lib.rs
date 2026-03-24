use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

//TODO : a session should be/ could be loaded and ready to infer model
//      maybe some kinda of cache of what's already done that can be reused
//      and some more
type KittenTtsSession = u8;

//TODO : should be
static GLOBAL_TRACING: Lazy<Mutex<()>> = Lazy::new(|| {
    let stdout = tracing_subscriber::fmt::layer().with_filter(EnvFilter::new("ort=debug"));
    tracing_subscriber::registry().with(stdout).init();
    Mutex::new(())
});

static GLOBAL_SESSION: Lazy<Mutex<Option<KittenTtsSession>>> = Lazy::new(|| Mutex::new(None));

#[wasm_bindgen(start)]
pub fn init() {
    let _unused = GLOBAL_TRACING.try_lock();
    let _unused = GLOBAL_SESSION.try_lock();
}
