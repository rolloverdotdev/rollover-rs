mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_invoices() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/invoices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{
                "id": "inv1", "wallet_address": "0xabc",
                "subscription_id": "sub1", "status": "paid",
                "base_amount": "9.99", "overage_amount": "0",
                "total_amount": "9.99"
            }],
            "total": 1, "limit": 100, "offset": 0
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let page = client.list_invoices(None).await.unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.data[0].status, "paid");
    assert_eq!(page.data[0].total_amount, "9.99");
}
