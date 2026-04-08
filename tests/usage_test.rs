mod common;

use rollover::TrackOptions;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_check() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check"))
        .and(query_param("wallet", "0xabc"))
        .and(query_param("feature", "api-calls"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "allowed": true,
            "used": 5,
            "remaining": 95,
            "limit": 100,
            "plan": "starter",
            "credit_balance": 50,
            "credit_cost": 1
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client.check("0xabc", "api-calls").await.unwrap();

    assert!(result.allowed);
    assert_eq!(result.used, 5);
    assert_eq!(result.remaining, 95);
    assert_eq!(result.limit, 100);
    assert_eq!(result.plan, "starter");
}

#[tokio::test]
async fn test_check_missing_optional_fields() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"allowed": false})),
        )
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client.check("0xabc", "api-calls").await.unwrap();

    assert!(!result.allowed);
    assert_eq!(result.used, 0);
    assert_eq!(result.remaining, 0);
    assert_eq!(result.limit, 0);
}

#[tokio::test]
async fn test_track() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/track"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "allowed": true, "used": 8, "remaining": 92
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client.track("0xabc", "api-calls", 3, None).await.unwrap();

    assert_eq!(result.used, 8);
    assert_eq!(result.remaining, 92);
}

#[tokio::test]
async fn test_track_with_idempotency_key() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/track"))
        .and(header("Idempotency-Key", "order-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "allowed": true, "used": 1, "remaining": 99
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let opts = TrackOptions {
        idempotency_key: "order-123".to_string(),
    };
    let result = client
        .track("0xabc", "api-calls", 1, Some(&opts))
        .await
        .unwrap();

    assert_eq!(result.used, 1);
}

#[tokio::test]
async fn test_track_without_idempotency_key() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/track"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "allowed": true, "used": 1, "remaining": 99
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client.track("0xabc", "api-calls", 1, None).await.unwrap();
    assert_eq!(result.used, 1);
}
