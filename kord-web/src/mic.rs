#[cfg(feature = "hydrate")]
use js_sys::{Float32Array, Object, Reflect};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsValue;

// Bind to external JS module at /ffi.js (placed at crate root; ensure served as static asset).
#[cfg(feature = "hydrate")]
#[wasm_bindgen(module = "/src/ffi.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = recordMicrophone)]
    async fn js_record_microphone(seconds: u32, frame_size: u32) -> Result<JsValue, JsValue>;
}

// Records mono PCM for `seconds`, frame_size controls internal JS processing buffer.
// Returns Vec<u8> containing little-endian f32 samples concatenated.
#[cfg(feature = "hydrate")]
pub async fn record_microphone(seconds: u32, frame_size: u32) -> Result<Vec<u8>, String> {
    // Call the ffi layer.

    let js_val = js_record_microphone(seconds, frame_size).await.map_err(|e| format!("js error: {e:?}"))?;

    // Get the data from the object.

    let obj = Object::from(js_val);
    let get = |k: &str| Reflect::get(&obj, &JsValue::from_str(k));
    let data_val = get("data").map_err(|_| "missing data".to_string())?;
    let f32_array = data_val.dyn_into::<Float32Array>().map_err(|_| "data not Float32Array".to_string())?;
    let mut data = vec![0f32; f32_array.length() as usize];
    f32_array.copy_to(&mut data);

    // Convert f32 slice to bytes (little endian).

    let mut bytes = Vec::<u8>::with_capacity(data.len() * 4);
    for v in data.into_iter() {
        bytes.extend_from_slice(&v.to_le_bytes());
    }

    Ok(bytes)
}

#[cfg(not(feature = "hydrate"))]
pub async fn record_microphone(_seconds: u32, _frame_size: u32) -> Result<Vec<u8>, String> {
    Err("microphone only available in browser".into())
}
