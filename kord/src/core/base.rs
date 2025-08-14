//! Base types and traits.

// Helpers.

#[cfg(feature = "audio")]
use rodio::{OutputStream, Sink};

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
#[cfg(feature = "audio")]
pub struct PlaybackHandle {
    _stream: OutputStream,
    _sinks: Vec<Sink>,
}

#[cfg(feature = "audio")]
impl PlaybackHandle {
    /// Creates a new [`PlayableResult`].
    pub fn new(stream_handle: OutputStream, sinks: Vec<Sink>) -> Self {
        Self { _stream: stream_handle, _sinks: sinks }
    }
}

/// A trait for types that can be "played" via the system's audio output.
/// ```rust, no_run
/// use std::time::Duration;
///
/// use klib::core::base::Playable;
/// use klib::core::{named_pitch::NamedPitch, note::Note, octave::Octave};
///
/// let handle = Note::new(NamedPitch::A, Octave::Four).play(
///     Duration::ZERO,
///     Duration::from_secs(1),
///     Duration::ZERO,
/// );
/// std::thread::sleep(Duration::from_secs(1));
/// ```
#[cfg(feature = "audio")]
pub trait Playable {
    /// Plays the [`Playable`].
    #[must_use = "Dropping the PlayableResult will stop the playback."]
    fn play(&self, delay: std::time::Duration, length: std::time::Duration, fade_in: std::time::Duration) -> Res<PlaybackHandle>;
}
