use leptos::{html::Code, prelude::NodeRef};
use js_sys::{Float32Array, Object, Reflect};
use leptos::prelude::Get;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

// Bind to external JS module at /ffi.js (placed at crate root; ensure served as static asset).
#[wasm_bindgen(module = "/src/client/ffi.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = recordMicrophone)]
    async fn js_record_microphone(seconds: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = highlightCodeBlock)]
    fn js_highlight_code_block(code_block: web_sys::HtmlElement) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_name = playMidiNote)]
    async fn js_play_midi_note(note: &str, velocity: f32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = stopMidiNote)]
    async fn js_stop_midi_note(handle: JsValue) -> Result<(), JsValue>;
}

// Records mono PCM for `seconds`, frame_size controls internal JS processing buffer.
// Returns Vec<u8> containing little-endian f32 samples concatenated.
pub async fn record_microphone(seconds: u32) -> Result<Vec<u8>, String> {
    // Call the ffi layer.
    let js_val = js_record_microphone(seconds).await.map_err(|e| format!("js error: {e:?}"))?;

    // Get the data from the object.
    let obj = Object::from(js_val);
    let get = |k: &str| Reflect::get(&obj, &JsValue::from_str(k));
    let data_val = get("data").map_err(|_| "missing data".to_string())?;
    let f32_array = data_val
        .dyn_into::<Float32Array>()
        .map_err(|_| "data not Float32Array".to_string())?;
    let mut data = vec![0f32; f32_array.length() as usize];
    f32_array.copy_to(&mut data);

    // Convert f32 slice to bytes (little endian).
    let mut bytes = Vec::<u8>::with_capacity(data.len() * 4);
    for v in data.into_iter() {
        bytes.extend_from_slice(&v.to_le_bytes());
    }

    Ok(bytes)
}

pub fn highlight_code_block(node_ref: &NodeRef<Code>) -> Result<(), String> {
    let Some(element) = node_ref.get() else {
        return Err("element not found".into());
    };

    js_highlight_code_block(element).map_err(|e| format!("js error: {e:?}"))
}

/// struct that can play piano notes using the Web Audio API.
pub struct MidiPlayer {
    /// Handles to active notes.
    handles: RefCell<HashMap<String, Vec<JsValue>>>,
}

impl MidiPlayer {
    /// Creates a new MidiPlayer.
    pub fn new() -> Self {
        Self { handles: RefCell::new(HashMap::new()) }
    }

    /// Plays a MIDI note with the given velocity.
    pub async fn play_midi_note(&self, note: &str, velocity: f32) -> Result<(), String> {
        let handle = js_play_midi_note(note, velocity)
            .await
            .map_err(|e| format!("js error: {e:?}"))?;
        self.handles
            .borrow_mut()
            .entry(note.to_string())
            .or_default()
            .push(handle);
        Ok(())
    }

    /// Stops all currently playing instances of a specific MIDI note.
    pub async fn stop_note(&self, note: &str) -> Result<(), String> {
        let handles = {
            let mut map = self.handles.borrow_mut();
            map.remove(note).unwrap_or_default()
        };
        for handle in handles {
            js_stop_midi_note(handle)
                .await
                .map_err(|e| format!("js error: {e:?}"))?;
        }
        Ok(())
    }

    /// Stops all currently playing MIDI notes.
    pub async fn stop_all_notes(&self) -> Result<(), String> {
        let all = {
            let mut map = self.handles.borrow_mut();
            std::mem::take(&mut *map)
        };
        for (_note, handles) in all {
            for handle in handles {
                js_stop_midi_note(handle)
                    .await
                    .map_err(|e| format!("js error: {e:?}"))?;
            }
        }
        Ok(())
    }
}

impl Default for MidiPlayer {
    fn default() -> Self {
        Self::new()
    }
}
