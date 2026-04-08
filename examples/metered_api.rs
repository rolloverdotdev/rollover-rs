// Metered API Server
//
// Track usage for multiple features across different routes, with each
// route mapped to a Rollover feature.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example metered_api

use rollover::Rollover;

async fn handle_request(ro: &Rollover, wallet: &str, feature: &str, result: &str) {
    let check = match ro.check(wallet, feature).await {
        Ok(r) => r,
        Err(_) => {
            println!("[{}] check failed", feature);
            return;
        }
    };

    if !check.allowed {
        println!("[{}] rate limited", feature);
        return;
    }

    println!("[{}] {}", feature, result);
    let _ = ro.track(wallet, feature, 1, None).await;
}

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Simulate requests to different metered endpoints.
    handle_request(&ro, wallet, "translations", r#"{"text":"translated"}"#).await;
    handle_request(&ro, wallet, "summaries", r#"{"text":"summarized"}"#).await;
    handle_request(&ro, wallet, "embeddings", r#"{"embeddings":[0.1,0.2]}"#).await;
}
