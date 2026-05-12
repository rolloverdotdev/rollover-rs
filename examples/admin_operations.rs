// Admin Operations
//
// Manage plans, features, subscriptions, invoices, and credit transactions
// using the admin API, covering the full set of operations available to
// API key holders beyond the core check and track workflow.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example admin_operations

use rollover::{
    CreatePlanParams, GrantOptions, LinkFeatureParams, ListOptions, Rollover, UpdatePlanParams,
};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    let slug = format!(
        "admin-demo-{}",
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
            name: "Admin Demo".to_string(),
            price_usdc: "19.99".to_string(),
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
                name: Some("Admin Demo (Updated)".to_string()),
                description: Some("Updated via SDK".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    println!("Updated plan: {}", updated.name);

    // Link a catalog feature to the plan.
    let link = ro
        .link_feature(
            &slug,
            &LinkFeatureParams {
                feature_slug: Some("requests".to_string()),
                limit_amount: Some(5000),
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
        .unwrap_or("requests");
    println!(
        "Linked feature: {} (limit: {})",
        feature_slug, link.limit_amount
    );

    // Subscribe a wallet and inspect the subscription.
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
    println!("Subscribed: {} (status: {})", short, sub.status);

    let fetched = ro.get_subscription(&sub.id).await.unwrap();
    println!(
        "Fetched subscription: plan={}, period ends {}",
        fetched.plan_name, fetched.period_end
    );

    // Grant credits and list transactions.
    let _ = ro
        .grant_credits(
            &wallet,
            100,
            Some(&GrantOptions {
                description: "Demo grant".to_string(),
                ..Default::default()
            }),
        )
        .await;
    let txns = ro
        .list_credit_transactions(Some(ListOptions {
            wallet: wallet.clone(),
            ..Default::default()
        }))
        .await
        .unwrap();
    println!("Credit transactions: {}", txns.total);
    for tx in &txns.data {
        println!("  {}: {} credits ({})", tx.type_, tx.amount, tx.description);
    }

    // List invoices.
    let invoices = ro
        .list_invoices(Some(ListOptions {
            wallet: wallet.clone(),
            ..Default::default()
        }))
        .await
        .unwrap();
    println!("Invoices: {}", invoices.total);

    // Cleanup.
    let _ = ro.unlink_feature(&slug, "requests").await;
    let _ = ro.archive_plan(&slug).await;
    println!("Cleaned up.");
}
