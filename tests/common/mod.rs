use rollover::Rollover;
use wiremock::MockServer;

pub async fn test_client(mock_server: &MockServer) -> Rollover {
    Rollover::builder()
        .api_key("ro_test_key")
        .base_url(&mock_server.uri())
        .build()
        .unwrap()
}
