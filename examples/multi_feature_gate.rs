// Multi-Feature Gate
//
// Check multiple features in one call before starting an operation that
// requires all of them, such as an AI pipeline consuming both API calls
// and image generation credits.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example multi_feature_gate

use rollover::{Atomicity, BatchCheckItem, BatchTrackEvent, Rollover};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // check_batch resolves the subscription once and answers for every
    // feature in a single request. Supplying amount per feature makes
    // allowed reflect whether N units would succeed, not just whether any
    // quota remains.
    let gate = ro
        .check_batch(
            wallet,
            &[
                BatchCheckItem { feature: "api-calls".to_string(), amount: Some(1) },
                BatchCheckItem { feature: "image-gen".to_string(), amount: Some(1) },
            ],
        )
        .await
        .unwrap();

    let blocked: Vec<String> = gate
        .results
        .iter()
        .filter(|r| !r.allowed)
        .map(|r| r.feature.clone())
        .collect();
    if !blocked.is_empty() {
        println!("Blocked on: {}", blocked.join(", "));
        println!("Please upgrade your plan to continue.");
        return;
    }

    println!("All features available. Running pipeline...");
    println!("Pipeline completed.");

    // track_batch records every event in one call and groups the resulting
    // usage_events rows under a shared batch_id. AllOrNothing rolls the
    // whole batch back if any event would block.
    let result = ro
        .track_batch(
            wallet,
            &[
                BatchTrackEvent { feature: "api-calls".to_string(), amount: 1 },
                BatchTrackEvent { feature: "image-gen".to_string(), amount: 1 },
            ],
            Atomicity::AllOrNothing,
            None,
        )
        .await
        .unwrap();
    println!("Usage tracked (batch {}).", result.batch_id);
}
