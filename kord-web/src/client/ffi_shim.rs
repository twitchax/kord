use leptos::{html::Code, prelude::NodeRef};

pub async fn record_microphone(_seconds: u32) -> Result<Vec<u8>, String> {
    Err("microphone only available in browser".into())
}

pub fn highlight_code_block(_node_ref: &NodeRef<Code>) -> Result<(), String> {
    Err("highlight_code_block only available in browser".into())
}

pub struct MidiPlayer {}

impl MidiPlayer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn play_midi_note(&self, _note: &str, _velocity: f32) -> Result<(), String> {
        Err("play_midi_note only available in browser".into())
    }

    pub async fn stop_note(&self, _note: &str) -> Result<(), String> {
        Err("stop_note only available in browser".into())
    }

    pub async fn stop_all_notes(&self) -> Result<(), String> {
        Err("stop_all_notes only available in browser".into())
    }
}

impl Default for MidiPlayer {
    fn default() -> Self {
        Self::new()
    }
}
