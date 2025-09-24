use super::setup_note_highlight;
use crate::client::ffi::MidiPlayer;
use klib::core::{
    base::{HasName, Parsable},
    note::Note,
};
use leptos::prelude::*;
use leptos_use::use_event_listener;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use thaw_utils::ArcOneCallback;
use web_sys::KeyboardEvent;

pub fn setup_keyboard_listeners(
    midi_player: Rc<MidiPlayer>,
    on_key_press: ArcOneCallback<Note>,
    pressed_notes: RwSignal<HashSet<String>>,
) {
    // Build the key -> base note map (C4..C5)
    let mut map: HashMap<String, Note> = HashMap::new();
    let add = |k: &str, n: &str, map: &mut HashMap<String, Note>| {
        if let Ok(note) = Note::parse(n) {
            map.insert(k.to_string(), note);
        }
    };
    add("a", "C4", &mut map);
    add("s", "D4", &mut map);
    add("d", "E4", &mut map);
    add("f", "F4", &mut map);
    add("g", "G4", &mut map);
    add("h", "A4", &mut map);
    add("j", "B4", &mut map);
    add("k", "C5", &mut map);
    add("w", "C#4", &mut map);
    add("e", "D#4", &mut map);
    add("t", "F#4", &mut map);
    add("y", "G#4", &mut map);
    add("u", "A#4", &mut map);

    let pressed: Rc<RefCell<HashSet<String>>> = Rc::new(RefCell::new(HashSet::new()));

    {
        let map = map.clone();
        let pressed = Rc::clone(&pressed);
        let mp = midi_player.clone();
        let on_press = on_key_press.clone();
        let cleanup = use_event_listener(leptos::prelude::window(), leptos::ev::keydown, move |ev: KeyboardEvent| {
            if ev.repeat() {
                return;
            }
            let key = ev.key().to_lowercase();
            if let Some(&note) = map.get(&key) {
                let mut set = pressed.borrow_mut();
                if !set.insert(key.clone()) {
                    return;
                }
                let note_ascii = note.name_ascii();
                setup_note_highlight(&pressed_notes, &note_ascii, true);
                let mp = mp.clone();
                crate::client::helpers::spawn_local_with_error_handling(async move {
                    mp.play_midi_note(&note_ascii, 3.0).await?;
                    Ok::<(), String>(())
                });
                on_press(note);
            }
        });
        on_cleanup(cleanup);
    }

    {
        let map = map.clone();
        let pressed = Rc::clone(&pressed);
        let mp = midi_player.clone();
        let cleanup = use_event_listener(leptos::prelude::window(), leptos::ev::keyup, move |ev: KeyboardEvent| {
            let key = ev.key().to_lowercase();
            if let Some(&note) = map.get(&key) {
                let mut set = pressed.borrow_mut();
                if set.remove(&key) {
                    let note_ascii = note.name_ascii();
                    setup_note_highlight(&pressed_notes, &note_ascii, false);
                    let mp = mp.clone();
                    crate::client::helpers::spawn_local_with_error_handling(async move {
                        mp.stop_note(&note_ascii).await?;
                        Ok::<(), String>(())
                    });
                }
            }
        });
        on_cleanup(cleanup);
    }
}
