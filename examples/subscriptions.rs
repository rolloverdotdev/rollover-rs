// Subscription Lifecycle
//
// Manage the full subscription lifecycle by listing active subscriptions,
// filtering by wallet, and inspecting subscription details.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example subscriptions

use rollover::{ListOptions, Rollover};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    // List all active subscriptions.
    let subs = ro
        .list_subscriptions(Some(ListOptions {
            status: "active".to_string(),
            limit: 5,
            ..Default::default()
        }))
        .await
        .unwrap();
    println!("Active subscriptions: {}", subs.total);

    for s in &subs.data {
        println!(
            "  {} -> {} (status: {}, ends: {})",
            s.wallet_address, s.plan_name, s.status, s.period_end
        );
    }

    // Filter by wallet.
    if let Some(first) = subs.data.first() {
        let wallet = &first.wallet_address;
        let filtered = ro
            .list_subscriptions(Some(ListOptions {
                wallet: wallet.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();
        let short = if wallet.len() > 12 {
            format!("{}...", &wallet[..12])
        } else {
            wallet.clone()
        };
        println!("\nSubscriptions for {}: {}", short, filtered.total);
    }
}
