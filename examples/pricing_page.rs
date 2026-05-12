// Pricing Page
//
// Fetch your plans for a pricing page with a single API call that returns
// each plan and its included features.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example pricing_page

use rollover::Rollover;

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    let org = ro.get_organization().await.unwrap();
    let plans = ro.list_pricing(&org.slug).await.unwrap();

    println!("Pricing for {}:", org.name);
    for p in &plans {
        println!(
            "\n  {} ({}) - ${}/{}",
            p.name, p.slug, p.price_usdc, p.billing_period
        );
        if p.trial_days > 0 {
            println!("    {} day trial", p.trial_days);
        }
        for f in &p.features {
            let name = f.feature.as_ref().map(|c| c.name.as_str()).unwrap_or("feature");
            if f.limit_amount > 0 {
                println!("    - {} (limit: {})", name, f.limit_amount);
            } else {
                println!("    - {} (unlimited)", name);
            }
        }
    }
}
