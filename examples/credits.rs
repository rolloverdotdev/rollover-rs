// Credit-Gated Access
//
// Protect expensive operations by requiring an available credit balance,
// with credits automatically deducted according to the feature's credit_cost.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example credits

use rollover::{GrantOptions, Rollover};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Check credit balance.
    let balance = ro.get_credits(wallet).await.unwrap();
    println!("Credit balance: {}", balance.balance);

    // Grant credits.
    let grant = ro
        .grant_credits(
            wallet,
            500,
            Some(&GrantOptions {
                description: "Welcome bonus".to_string(),
                ..Default::default()
            }),
        )
        .await
        .unwrap();
    println!("Granted {} credits. New balance: {}", grant.granted, grant.balance);

    // Check if the wallet can use a credit-gated feature.
    let result = ro.check(wallet, "image-gen").await.unwrap();
    if !result.allowed {
        println!(
            "Not enough credits. Balance: {}, cost: {}",
            result.credit_balance, result.credit_cost
        );
        return;
    }

    // Do the expensive work, then track usage.
    let track = ro.track(wallet, "image-gen", 1, None).await.unwrap();
    println!("Tracked. Credits remaining: {}", track.credit_balance);
}
