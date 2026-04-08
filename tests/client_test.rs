mod common;

use rollover::Rollover;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_mode_detection() {
    let c = Rollover::new("ro_test_abc").unwrap();
    assert_eq!(c.mode(), "test");

    let c = Rollover::new("ro_live_abc").unwrap();
    assert_eq!(c.mode(), "live");
}

#[tokio::test]
async fn test_resolve_slug_caches_success() {
    let server = MockServer::start().await;

    // The org endpoint is called once to resolve the slug, then cached.
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "mrr": "0", "active_subs": 0, "total_revenue": "0",
            "top_features": [], "recent_activity": []
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;

    // First admin call resolves slug via /v1/organization.
    let _ = client.get_analytics().await.unwrap();

    // Second admin call uses cached slug (org mock expects exactly 1 call).
    let _ = client.get_analytics().await.unwrap();
}

#[tokio::test]
async fn test_api_key_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check"))
        .and(header("X-API-Key", "ro_test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "allowed": true, "used": 0, "remaining": 100, "limit": 100, "plan": "starter"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let result = client.check("0xabc", "feature").await.unwrap();
    assert!(result.allowed);
}
