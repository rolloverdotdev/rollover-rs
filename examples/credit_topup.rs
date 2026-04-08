// Credit Top-Up
//
// Monitor a wallet's credit balance and automatically grant more credits
// when the balance drops below a configured threshold.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example credit_topup

use rollover::{GrantOptions, Rollover};
use std::time::Duration;

const LOW_BALANCE_THRESHOLD: i64 = 100;
const TOP_UP_AMOUNT: i64 = 500;
const CHECK_INTERVAL: Duration = Duration::from_secs(30);

async fn check_and_topup(ro: &Rollover, wallet: &str) {
    let balance = match ro.get_credits(wallet).await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("failed to check balance: {}", e);
            return;
        }
    };

    print!("[{}] balance: {}", chrono_now(), balance.balance);

    if balance.balance < LOW_BALANCE_THRESHOLD {
        println!(" (low! granting {} credits)", TOP_UP_AMOUNT);

        match ro
            .grant_credits(
                wallet,
                TOP_UP_AMOUNT,
                Some(&GrantOptions {
                    description: "Auto top-up: balance below threshold".to_string(),
                    ..Default::default()
                }),
            )
            .await
        {
            Ok(grant) => println!("  new balance: {}", grant.balance),
            Err(e) => eprintln!("  top-up failed: {}", e),
        }
    } else {
        println!(" (ok)");
    }
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hours = (now % 86400) / 3600;
    let minutes = (now % 3600) / 60;
    let seconds = now % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    let short = if wallet.len() > 12 {
        format!("{}...", &wallet[..12])
    } else {
        wallet.to_string()
    };
    println!(
        "Monitoring {} (threshold: {}, top-up: {})",
        short, LOW_BALANCE_THRESHOLD, TOP_UP_AMOUNT
    );

    check_and_topup(&ro, wallet).await;
    loop {
        tokio::time::sleep(CHECK_INTERVAL).await;
        check_and_topup(&ro, wallet).await;
    }
}
