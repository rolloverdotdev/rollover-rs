// Idempotent Tracking
//
// Avoid double-counting in distributed systems by using idempotency keys,
// where the same Idempotency-Key always produces the same result.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example idempotency

use rollover::{Rollover, TrackOptions};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Use a deterministic key tied to the operation being tracked.
    let opts = TrackOptions {
        idempotency_key: "order-12345-image-gen".to_string(),
    };

    // First call records the usage.
    let r1 = ro
        .track(wallet, "api-calls", 1, Some(&opts))
        .await
        .unwrap();
    println!("First:  used={} remaining={}", r1.used, r1.remaining);

    // Second call with same key returns the cached result.
    let r2 = ro
        .track(wallet, "api-calls", 1, Some(&opts))
        .await
        .unwrap();
    println!(
        "Second: used={} remaining={} (same as first, not double-counted)",
        r2.used, r2.remaining
    );
}
