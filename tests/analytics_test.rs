mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_analytics() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "mrr": "99.99",
            "active_subs": 10,
            "total_revenue": "599.94",
            "top_features": [{"feature_slug": "api-calls", "total_used": 5000}],
            "recent_activity": []
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let stats = client.get_analytics().await.unwrap();

    assert_eq!(stats.mrr, "99.99");
    assert_eq!(stats.active_subs, 10);
    assert_eq!(stats.top_features.len(), 1);
    assert_eq!(stats.top_features[0].feature_slug, "api-calls");
}
