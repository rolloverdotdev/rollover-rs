// Multi-Feature Gate
//
// Check multiple features concurrently before starting an operation that
// requires all of them, such as an AI pipeline consuming both API calls
// and image generation credits.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example multi_feature_gate

use rollover::Rollover;
use std::sync::Arc;
use tokio::task::JoinSet;

/// Check multiple features concurrently, returning the list of blocked features.
async fn check_all(ro: &Arc<Rollover>, wallet: &str, features: &[&str]) -> Vec<String> {
    let mut set = JoinSet::new();

    for &feature in features {
        let ro = ro.clone();
        let wallet = wallet.to_string();
        let feature = feature.to_string();
        set.spawn(async move {
            let result = ro.check(&wallet, &feature).await;
            match result {
                Ok(r) if r.allowed => None,
                _ => Some(feature),
            }
        });
    }

    let mut blocked = Vec::new();
    while let Some(result) = set.join_next().await {
        if let Ok(Some(feature)) = result {
            blocked.push(feature);
        }
    }
    blocked
}

/// Track usage for multiple features concurrently.
async fn track_all(ro: &Arc<Rollover>, wallet: &str, features: &[(&str, i64)]) {
    let mut set = JoinSet::new();

    for &(feature, amount) in features {
        let ro = ro.clone();
        let wallet = wallet.to_string();
        let feature = feature.to_string();
        set.spawn(async move {
            if let Err(e) = ro.track(&wallet, &feature, amount, None).await {
                eprintln!("rollover: track {} failed: {}", feature, e);
            }
        });
    }

    while set.join_next().await.is_some() {}
}

#[tokio::main]
async fn main() {
    let ro = Arc::new(Rollover::from_env().unwrap());
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // This operation requires both api-calls and image-gen.
    let required = &["api-calls", "image-gen"];

    let blocked = check_all(&ro, wallet, required).await;
    if !blocked.is_empty() {
        println!("Blocked on: {}", blocked.join(", "));
        println!("Please upgrade your plan to continue.");
        return;
    }

    println!("All features available. Running pipeline...");
    println!("Pipeline completed.");

    track_all(&ro, wallet, &[("api-calls", 1), ("image-gen", 1)]).await;
    println!("Usage tracked for all features.");
}
