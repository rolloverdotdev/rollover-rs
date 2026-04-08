mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn org_mock() -> Mock {
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
}

fn sub_json() -> serde_json::Value {
    serde_json::json!({
        "id": "sub1", "wallet_address": "0xabc", "plan_id": "p1",
        "plan_name": "Starter", "status": "active", "mode": "test",
        "period_start": "2025-01-01T00:00:00Z", "period_end": "2025-02-01T00:00:00Z",
        "cancel_at_end": false
    })
}

#[tokio::test]
async fn test_list_subscriptions() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("GET"))
        .and(path("/v1/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [sub_json()], "total": 1, "limit": 100, "offset": 0
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let page = client.list_subscriptions(None).await.unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.data[0].status, "active");
}

#[tokio::test]
async fn test_get_subscription() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("GET"))
        .and(path("/v1/subscriptions/sub1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sub_json()))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let sub = client.get_subscription("sub1").await.unwrap();

    assert_eq!(sub.id, "sub1");
    assert_eq!(sub.plan_name, "Starter");
}

#[tokio::test]
async fn test_create_subscription() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("POST"))
        .and(path("/v1/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sub_json()))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let sub = client.create_subscription("0xabc", "starter").await.unwrap();

    assert_eq!(sub.wallet_address, "0xabc");
    assert_eq!(sub.status, "active");
}

#[tokio::test]
async fn test_cancel_subscription() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("DELETE"))
        .and(path("/v1/subscriptions/sub1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "sub1", "wallet_address": "0xabc", "plan_id": "p1",
            "plan_name": "Starter", "status": "active", "mode": "test",
            "cancel_at_end": true
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let sub = client.cancel_subscription("sub1").await.unwrap();

    assert!(sub.cancel_at_end);
}
