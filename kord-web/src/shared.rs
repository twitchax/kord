use leptos::{prelude::ServerFnError, server};

/// Merely here as an example.

#[server]
pub async fn hello(name: String) -> Result<String, ServerFnError> {
    if name == "Darth" {
        return Err(ServerFnError::ServerError("I am your father.".to_string()));
    }

    Ok(format!("Hello, {name}!"))
}