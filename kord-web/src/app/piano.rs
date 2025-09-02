use std::sync::LazyLock;

use crate::client::audio::play_note;
use klib::core::{
    base::{HasName, Parsable},
    note::Note,
};
use leptos::prelude::*;
use thaw_utils::ArcOneCallback;

// Public Piano component ------------------------------------------------------

#[component]
pub fn Piano(#[prop(optional, into)] on_key_press: Option<ArcOneCallback<Note>>) -> impl IntoView {
    // Get the notes of the piano statically.

    let notes = &*PIANO_NOTES;

    // Build white and black key positions.

    let mut whites: Vec<(usize, Note)> = Vec::with_capacity(52);
    let mut blacks: Vec<(f32, Note)> = Vec::with_capacity(36);

    let mut white_idx: usize = 0;
    for n in notes {
        let name = n.name();
        if name.contains('#') || name.contains('♯') {
            // Position halfway between the current and previous white key
            // left = (white_idx as f32 - 0.5) * (100% / 52) computed later as percent
            let left_percent = (white_idx as f32 - 0.5) * (100.0 / 52.0);
            blacks.push((left_percent, *n));
        } else {
            whites.push((white_idx + 1, *n)); // grid-column is 1-based
            white_idx += 1;
        }
    }

    // Share the optional callback across many keys safely.
    let shared_on_press: ArcOneCallback<Note> = on_key_press.unwrap_or_else(|| ArcOneCallback::new(|_| {}));

    let white_keys = whites
        .into_iter()
        .map({
            let shared_on_press = shared_on_press.clone();
            move |(col, note)| {
                let on_press = shared_on_press.clone();
                view! { <WhiteKey note index=col on_key_press=on_press/> }
            }
        })
        .collect_view();

    let black_keys = blacks
        .into_iter()
        .map({
            let shared_on_press = shared_on_press.clone();
            move |(left, note)| {
                let on_press = shared_on_press.clone();
                view! { <BlackKey note left_percent=left on_press=on_press/> }
            }
        })
        .collect_view();

    view! {
        <div class="kord-piano">
            <div class="kord-piano__case">
                <div class="kord-piano__top">
                    <div class="kord-piano__badge">kord</div>
                    <div class="kord-piano__screw kord-piano__screw--tl"></div>
                </div>
                <div class="kord-piano__keys">
                    <div class="kord-piano__whites">{white_keys}</div>
                    <div class="kord-piano__blacks">{black_keys}</div>
                </div>
            </div>
        </div>
    }
}

// Key components --------------------------------------------------------------

#[component]
pub fn WhiteKey(note: Note, index: usize, #[prop(into)] on_key_press: ArcOneCallback<Note>) -> impl IntoView {
    // grid-column is 1-based and spans 1 col
    let style = format!("grid-column: {index} / span 1");
    view! { <Key note class="kord-piano__key--white" on_key_press=on_key_press attr:style=style /> }
}

#[component]
pub fn BlackKey(note: Note, left_percent: f32, #[prop(into)] on_press: ArcOneCallback<Note>) -> impl IntoView {
    // place relative to the white grid using left percentage
    let style = format!("left: {left_percent:.6}%");
    view! { <Key note class="kord-piano__key--black" on_key_press=on_press attr:style=style /> }
}

#[component]
pub fn Key(note: Note, #[prop(optional, into)] class: Option<String>, #[prop(into)] on_key_press: ArcOneCallback<Note>) -> impl IntoView {
    let base = "kord-piano__key";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());

    let title_note = note.name();

    view! {
        <div
            class=cls
            title=title_note
            on:click=move |_| {
                play_note(&note, 0.6);
                on_key_press(note);
            }
        ></div>
    }
}

// Static Helpers --------------------------------------------------------------------

static NOTE_NAMES: [&str; 88] = [
    "A0", "A#0", "B0", "C1", "C#1", "D1", "D#1", "E1", "F1", "F#1", "G1", "G#1", "A1", "A#1", "B1", "C2", "C#2", "D2", "D#2", "E2", "F2", "F#2", "G2", "G#2", "A2", "A#2", "B2", "C3", "C#3", "D3",
    "D#3", "E3", "F3", "F#3", "G3", "G#3", "A3", "A#3", "B3", "C4", "C#4", "D4", "D#4", "E4", "F4", "F#4", "G4", "G#4", "A4", "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5",
    "A5", "A#5", "B5", "C6", "C#6", "D6", "D#6", "E6", "F6", "F#6", "G6", "G#6", "A6", "A#6", "B6", "C7", "C#7", "D7", "D#7", "E7", "F7", "F#7", "G7", "G#7", "A7", "A#7", "B7", "C8",
];
static PIANO_NOTES: LazyLock<Vec<Note>> = LazyLock::new(|| NOTE_NAMES.iter().filter_map(|s| Note::parse(s).ok()).collect());
