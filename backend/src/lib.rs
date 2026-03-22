use std::{env, sync::Arc, time::Duration};

use axum::{
    Router,
    http::{Request, Response},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{
    layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

use crate::{
    endpoints::{
        cart::get_cart_router, items::get_items_router,
        recipes::get_recipes_router,
    },
    queries::Queries,
    state::AppState,
};

pub mod endpoints;
pub mod methods;
pub mod models;
pub mod queries;
pub mod state;

pub async fn run() -> Result<(), String> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").map_err(|_| "no env")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|_| "could not connect")?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|_| "could not migrate")?;

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

    let queries = Queries::new(pool);
    let state = AppState {
        queries: Arc::new(Mutex::new(queries)),
    };
    let app = Router::new()
        .nest("/items", get_items_router())
        .nest("/recipes", get_recipes_router())
        .nest("/cart", get_cart_router())
        .with_state(state)
        .layer(trace_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("server running");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
