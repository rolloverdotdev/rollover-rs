// Provision a Customer
//
// A complete server-side onboarding flow that creates a plan with features,
// subscribes a wallet, and grants welcome credits for a new customer.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example provisioning

use rollover::{CreateFeatureParams, CreatePlanParams, GrantOptions, Rollover};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    let slug = format!(
        "starter-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 100000
    );

    // 1. Create a plan.
    let plan = ro
        .create_plan(&CreatePlanParams {
            slug: slug.clone(),
            name: "Starter".to_string(),
            price_usdc: "9.99".to_string(),
            billing_period: Some("monthly".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("Created plan: {} ({})", plan.name, plan.slug);

    // 2. Add features.
    let f = ro
        .create_feature(
            &slug,
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
    println!("  Added feature: {} (limit: {})", f.feature_slug, f.limit_amount);

    // 3. Subscribe a wallet.
    let wallet = format!(
        "0x{:0>40}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let sub = ro.create_subscription(&wallet, &slug).await.unwrap();
    let short = if wallet.len() > 12 {
        format!("{}...", &wallet[..12])
    } else {
        wallet.clone()
    };
    println!("Subscribed {} to {} (status: {})", short, sub.plan_name, sub.status);

    // 4. Grant welcome credits.
    let grant = ro
        .grant_credits(
            &wallet,
            500,
            Some(&GrantOptions {
                description: "Welcome bonus".to_string(),
                ..Default::default()
            }),
        )
        .await
        .unwrap();
    println!("Granted 500 credits (balance: {})", grant.balance);

    // Cleanup.
    let _ = ro.archive_plan(&slug).await;
}
