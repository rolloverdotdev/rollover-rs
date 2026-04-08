mod common;

use rollover::{error_code, is_error_code, RolloverError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_parse_error_json() {
    let err = rollover::RolloverError::Api {
        status: 400,
        code: "validation_error".to_string(),
        message: "invalid wallet".to_string(),
    };
    assert!(err.is_code("validation_error"));
    assert!(!err.temporary());
}

#[test]
fn test_temporary() {
    let cases = vec![
        (429, true),
        (500, true),
        (502, true),
        (400, false),
        (401, false),
        (404, false),
    ];

    for (status, expected) in cases {
        let err = RolloverError::Api {
            status,
            code: "test".to_string(),
            message: "test".to_string(),
        };
        assert_eq!(
            err.temporary(),
            expected,
            "temporary() for status {}: expected {}",
            status,
            expected
        );
    }
}

#[tokio::test]
async fn test_is_error_code() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "code": "not_found", "message": "plan not found"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let err = client.check("0xabc", "feature").await.unwrap_err();

    assert!(is_error_code(&err, error_code::NOT_FOUND));
    assert!(!is_error_code(&err, error_code::RATE_LIMIT));
}

#[tokio::test]
async fn test_error_display() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "code": "unauthorized", "message": "bad key"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let err = client.check("0xabc", "feature").await.unwrap_err();

    match &err {
        RolloverError::Api {
            status,
            code,
            message,
        } => {
            assert_eq!(*status, 401);
            assert_eq!(code, "unauthorized");
            assert_eq!(message, "bad key");
        }
        _ => panic!("expected Api error"),
    }
}

#[test]
fn test_error_empty_body() {
    // Simulate parseError with empty body by testing the Display impl
    let err = RolloverError::Api {
        status: 503,
        code: "http_error".to_string(),
        message: "Service Unavailable".to_string(),
    };
    assert!(err.to_string().contains("503"));
    assert!(err.temporary());
}
