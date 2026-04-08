// Usage Dashboard
//
// Pull analytics stats and paginated usage events to display in an admin
// dashboard, combining MRR, active subscriptions, and event history.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example usage_dashboard

use rollover::{ListOptions, Rollover};

fn short_addr(s: &str) -> String {
    if s.len() <= 12 {
        s.to_string()
    } else {
        format!("{}...", &s[..10])
    }
}

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    // 1. Fetch high-level analytics.
    let stats = ro.get_analytics().await.unwrap();

    println!("Dashboard");
    println!("MRR:           ${}", stats.mrr);
    println!("Active subs:   {}", stats.active_subs);
    println!("Total revenue: ${}", stats.total_revenue);

    if !stats.top_features.is_empty() {
        println!("\nTop features:");
        for f in &stats.top_features {
            println!("  {:<20} {} events", f.feature_slug, f.total_used);
        }
    }

    // 2. Fetch recent usage events.
    let events = ro
        .list_usage(Some(ListOptions {
            limit: 10,
            ..Default::default()
        }))
        .await
        .unwrap();

    println!(
        "\nRecent events (showing {} of {}):",
        events.data.len(),
        events.total
    );
    for e in &events.data {
        println!(
            "  {}  {:<15}  {} units  {}",
            short_addr(&e.wallet_address),
            e.feature_slug,
            e.amount,
            e.recorded_at,
        );
    }
}
