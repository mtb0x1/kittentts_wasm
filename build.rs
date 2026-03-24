use hf_hub::api::sync::ApiBuilder;
use std::env;
use std::path::Path;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

const REPO_ID: &str = "KittenML/kitten-tts-mini-0.8";
// Files to download from the hub - just the filenames, repo ID is handled separately
const FILES: [&str; 3] = ["kitten_tts_mini_v0_8.onnx", "config.json", "voices.npz"];

//TODO: this will do for now
//      but in https://github.com/KittenML/KittenTTS
//      voice have English names (i.e: 'Bella', 'Jasper' ... etc)
//      we need to figure out how to get/map those vs voices.npz/json
fn format_colloquial_name(technical: &str) -> String {
    let parts: Vec<&str> = technical.split('-').collect();
    if parts.len() >= 3 {
        let id = match parts[3] {
            "m" => "Male",
            "f" => "Female",
            _ => parts[3],
        };
        format!("Voice {} {}", parts[2], id)
    } else {
        technical.to_string()
    }
}

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
    } else {
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

    // Extract voices.npz to web/voices.json
    let voices_npz_path = model_dir.join("voices.npz");
    if voices_npz_path.exists() {
        tracing::info!("Extracting voices to web/voices.json");
        let file = std::fs::File::open(&voices_npz_path).expect("Failed to open voices.npz");
        let mut archive = zip::ZipArchive::new(file).expect("Failed to read zip archive");
        let mut voices = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let name = file.name();
            if name.ends_with(".npy") {
                let technical = name.trim_end_matches(".npy").to_string();
                let colloquial = format_colloquial_name(&technical);
                voices.push(format!(
                    r#"{{"technical": "{}", "colloquial": "{}"}}"#,
                    technical, colloquial
                ));
            }
        }

        let json = format!("[\n  {}\n]", voices.join(",\n  "));
        let web_dir = Path::new(&manifest_dir).join("web");
        if !web_dir.exists() {
            std::fs::create_dir_all(&web_dir).expect("Failed to create web directory");
        }
        std::fs::write(web_dir.join("voices.json"), json).expect("Failed to write voices.json");
    }
}
