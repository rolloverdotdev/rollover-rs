mod common;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_organization() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "org1", "name": "Acme Corp", "slug": "acme",
            "logo": "https://example.com/logo.png",
            "webhook_url": "https://example.com/webhook"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let org = client.get_organization().await.unwrap();

    assert_eq!(org.id, "org1");
    assert_eq!(org.name, "Acme Corp");
    assert_eq!(org.slug, "acme");
}
