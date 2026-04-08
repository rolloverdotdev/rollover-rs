// Webhook Receiver
//
// Process real-time events from Rollover by registering a webhook URL
// in the dashboard and handling events as they arrive.
//
//     cargo run --example webhooks

use serde::Deserialize;

#[derive(Deserialize)]
struct WebhookEvent {
    #[serde(rename = "type")]
    event_type: String,
    data: serde_json::Value,
}

#[derive(Deserialize)]
struct SubscriptionData {
    wallet_address: String,
    plan_name: String,
}

fn handle_event(body: &[u8]) {
    let event: WebhookEvent = match serde_json::from_slice(body) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("invalid json: {}", e);
            return;
        }
    };

    match event.event_type.as_str() {
        "subscription.created" => {
            if let Ok(data) = serde_json::from_value::<SubscriptionData>(event.data) {
                println!(
                    "New subscription: {} -> {}",
                    data.wallet_address, data.plan_name
                );
            }
        }
        "subscription.canceled" => {
            if let Ok(data) = serde_json::from_value::<SubscriptionData>(event.data) {
                println!(
                    "Canceled: {} from {}",
                    data.wallet_address, data.plan_name
                );
            }
        }
        _ => println!("Received event: {}", event.event_type),
    }
}

fn main() {
    // Simulate receiving webhook events.
    let events = vec![
        r#"{"type":"subscription.created","data":{"wallet_address":"0xabc...","plan_name":"Starter"}}"#,
        r#"{"type":"subscription.canceled","data":{"wallet_address":"0xdef...","plan_name":"Pro"}}"#,
        r#"{"type":"invoice.created","data":{"id":"inv_123"}}"#,
    ];

    println!("Webhook receiver processing events:");
    for event in events {
        handle_event(event.as_bytes());
    }
}
