use leptos::{logging::log, prelude::*, task::spawn_local};
use thaw::{Flex, FlexGap};

use super::shared::{PageTitle, PrimaryButton, SecondaryButton};
use crate::api::hello;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button.
    let count = RwSignal::new(0);

    let on_click_me = move |_| *count.write() += 1;
    let on_click_hello = move |_| {
        let name = "Leptos".to_string();

        spawn_local(async move {
            match hello(name).await {
                Ok(hello_result) => log!("{hello_result}"),
                Err(e) => log!("{e}"),
            }
        });
    };

    view! {
        <PageTitle>"Welcome to Kord!"</PageTitle>
        <Flex gap=FlexGap::Large>
            <PrimaryButton id="click-me" on_click=on_click_me>
                "Click Me: " {count}
            </PrimaryButton>
            <SecondaryButton on_click=on_click_hello>
                "Click Me for a server call"
            </SecondaryButton>
        </Flex>
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_page_renders() {
        let _ = HomePage();
    }
}
