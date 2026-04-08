// Usage Middleware
//
// An HTTP middleware that gates endpoints by verifying usage before
// handling the request and recording consumption after a successful response.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example middleware

use rollover::Rollover;
use std::sync::Arc;

/// Check usage before handling, track after success.
async fn usage_gate(ro: &Rollover, wallet: &str, feature: &str) -> bool {
    let result = match ro.check(wallet, feature).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("usage check failed: {}", e);
            return false;
        }
    };

    if !result.allowed {
        println!(
            "Rate limited: {}/{} used for {}",
            result.used, result.limit, feature
        );
        return false;
    }

    // Do the work here...
    println!("Request handled for feature: {}", feature);

    // Track usage after success.
    if let Err(e) = ro.track(wallet, feature, 1, None).await {
        eprintln!("failed to track usage: {}", e);
    }

    true
}

#[tokio::main]
async fn main() {
    let ro = Arc::new(Rollover::from_env().unwrap());
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Simulate gated requests.
    usage_gate(&ro, wallet, "translations").await;
    usage_gate(&ro, wallet, "translations").await;
}
