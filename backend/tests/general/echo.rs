use std::collections::HashMap;

use fmn_backend::models::responses::EchoResponse;
use reqwest::StatusCode;

use crate::common;

#[tokio::test]
async fn endpoint_returns_same_message() {
    let state = common::setup().await;
    let response = reqwest::get("http://localhost:3000?message=helo")
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let model = response.json::<EchoResponse>().await.unwrap();
    assert_eq!(model.message, "helo".to_string());
    common::teardown(state).await;
}
#[tokio::test]
async fn endpoint_returns_error_when_error() {
    let state = common::setup().await;
    let response = reqwest::get("http://localhost:3000?message=error")
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let model = response.json::<HashMap<String, String>>().await.unwrap();
    assert_eq!(
        model.get("error"),
        Some(&"i dont like the error word".to_string())
    );
    common::teardown(state).await;
}
#[tokio::test]
async fn endpoint_returns_error_when_no_message() {
    let state = common::setup().await;
    let response = reqwest::get("http://localhost:3000")
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let model = response.json::<HashMap<String, String>>().await.unwrap();
    assert_eq!(
        model.get("error"),
        Some(&"unprocessable entity".to_string())
    );
    common::teardown(state).await;
}
