/// Converts a frequency to a mel.
pub fn mel(f: f32) -> f32 {
    2595f32 * (1f32 + f / 700f32).log10()
}

/// Converts a mel to a frequency.
pub fn inv_mel(m: f32) -> f32 {
    700f32 * (10f32.powf(m / 2595f32) - 1f32)
}
