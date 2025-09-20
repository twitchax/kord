use std::{collections::HashSet, rc::Rc};
use leptos::prelude::*;
use crate::client::ffi::MidiPlayer;
use klib::core::note::Note;
use thaw_utils::ArcOneCallback;

pub fn setup_keyboard_listeners(_midi_player: Rc<MidiPlayer>, _on_key_press: ArcOneCallback<Note>, _pressed_notes: RwSignal<HashSet<String>>) {
    // no-op on SSR
}
