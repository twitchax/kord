use std::{time::Duration};

use clap::Parser;
use klib::{base::Void, note::*, chord::{Chordable, HasChord}, pitch::HasFrequency};
use rodio::{OutputStream, Sink, source::SineWave, Source};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() -> Void {
    //let args = Args::parse();

    let chord = C.into_chord().seven().sharp9();
    let delay = 0.2f32;
    let duration = 3.0f32;

    println!("{}", chord);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut sinks = vec![];

    for (k, n) in chord.chord().into_iter().enumerate() {
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(n.frequency())
            .take_duration(Duration::from_secs_f32(duration - k as f32 * delay))
            .delay(Duration::from_secs_f32(k as f32 * delay))
            .amplify(0.20);

        sink.append(source);

        sinks.push(sink);
    }

    std::thread::sleep(Duration::from_secs_f32(5.0));

    Ok(())
}