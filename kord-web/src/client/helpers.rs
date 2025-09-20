use std::{fmt::Display, future::Future};

use leptos::{logging::error, reactive::spawn_local};

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

pub fn spawn_local_with_error_handling<F, E>(future: F)
where
    F: Future<Output = Result<(), E>> + 'static,
    E: Display,
{
    spawn_local(async move {
        if let Err(e) = future.await {
            error!("Error in local task: {e}");
        }
    });
}
