mod common;

use rollover::{CreateFeatureParams, CreatePlanParams, UpdatePlanParams};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn org_mock() -> Mock {
    Mock::given(method("GET"))
        .and(path("/v1/organization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "1", "name": "Acme", "slug": "acme"
        })))
}

fn plan_json() -> serde_json::Value {
    serde_json::json!({
        "id": "p1", "slug": "starter", "name": "Starter",
        "price_usdc": "9.99", "billing_period": "monthly",
        "is_active": true, "features": []
    })
}

#[tokio::test]
async fn test_list_plans() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("GET"))
        .and(path("/v1/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [plan_json()], "total": 1, "limit": 100, "offset": 0
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let page = client.list_plans(None).await.unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.data[0].slug, "starter");
}

#[tokio::test]
async fn test_get_plan() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("GET"))
        .and(path("/v1/plans/starter"))
        .respond_with(ResponseTemplate::new(200).set_body_json(plan_json()))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let plan = client.get_plan("starter").await.unwrap();

    assert_eq!(plan.slug, "starter");
    assert_eq!(plan.price_usdc, "9.99");
}

#[tokio::test]
async fn test_create_plan() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("POST"))
        .and(path("/v1/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(plan_json()))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let plan = client
        .create_plan(&CreatePlanParams {
            slug: "starter".to_string(),
            name: "Starter".to_string(),
            price_usdc: "9.99".to_string(),
            billing_period: Some("monthly".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(plan.name, "Starter");
}

#[tokio::test]
async fn test_update_plan() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("PUT"))
        .and(path("/v1/plans/starter"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "p1", "slug": "starter", "name": "Starter Plus",
            "price_usdc": "9.99", "billing_period": "monthly"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let plan = client
        .update_plan(
            "starter",
            &UpdatePlanParams {
                name: Some("Starter Plus".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(plan.name, "Starter Plus");
}

#[tokio::test]
async fn test_archive_plan() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("DELETE"))
        .and(path("/v1/plans/starter"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    client.archive_plan("starter").await.unwrap();
}

#[tokio::test]
async fn test_create_feature() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("POST"))
        .and(path("/v1/plans/starter/features"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "f1", "feature_slug": "api-calls", "name": "API Calls",
            "limit_amount": 10000, "reset_period": "monthly"
        })))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let feature = client
        .create_feature(
            "starter",
            &CreateFeatureParams {
                feature_slug: "api-calls".to_string(),
                name: "API Calls".to_string(),
                limit_amount: Some(10000),
                reset_period: Some("monthly".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(feature.feature_slug, "api-calls");
    assert_eq!(feature.limit_amount, 10000);
}

#[tokio::test]
async fn test_delete_feature() {
    let server = MockServer::start().await;
    org_mock().mount(&server).await;

    Mock::given(method("DELETE"))
        .and(path("/v1/plans/starter/features/api-calls"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    client.delete_feature("starter", "api-calls").await.unwrap();
}

#[tokio::test]
async fn test_list_pricing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/pricing/acme"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            plan_json()
        ])))
        .mount(&server)
        .await;

    let client = common::test_client(&server).await;
    let plans = client.list_pricing("acme").await.unwrap();

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].slug, "starter");
}
