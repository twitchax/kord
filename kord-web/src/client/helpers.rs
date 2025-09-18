pub trait MapErrorToString<T> {
    fn map_err_to_string(self) -> Result<T, String>;
}

impl<T, E> MapErrorToString<T> for Result<T, E>
where
    E: ToString,
{
    fn map_err_to_string(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
