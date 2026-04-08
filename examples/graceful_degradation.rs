// Graceful Degradation
//
// Handle billing errors gracefully by failing open, and return helpful
// responses with usage details when a wallet hits its limit.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example graceful_degradation

use rollover::Rollover;

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Check if the wallet can use the feature.
    match ro.check(wallet, "generations").await {
        Ok(result) => {
            if result.allowed {
                println!("Access granted, generating...");
                let _ = ro.track(wallet, "generations", 1, None).await;
                println!("Done.");
            } else {
                // Limit reached: return helpful response with upgrade info.
                println!("Generation limit reached.");
                println!("  used:  {}", result.used);
                println!("  limit: {}", result.limit);
                println!("  plan:  {}", result.plan);
                println!(
                    "  You've used all your generations for this period. Upgrade for more."
                );
            }
        }
        Err(e) => {
            // Billing check failed, fail open.
            eprintln!("Billing check failed: {} (failing open)", e);
            println!("Generating anyway...");
        }
    }
}
