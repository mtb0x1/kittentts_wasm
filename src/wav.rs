use wasm_bindgen::JsValue;
use web_sys::{Blob, BlobPropertyBag};

pub fn save_as_wav(data: &[f32], sample_rate: Option<u32>) -> Vec<u8> {
    tracing::debug!("Entering save_as_wav");
    let sample_rate = sample_rate.unwrap_or(24000);
    let num_samples = data.len() as u32;
    let num_channels = 1u16;
    let bits_per_sample = 32u16;

    let byte_rate = sample_rate * num_channels as u32 * (bits_per_sample as u32 / 8);
    let block_align = num_channels * (bits_per_sample / 8);
    let data_size = num_samples * (bits_per_sample as u32 / 8);

    let mut file = Vec::with_capacity(44 + data_size as usize);

    file.extend_from_slice(b"RIFF");
    file.extend_from_slice(&(36 + data_size).to_le_bytes());
    file.extend_from_slice(b"WAVE");

    file.extend_from_slice(b"fmt ");
    file.extend_from_slice(&16u32.to_le_bytes());
    file.extend_from_slice(&3u16.to_le_bytes());
    file.extend_from_slice(&num_channels.to_le_bytes());
    file.extend_from_slice(&sample_rate.to_le_bytes());
    file.extend_from_slice(&byte_rate.to_le_bytes());
    file.extend_from_slice(&block_align.to_le_bytes());
    file.extend_from_slice(&bits_per_sample.to_le_bytes());

    file.extend_from_slice(b"data");
    file.extend_from_slice(&data_size.to_le_bytes());

    let byte_data =
        unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) };
    file.extend_from_slice(byte_data);

    tracing::debug!("Exiting save_as_wav, file size: {}", file.len());
    file
}

pub fn process_and_get_blob(
    slice: &[f32],
    len: usize,
    sample_rate: Option<u32>,
) -> Result<Blob, JsValue> {
    tracing::debug!("Entering process_and_get_blob, len: {}", len);
    let wav_bin = save_as_wav(&slice[..len], sample_rate);

    let js_array_view = unsafe { js_sys::Uint8Array::view(&wav_bin) };

    let array = js_sys::Array::new();
    array.push(&js_array_view.into());

    let options = BlobPropertyBag::new();
    options.set_type("audio/wav");

    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &options)
        .map_err(|e| JsValue::from(format!("Failed to create Blob: {:?}", e)))?;
    tracing::debug!("Exiting process_and_get_blob, blob size: {}", blob.size());
    Ok(blob)
}
