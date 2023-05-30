//! Base types and traits.

// Helpers.

use rodio::{OutputStream, OutputStreamHandle, Sink};

/// Global result type.
pub type Res<T> = anyhow::Result<T>;

/// Global error type.
pub type Err = anyhow::Error;

/// Global void type.

pub type Void = Res<()>;
// Traits.

/// A trait for types that have a static name.
pub trait HasStaticName {
    /// Returns the static name of the type.
    fn static_name(&self) -> &'static str;
}

/// A trait for types that have a computed name.
pub trait HasName {
    /// Returns the computed name of the type.
    fn name(&self) -> String;
}

/// A trait for types that have a computed name.
pub trait HasPreciseName {
    /// Returns the computed name of the type.
    fn precise_name(&self) -> String;
}

/// A trait for types that have a description.
pub trait HasDescription {
    /// Returns the description of the type.
    fn description(&self) -> &'static str;
}

/// A trait for types that can be parsed from a string.
pub trait Parsable {
    /// Parses the type from a string.
    fn parse(symbol: &str) -> Res<Self>
    where
        Self: Sized;
}

/// A struct for holding the types for a [`Playable`].
pub struct PlaybackHandle {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    _sinks: Vec<Sink>,
}

impl PlaybackHandle {
    /// Creates a new [`PlayableResult`].
    pub fn new(stream: OutputStream, stream_handle: OutputStreamHandle, sinks: Vec<Sink>) -> Self {
        Self {
            _stream: stream,
            _stream_handle: stream_handle,
            _sinks: sinks,
        }
    }
}

/// A trait for types that can be "played" via the system's audio output.
pub trait Playable {
    /// Plays the [`Playable`].
    #[must_use = "Dropping the PlayableResult will stop the playback."]
    fn play(&self, delay: f32, length: f32, fade_in: f32) -> Res<PlaybackHandle>;
}
