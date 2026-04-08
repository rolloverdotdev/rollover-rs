// Error Handling
//
// Handle Rollover API errors by inspecting the status code and error code,
// allowing your application to respond differently to authentication failures,
// rate limits, and other error conditions.
//
//     ROLLOVER_API_KEY=ro_test_... cargo run --example error_handling

use rollover::{error_code, is_error_code, Rollover, RolloverError};

#[tokio::main]
async fn main() {
    let ro = Rollover::from_env().unwrap();

    match ro.check("0xinvalid", "api-calls").await {
        Ok(result) => println!("Allowed: {}", result.allowed),
        Err(ref e) => {
            match e {
                RolloverError::Api {
                    status, message, ..
                } => {
                    println!("API error: {} (status {})", message, status);
                    if e.temporary() {
                        println!("This is a transient error, safe to retry.");
                    }
                }
                _ => println!("Network or other error: {}", e),
            }
        }
    }

    // Use is_error_code for clean checks.
    if let Err(ref e) = ro.get_plan("nonexistent-plan").await {
        if is_error_code(e, error_code::NOT_FOUND) {
            println!("\nPlan not found (checked via is_error_code).");
        }
    }

    // Match on error codes directly.
    if let Err(ref e) = ro.grant_credits("0xabc", -1, None).await {
        match e {
            RolloverError::Api { code, message, .. } if code == error_code::VALIDATION => {
                println!("\nValidation error: {}", message);
            }
            RolloverError::Api { code, .. } if code == error_code::UNAUTHORIZED => {
                println!("\nCheck your API key.");
            }
            _ => println!("\nUnexpected: {}", e),
        }
    }
}
