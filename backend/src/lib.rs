use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    http::{Request, Response},
    routing::get,
};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{
    layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

use crate::{
    endpoints::{echo::get_root_endpoint, items::get_items_router},
    methods::memory_queries::MemoryQueries,
    state::AppState,
};

pub mod endpoints;
pub mod methods;
pub mod models;
pub mod state;

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<_>| tracing::info_span!(""))
        .on_request(|request: &Request<_>, _span: &Span| {
            tracing::info!("{} {}", request.method(), request.uri().path())
        })
        .on_response(
            |response: &Response<_>, latency: Duration, _span: &Span| {
                tracing::info!(
                    "{} after {}ms",
                    response.status(),
                    latency.as_millis()
                )
            },
        );

    let queries = MemoryQueries::default();
    let state = AppState {
        queries: Arc::new(Mutex::new(queries)),
    };
    let app = Router::new()
        .route("/", get(get_root_endpoint))
        .nest("/items", get_items_router())
        .with_state(state)
        .layer(trace_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("server running");
    axum::serve(listener, app).await.unwrap();
}
