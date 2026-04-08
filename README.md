# Rollover Rust SDK

The official Rust client for the [Rollover](https://rollover.dev) API, a subscription billing platform built on [x402](https://github.com/coinbase/x402) that settles in USDC on-chain.

## Install

```toml
[dependencies]
rollover = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust
use rollover::Rollover;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ro = Rollover::from_env()?; // reads ROLLOVER_API_KEY env var
    let wallet = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    // Check if the wallet can use a feature.
    let result = ro.check(wallet, "api-calls").await?;
    if !result.allowed {
        println!("Limit reached");
        return Ok(());
    }

    // Do your work, then track the usage.
    ro.track(wallet, "api-calls", 1, None).await?;

    Ok(())
}
```

## Configuration

```rust
// Default: reads ROLLOVER_API_KEY from environment
let ro = Rollover::from_env()?;

// Explicit API key
let ro = Rollover::new("ro_test_...")?;

// Custom base URL (for local dev)
let ro = Rollover::builder()
    .api_key("ro_test_...")
    .base_url("http://localhost:9000")
    .build()?;

// Custom HTTP client
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()?;
let ro = Rollover::builder()
    .api_key("ro_test_...")
    .http_client(client)
    .build()?;
```

The mode (`test` or `live`) is parsed from the API key prefix (`ro_test_` or `ro_live_`). The default HTTP client has a 30-second timeout.

## API

### Core

```rust
// Check if a wallet can use a feature.
let result = ro.check(wallet, "api-calls").await?;
// result.allowed, result.used, result.remaining, result.limit,
// result.plan, result.credit_balance, result.credit_cost

// Track usage.
let result = ro.track(wallet, "api-calls", 1, None).await?;
// result.allowed, result.used, result.remaining, result.credit_balance

// Track with idempotency key to prevent double-counting.
let opts = TrackOptions { idempotency_key: "order-12345".to_string() };
let result = ro.track(wallet, "api-calls", 1, Some(&opts)).await?;
```

### Credits

```rust
// Get credit balance.
let balance = ro.get_credits(wallet).await?;
// balance.wallet, balance.balance

// Grant credits.
let result = ro.grant_credits(wallet, 500, Some(&GrantOptions {
    description: "Welcome bonus".to_string(),
    ..Default::default()
})).await?;
// result.balance, result.granted

// List credit transaction history.
let txns = ro.list_credit_transactions(Some(ListOptions {
    wallet: wallet.to_string(),
    ..Default::default()
})).await?;
```

### Plans

```rust
// List plans.
let plans = ro.list_plans(Some(ListOptions { limit: 10, ..Default::default() })).await?;

// Get a plan.
let plan = ro.get_plan("starter").await?;

// Create a plan.
let plan = ro.create_plan(&CreatePlanParams {
    slug: "starter".to_string(),
    name: "Starter".to_string(),
    price_usdc: "9.99".to_string(),
    billing_period: Some("monthly".to_string()),
    ..Default::default()
}).await?;

// Update a plan.
let plan = ro.update_plan("starter", &UpdatePlanParams {
    name: Some("Starter Plus".to_string()),
    ..Default::default()
}).await?;

// Archive a plan.
ro.archive_plan("starter").await?;

// Add a feature to a plan.
let feature = ro.create_feature("starter", &CreateFeatureParams {
    feature_slug: "api-calls".to_string(),
    name: "API Calls".to_string(),
    limit_amount: Some(10000),
    reset_period: Some("monthly".to_string()),
    ..Default::default()
}).await?;

// Update a feature.
let feature = ro.update_feature("starter", "api-calls", &UpdateFeatureParams {
    limit_amount: Some(20000),
    ..Default::default()
}).await?;

// Delete a feature.
ro.delete_feature("starter", "api-calls").await?;

// List public pricing for a pricing page.
let plans = ro.list_pricing("your-org-slug").await?;
```

### Subscriptions

```rust
// List subscriptions.
let subs = ro.list_subscriptions(Some(ListOptions {
    wallet: "0xabc...".to_string(),
    status: "active".to_string(),
    ..Default::default()
})).await?;

// Get a single subscription.
let sub = ro.get_subscription(subscription_id).await?;

// Create a subscription (admin).
let sub = ro.create_subscription("0xabc...", "starter").await?;

// Cancel a subscription.
let sub = ro.cancel_subscription(subscription_id).await?;
```

### Usage and Analytics

```rust
// List usage events.
let events = ro.list_usage(Some(ListOptions {
    wallet: "0xabc...".to_string(),
    feature: "api-calls".to_string(),
    after: "2025-01-01T00:00:00Z".to_string(),
    ..Default::default()
})).await?;

// Get analytics stats.
let stats = ro.get_analytics().await?;
// stats.mrr, stats.active_subs, stats.total_revenue, stats.top_features

// List invoices.
let invoices = ro.list_invoices(Some(ListOptions {
    wallet: "0xabc...".to_string(),
    ..Default::default()
})).await?;

// Get organization info.
let org = ro.get_organization().await?;
```

## Pagination

All list methods accept `Option<ListOptions>` with `limit` and `offset` fields. For convenience, the SDK provides two helpers that handle pagination automatically.

```rust
use rollover::{collect_all, pages, ListOptions};

// Collect loads all items into a single Vec.
let all = collect_all(
    |opts| ro.list_usage(Some(opts)),
    Some(ListOptions { feature: "api-calls".to_string(), ..Default::default() }),
).await?;

// Pages iterates one page at a time without loading everything into memory.
let mut iter = pages(
    |opts| ro.list_usage(Some(opts)),
    Some(ListOptions { feature: "api-calls".to_string(), ..Default::default() }),
);
while iter.next().await {
    if let Some(page) = iter.page() {
        for e in &page.data {
            println!("{} {}", e.feature_slug, e.amount);
        }
    }
}
if let Some(err) = iter.err() {
    eprintln!("{}", err);
}
```

## Error handling

Non-2xx responses are returned as `RolloverError::Api` with a status code, error code, and message.

```rust
use rollover::{error_code, is_error_code, RolloverError};

match ro.check(wallet, "api-calls").await {
    Ok(result) => println!("Allowed: {}", result.allowed),
    Err(ref e) => match e {
        RolloverError::Api { status, code, message } => {
            println!("API error: {} (status {})", message, status);
            if e.temporary() {
                println!("Transient error, safe to retry.");
            }
        }
        _ => println!("Network or other error: {}", e),
    }
}

// Use is_error_code for clean checks.
if let Err(ref e) = ro.get_plan("nonexistent").await {
    if is_error_code(e, error_code::NOT_FOUND) {
        println!("Not found.");
    }
}
```

Error code constants: `error_code::INVALID_API_KEY`, `error_code::UNAUTHORIZED`, `error_code::RATE_LIMIT`, `error_code::NOT_FOUND`, `error_code::INSUFFICIENT_CREDITS`, `error_code::VALIDATION`.

## Examples

See the [examples](./examples) directory:

- [check_and_track](./examples/check_and_track.rs) - Verify feature access before doing work, then record usage after the operation succeeds
- [credits](./examples/credits.rs) - Protect expensive operations by requiring an available credit balance
- [subscriptions](./examples/subscriptions.rs) - Manage the full subscription lifecycle with listing, filtering, and inspection
- [plans](./examples/plans.rs) - Create, update, and manage plans and features using the admin API
- [pagination](./examples/pagination.rs) - Query usage events with both page-by-page and collect-all pagination helpers
- [error_handling](./examples/error_handling.rs) - Handle API errors by inspecting status codes, error codes, and retryability
- [admin_operations](./examples/admin_operations.rs) - Manage plans, features, subscriptions, invoices, and credit transactions using the admin API
- [pricing_page](./examples/pricing_page.rs) - Fetch plans for a public pricing page with a single API call
- [idempotency](./examples/idempotency.rs) - Avoid double-counting in distributed systems by using idempotency keys
- [graceful_degradation](./examples/graceful_degradation.rs) - Handle billing errors gracefully and return helpful responses when limits are reached

## Docs

Visit [docs.rollover.dev](https://docs.rollover.dev) for guides and API reference.

## License

[MIT](LICENSE)
