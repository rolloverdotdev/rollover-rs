mod common;

use rollover::GrantOptions;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_credits() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .and(query_param("wallet", "0xabc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "wallet": "0xabc", "balance": 500
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let balance = client.get_credits("0xabc").await.unwrap();

    assert_eq!(balance.wallet, "0xabc");
    assert_eq!(balance.balance, 500);
}

#[tokio::test]
async fn test_grant_credits() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/credits"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "balance": 600, "granted": 100
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client
        .grant_credits(
            "0xabc",
            100,
            Some(&GrantOptions {
                description: "Welcome bonus".to_string(),
                ..Default::default()
            }),
        )
        .await
        .unwrap();

    assert_eq!(result.granted, 100);
    assert_eq!(result.balance, 600);
}

#[tokio::test]
async fn test_list_credit_transactions() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/credits/transactions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": "tx1", "wallet_address": "0xabc", "amount": 100, "type": "grant", "description": "bonus"}],
            "total": 1, "limit": 100, "offset": 0
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let page = client.list_credit_transactions(None).await.unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.data[0].type_, "grant");
}
