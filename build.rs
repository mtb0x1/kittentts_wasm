use hf_hub::api::sync::ApiBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::Path;

#[derive(Deserialize)]
struct Metadata {
    voice_aliases: HashMap<String, String>,
}

const REPO_ID: &str = "KittenML/kitten-tts-mini-0.8";

// Files to download from the hub - just the filenames, repo ID is handled separately
const FILES: [&str; 3] = ["kitten_tts_mini_v0_8.onnx", "config.json", "voices.npz"];
const DICT: &str =
    "https://raw.githubusercontent.com/Alexir/CMUdict/refs/heads/master/cmudict-0.7b";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let model_dir = Path::new(&manifest_dir).join("models");
    let web_dir = Path::new(&manifest_dir).join("web");
    if !model_dir.exists() {
        std::fs::create_dir_all(&model_dir).expect("Failed to create model directory");
    }
    let need_download_models = FILES
        .iter()
        .cloned()
        .filter(|&model| !model_dir.join(model).exists())
        .collect::<Vec<_>>();
    if need_download_models.is_empty() {
        println!("No need for downloading models...");
    } else {
        let api = ApiBuilder::new().with_progress(true).build().unwrap();
        let downloaded_path = need_download_models
            .iter()
            .map(|&model| {
                println!("Downloading model: {}", model);
                api.model(REPO_ID.to_string())
                    .get(model)
                    .expect("Failed to download model")
            })
            .collect::<Vec<_>>();
        for src in &downloaded_path {
            let file_name = src.file_name().unwrap();
            let link_path = model_dir.join(file_name);
            if link_path.exists() {
                println!("Link {} already exists, skip", file_name.to_string_lossy());
                continue;
            }
            println!(
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

    // Extract voices.npz to web/voices.json and models/voices.bin
    let voices_npz_path = model_dir.join("voices.npz");
    let config_path = model_dir.join("config.json");

    if voices_npz_path.exists() && config_path.exists() {
        println!("Extracting voices to web/voices.json and models/voices.bin");

        let config_content =
            std::fs::read_to_string(&config_path).expect("Failed to read config.json");
        let metadata: Metadata =
            serde_json::from_str(&config_content).expect("Failed to parse config.json");

        // Reverse mapping: technical -> colloquial alias
        let technical_to_alias: HashMap<String, String> = metadata
            .voice_aliases
            .into_iter()
            .map(|(alias, technical)| (technical, alias))
            .collect();

        let file = std::fs::File::open(&voices_npz_path).expect("Failed to open voices.npz");
        let mut archive = zip::ZipArchive::new(file).expect("Failed to read zip archive");
        let mut voices = Vec::new();

        let mut voices_data = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name().to_string();
            if name.ends_with(".npy") {
                let technical = name.trim_end_matches(".npy").to_string();
                let display_name = technical_to_alias.get(&technical).unwrap_or(&technical);
                let colloquial = display_name.to_string();

                // Read the .npy file content
                let mut content: Vec<u8> = Vec::new();
                std::io::Read::read_to_end(&mut file, &mut content).unwrap();

                // numpy header: \x93NUMPY \x01 \x00 <header_len u16 LE>
                let header_len = u16::from_le_bytes(content[8..10].try_into().unwrap()) as usize;
                let data_start = 10 + header_len;
                let raw_data = &content[data_start..];

                let voice_bin_path = model_dir.join(format!("{}.bin", technical));
                std::fs::write(&voice_bin_path, raw_data).expect("Failed to write voice bin");

                let current_offset = voices_data.len() * 102400; // offset in f32 elements, 400*256=102400 per voice
                voices_data.push((technical.clone(), raw_data.to_vec()));

                voices.push(format!(
                    r#"{{"technical": "{}", "colloquial": "{}", "offset": {}}}"#,
                    technical, colloquial, current_offset
                ));
            }
        }

        // Generate voices.rs
        let mut voices_rs = String::new();
        voices_rs.push_str("use std::collections::HashMap;\nuse once_cell::sync::Lazy;\n\n");

        for (technical, _) in &voices_data {
            let const_name = technical.replace("-", "_").to_uppercase();
            voices_rs.push_str(&format!(
                "const VOICE_{}: &[u8] = include_bytes!(\"../models/{}.bin\");\n",
                const_name, technical
            ));
        }

        voices_rs.push_str("\npub static VOICE_MAP: Lazy<HashMap<&str, &[u8]>> = Lazy::new(|| {\n    let mut map = HashMap::new();\n");
        for (technical, _) in &voices_data {
            let const_name = technical.replace("-", "_").to_uppercase();
            voices_rs.push_str(&format!(
                "    map.insert(\"{}\", VOICE_{});\n",
                technical, const_name
            ));
        }
        voices_rs.push_str("    map\n});\n");

        std::fs::write(
            Path::new(&manifest_dir).join("src").join("voices.rs"),
            voices_rs,
        )
        .expect("Failed to write voices.rs");

        let json = format!("[\n  {}\n]", voices.join(",\n  "));
        std::fs::write(web_dir.join("voices.json"), json).expect("Failed to write voices.json");
        //use reqwest to download DICT and save it under web/cmu.dict
        let dict_path = web_dir.join("cmu.dict");
        if !dict_path.exists() {
            let mut response = reqwest::blocking::get(DICT).expect("Failed to download dict");
            let mut file = std::fs::File::create(&dict_path).expect("Failed to create dict file");
            std::io::copy(&mut response, &mut file).expect("Failed to write dict file");
        }
    }
}
