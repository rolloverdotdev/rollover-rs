// Batch Usage Report
//
// Query usage events with pagination and aggregate totals by feature,
// demonstrating both page-by-page and collect-all pagination helpers.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example pagination

use std::collections::HashMap;

use rollover::{collect_all, pages, ListOptions, Rollover};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let opts = ListOptions {
        limit: 50,
        ..Default::default()
    };

    // Collect loads all events into memory at once.
    let all = collect_all(|o| ro.list_usage(Some(o)), Some(opts.clone())).await.unwrap();

    let mut by_feature: HashMap<String, f64> = HashMap::new();
    for e in &all {
        let amt: f64 = e.amount.parse().unwrap_or(0.0);
        *by_feature.entry(e.feature_slug.clone()).or_default() += amt;
    }

    println!("Total events: {}", all.len());
    for (f, total) in &by_feature {
        println!("  {:<25} {:.0} units", f, total);
    }

    // Pages fetches one page at a time.
    println!("\nPage-by-page:");
    let mut iter = pages(|o| ro.list_usage(Some(o)), Some(opts));
    let mut page_num = 0;
    while iter.next().await {
        page_num += 1;
        if let Some(page) = iter.page() {
            println!("Page {}: {} events", page_num, page.data.len());
        }
    }
    if let Some(err) = iter.err() {
        eprintln!("Error: {}", err);
    }
}
