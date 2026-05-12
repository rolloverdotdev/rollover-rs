// Plan Management
//
// Create, update, and manage plans and features using the admin API.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example plans

use rollover::{CreatePlanParams, LinkFeatureParams, Rollover, UpdatePlanParams};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    let slug = format!(
        "demo-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 100000
    );

    // Create a plan.
    let plan = ro
        .create_plan(&CreatePlanParams {
            slug: slug.clone(),
            name: "Demo Plan".to_string(),
            price_usdc: "9.99".to_string(),
            billing_period: Some("monthly".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("Created plan: {}", plan.name);

    // Update the plan.
    let updated = ro
        .update_plan(
            &slug,
            &UpdatePlanParams {
                name: Some("Demo Plan (Updated)".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    println!("Updated plan: {}", updated.name);

    // Link a catalog feature to the plan. Unknown feature slugs auto-create a metered
    // catalog feature on the server.
    let link = ro
        .link_feature(
            &slug,
            &LinkFeatureParams {
                feature_slug: Some("api-calls".to_string()),
                limit_amount: Some(10000),
                reset_period: Some("monthly".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let feature_slug = link
        .feature
        .as_ref()
        .map(|f| f.slug.as_str())
        .unwrap_or("api-calls");
    println!(
        "Linked feature: {} (limit: {})",
        feature_slug, link.limit_amount
    );

    // Cleanup.
    ro.unlink_feature(&slug, "api-calls").await.unwrap();
    ro.archive_plan(&slug).await.unwrap();
    println!("Cleaned up.");
}
