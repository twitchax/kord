#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{http::StatusCode, Router};
    use axum_insights::AppInsights;
    use kord_web::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::{collections::HashMap, sync::OnceLock};

    // Statics.

    static FLY_REGION: OnceLock<String> = OnceLock::new();
    static FLY_ALLOC_ID: OnceLock<String> = OnceLock::new();
    static FLY_PUBLIC_IP: OnceLock<String> = OnceLock::new();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let connection_string = std::env::var("KORD_ANALYTICS_API_KEY").ok();

    let name = std::env::var("FLY_REGION").unwrap_or_else(|_| "server".to_string());
    let _ = FLY_REGION.set(name.clone());
    let _ = FLY_ALLOC_ID.set(std::env::var("FLY_ALLOC_ID").unwrap_or_else(|_| "unknown".to_string()));
    let _ = FLY_PUBLIC_IP.set(std::env::var("FLY_PUBLIC_IP").unwrap_or_else(|_| "unknown".to_string()));

    let telemetry_layer = AppInsights::default()
        .with_connection_string(connection_string.clone())
        .with_service_config("twitchax", "kord", name)
        .with_live_metrics(true)
        .with_catch_panic(true)
        .with_field_mapper(|p| {
            let fly_alloc_id = FLY_ALLOC_ID.get().unwrap().to_owned();
            let fly_public_ip = FLY_PUBLIC_IP.get().unwrap().to_owned();
            let fly_region = FLY_REGION.get().unwrap().to_owned();
            let fly_accept_region = p.headers.get("Fly-Region").map(|v| v.to_str().unwrap_or("unknown").to_owned()).unwrap_or("unknown".to_owned());

            HashMap::from([
                ("fly.alloc_id".to_string(), fly_alloc_id),
                ("fly.public_ip".to_string(), fly_public_ip),
                ("fly.server_region".to_string(), fly_region),
                ("fly.accept_region".to_string(), fly_accept_region),
            ])
        })
        .with_panic_mapper(|e| {
            (
                500,
                WebError {
                    status: 500,
                    message: format!("A panic occurred: {:?}", e),
                    backtrace: None,
                },
            )
        })
        .with_noop(connection_string.is_none())
        .with_success_filter(|status| status.is_success() || status.is_redirection() || status.is_informational() || status == StatusCode::NOT_FOUND)
        .with_error_type::<WebError>()
        .build_and_set_global_default()
        .unwrap()
        .layer();

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(telemetry_layer)
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(feature = "ssr")]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
struct WebError {
    status: u16,
    message: String,
    backtrace: Option<String>,
}

#[cfg(feature = "ssr")]
impl axum_insights::AppInsightsError for WebError {
    fn message(&self) -> Option<String> {
        Some(self.message.clone())
    }

    fn backtrace(&self) -> Option<String> {
        self.backtrace.clone()
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
