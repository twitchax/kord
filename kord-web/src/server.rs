use std::io::Read;

use axum::{response::IntoResponse, routing::get, Router};
use axum_insights::AppInsights;
use futures::TryStreamExt;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceExt;
use wasi::{
    exports::http::incoming_handler::{Guest as IncomingHandlerGuest, IncomingRequest, ResponseOutparam},
    http::{
        proxy::export,
        types::{ErrorCode, Headers, OutgoingBody, OutgoingResponse},
    },
};

use crate::{api::hello, app::{shell, App}};

// Only export if compiling against WASI.
#[cfg(target_arch = "wasm32")]
export!(LeptosServer with_types_in wasi);

struct LeptosServer;

impl IncomingHandlerGuest for LeptosServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        println!("Handling request: `{:?}` => `{:?}`.", request.method(), request.path_with_query());
        
        match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(rt) => {
                rt.block_on(async move {
                    match handle(request, response_out).await {
                        Ok(_) => {}
                        Err(e) => println!("Error: {e}"),
                    }
                });
            }
            Err(e) => println!("Tokio Error: {e}"),
        }
    }
}

async fn handle(wasi_request: IncomingRequest, wasi_response_outparam: ResponseOutparam) -> Result<(), String> {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    
    let telemetry_layer = AppInsights::default()
        .with_connection_string(None)
        .with_service_config("twitchax", "kord", "server1")
        .with_live_metrics(true)
        .build_and_set_global_default()
        .unwrap()
        .layer();

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .route("/api/hello/:name", get(hello))
        .fallback(|uri: axum::http::Uri| async move {
            let path = uri.path().split_at(1).1;

            StaticAssets::get(path)
                .map(|file| (axum::http::StatusCode::OK, [(axum::http::header::CONTENT_TYPE, file.metadata.mimetype())], file.data).into_response())
                .unwrap_or_else(|| {
                    println!("No route matched, returning 404!: {uri}.");
                    (axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()).into_response()
                })
        })
        .layer(telemetry_layer)
        .with_state(leptos_options);

    let axum_request = prepare_request(wasi_request)?;
    let axum_response = app.oneshot(axum_request).await.map_err(|_| "Could not run the axum app (supposedly infallible).".to_string())?;
    set_and_stream_response(axum_response, wasi_response_outparam).await?;

    Ok(())
}

fn prepare_request(wasi_request: IncomingRequest) -> Result<axum::http::Request<axum::body::Body>, String> {
    let mut tower_request = axum::http::Request::builder()
        .uri(wasi_request.path_with_query().ok_or("The incoming request from the Wasm runtime did not have a request path.")?)
        .method(wasi_method_to_axum_method(wasi_request.method())?);

    for (name, value) in wasi_request.headers().entries() {
        tower_request = tower_request.header(name, value);
    }

    let mut body_buf = Vec::new();
    let _ = wasi_request
        .consume()
        .map_err(|_| "Could not consume the body.")?
        .stream()
        .map_err(|_| "Could not stream the body.")?
        .read_to_end(&mut body_buf)
        .map_err(|_| "Could not read the body stream.")?;
    let body = axum::body::Body::from(body_buf);

    let tower_request = tower_request.body(body).map_err(|e| format!("Could not set the body into the axum Request: {e}."))?;

    Ok(tower_request)
}

async fn set_and_stream_response(axum_response: axum::http::Response<axum::body::Body>, wasi_response_outparam: ResponseOutparam) -> Result<(), String> {
    match prepare_response(&axum_response) {
        Ok(wasi_response) => {
            let body = wasi_response.body().unwrap(); // "Only fails" if called more than once, which we are not.
            ResponseOutparam::set(wasi_response_outparam, Ok(wasi_response));

            let wasi_stream = body.write().map_err(|_| "Could not get the body stream.")?;

            let axum_body = axum_response.into_body();
            let axum_body = axum_body.into_data_stream();

            axum_body
                .try_for_each(|chonk| {
                    // We have an arbitrary number of bytes, and the WASI writer only supports writing in chunks of 4096 bytes,
                    // so we need to chunk the chunk to have more chunks for our chunks.
                    chonk.chunks(4096).for_each(|chunk| {
                        let _ = wasi_stream.blocking_write_and_flush(chunk).map_err(|_| "Could not write to the body stream.");
                    });
                    async { Ok(()) }
                })
                .await
                .map_err(|_| "Could not stream the body.")?;

            drop(wasi_stream); // Must drop in order to "fully flush" the stream.
            OutgoingBody::finish(body, None).map_err(|_| "Could not finish the body stream.")?;
        }
        Err(code) => {
            ResponseOutparam::set(wasi_response_outparam, Err(code));
        }
    };

    Ok(())
}

fn prepare_response(axum_response: &axum::http::Response<axum::body::Body>) -> Result<OutgoingResponse, ErrorCode> {
    let wasi_headers = Headers::new();
    for (name, value) in axum_response.headers().iter() {
        wasi_headers
            .append(name.as_str(), value.as_bytes())
            .map_err(|_| ErrorCode::InternalError(Some("Could not append a header.".to_owned())))?;
    }

    let wasi_response = OutgoingResponse::new(wasi_headers);

    wasi_response
        .set_status_code(axum_response.status().as_u16())
        .map_err(|_| ErrorCode::InternalError(Some("Could not set the status code.".to_owned())))?;

    Ok(wasi_response)
}

fn wasi_method_to_axum_method(method: wasi::http::types::Method) -> Result<axum::http::Method, String> {
    Ok(match method {
        wasi::http::types::Method::Get => axum::http::Method::GET,
        wasi::http::types::Method::Post => axum::http::Method::POST,
        wasi::http::types::Method::Put => axum::http::Method::PUT,
        wasi::http::types::Method::Delete => axum::http::Method::DELETE,
        wasi::http::types::Method::Head => axum::http::Method::HEAD,
        wasi::http::types::Method::Options => axum::http::Method::OPTIONS,
        wasi::http::types::Method::Connect => axum::http::Method::CONNECT,
        wasi::http::types::Method::Trace => axum::http::Method::TRACE,
        wasi::http::types::Method::Patch => axum::http::Method::PATCH,
        wasi::http::types::Method::Other(method) => axum::http::Method::from_bytes(method.as_bytes()).map_err(|e| format!("Could not parse custom method: {e}."))?,
    })
}

// Embed the static assets into the Wasm binary.

#[derive(rust_embed::Embed)]
#[folder = "../target/site"]
struct StaticAssets;
