// Check and Track
//
// The core Rollover pattern is to verify a wallet has feature access before
// doing any work, then record usage after the operation succeeds.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example check_and_track

use rollover::Rollover;

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    let result = ro.check(wallet, "api-calls").await.unwrap();

    if !result.allowed {
        println!("Limit reached. {}/{} used.", result.used, result.limit);
        return;
    }

    println!("Access granted. {}/{} remaining.", result.remaining, result.limit);

    // Do your work here...

    let track = ro.track(wallet, "api-calls", 1, None).await.unwrap();
    println!("Tracked. {} used, {} remaining.", track.used, track.remaining);
}
