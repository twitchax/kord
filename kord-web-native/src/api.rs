use leptos::{prelude::ServerFnError, server};

/// Server function for hello greeting - called by the frontend
#[server]
pub async fn hello(name: String) -> Result<String, ServerFnError> {
    if name == "Darth" {
        return Err(ServerFnError::ServerError("I am your father.".to_string()));
    }

    Ok(format!("Hello, {name}!"))
}
