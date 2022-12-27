// Helpers.

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

/// A trait for types that have a description.
pub trait HasDescription {
    /// Returns the description of the type.
    fn description(&self) -> &'static str;
}

/// A trait for types that can be parsed from a string.
pub trait Parsable {
    fn parse(symbol: &str) -> Res<Self> where Self: Sized;
}

/// A trait for types that can be "played" via the system's audio output.
pub trait Playable {
    fn play(&self, delay: f32, length: f32, fade_in: f32) -> Void;
}