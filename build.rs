use hf_hub::api::sync::ApiBuilder;
use std::env;
use std::path::Path;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

const REPO_ID: &str = "KittenML/kitten-tts-mini-0.8";
// Files to download from the hub - just the filenames, repo ID is handled separately
const FILES: [&str; 3] = ["kitten_tts_mini_v0_8.onnx", "config.json", "voices.npz"];

fn main() {
    let stdout = tracing_subscriber::fmt::layer().with_filter(EnvFilter::new("debug"));
    tracing_subscriber::registry().with(stdout).init();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let model_dir = Path::new(&manifest_dir).join("models");
    if !model_dir.exists() {
        std::fs::create_dir_all(&model_dir).expect("Failed to create model directory");
    }
    let need_download_models = FILES
        .iter()
        .cloned()
        .filter(|&model| !model_dir.join(model).exists())
        .collect::<Vec<_>>();
    if need_download_models.is_empty() {
        tracing::info!("No need for downloading models...");
        return;
    }
    let api = ApiBuilder::new().with_progress(true).build().unwrap();
    let downloaded_path = need_download_models
        .iter()
        .map(|&model| {
            tracing::info!("Downloading model: {}", model);
            api.model(REPO_ID.to_string())
                .get(model)
                .expect("Failed to download model")
        })
        .collect::<Vec<_>>();
    for src in &downloaded_path {
        let file_name = src.file_name().unwrap();
        let link_path = model_dir.join(file_name);
        if link_path.exists() {
            tracing::warn!("Link {} already exists, skip", file_name.to_string_lossy());
            continue;
        }
        tracing::info!(
            "Creating symlink: {} → {}",
            link_path.display(),
            src.display()
        );
        #[cfg(unix)]
        std::os::unix::fs::symlink(src, &link_path)
            .unwrap_or_else(|e| panic!("failed to create unix symlink: {e}"));
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(src, &link_path)
            .unwrap_or_else(|e| panic!("failed to create windows symlink: {e}"));
    }
}
