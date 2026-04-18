use serde::{Deserialize, Serialize};

/// Result of a feature usage check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    #[serde(default)]
    pub allowed: bool,
    #[serde(default)]
    pub used: i64,
    #[serde(default)]
    pub remaining: i64,
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub plan: String,
    #[serde(default)]
    pub credit_balance: i64,
    #[serde(default)]
    pub credit_cost: i64,
}

/// Result of tracking a usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackResult {
    #[serde(default)]
    pub allowed: bool,
    #[serde(default)]
    pub used: i64,
    #[serde(default)]
    pub remaining: i64,
    #[serde(default)]
    pub credit_balance: i64,
}

/// Credit balance for a wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBalance {
    pub wallet: String,
    pub balance: i64,
}

/// Result of granting credits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantResult {
    pub balance: i64,
    pub granted: i64,
}

/// A billing plan whose pricing is hydrated from its latest revision; editing pricing on the
/// server creates a new revision and `latest_revision_id` advances accordingly so existing
/// subscribers stay pinned to the price they signed up on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub price_usdc: String,
    #[serde(default)]
    pub setup_fee_usdc: String,
    pub billing_period: String,
    #[serde(default)]
    pub trial_days: i32,
    #[serde(default)]
    pub auto_assign: bool,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub latest_revision_id: String,
    #[serde(default)]
    pub sort_order: i32,
    #[serde(default)]
    pub subscribers: i64,
    #[serde(default)]
    pub features: Vec<Feature>,
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub last_subscribed_at: String,
}

/// A metered feature on a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub feature_slug: String,
    pub name: String,
    #[serde(default)]
    pub limit_amount: i64,
    #[serde(default)]
    pub reset_period: String,
    #[serde(default)]
    pub credit_cost: i64,
    #[serde(default)]
    pub overage_price: String,
    #[serde(default)]
    pub weight: String,
}

/// A wallet's subscription to a plan, pinned to a specific pricing revision via
/// `plan_revision_id` so renewals charge the original price even after the plan is edited.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub wallet_address: String,
    pub plan_id: String,
    #[serde(default)]
    pub plan_revision_id: String,
    #[serde(default)]
    pub plan_name: String,
    pub status: String,
    #[serde(default)]
    pub billing_period: String,
    pub mode: String,
    #[serde(default)]
    pub period_start: String,
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub trial_end: String,
    #[serde(default)]
    pub cancel_at_end: bool,
    #[serde(default)]
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// A single usage tracking event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEvent {
    pub id: String,
    pub wallet_address: String,
    pub feature_slug: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub subscription_id: String,
    #[serde(default)]
    pub recorded_at: String,
}

/// The organization associated with the API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub logo: String,
    #[serde(default)]
    pub webhook_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// A paginated list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// High-level analytics stats for an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsStats {
    #[serde(default)]
    pub mrr: String,
    #[serde(default)]
    pub active_subs: i64,
    #[serde(default)]
    pub total_revenue: String,
    #[serde(default)]
    pub top_features: Vec<TopFeature>,
    #[serde(default)]
    pub recent_activity: Vec<RecentEvent>,
}

/// A feature ranked by total usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopFeature {
    pub feature_slug: String,
    #[serde(default)]
    pub total_used: i64,
}

/// A recent usage event in the activity feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentEvent {
    pub wallet_address: String,
    pub feature_slug: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub recorded_at: String,
}

/// A single credit ledger entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTransaction {
    pub id: String,
    pub wallet_address: String,
    #[serde(default)]
    pub amount: i64,
    #[serde(rename = "type", default)]
    pub type_: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub subscription_id: String,
    #[serde(default)]
    pub created_at: String,
}

/// A billing invoice; `chain_id` and `mode` identify which chain it settled on and which
/// environment it belongs to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub wallet_address: String,
    #[serde(default)]
    pub subscription_id: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub chain_id: String,
    pub status: String,
    #[serde(default)]
    pub base_amount: String,
    #[serde(default)]
    pub overage_amount: String,
    #[serde(default)]
    pub total_amount: String,
    #[serde(default)]
    pub tx_hash: String,
    #[serde(default)]
    pub period_start: String,
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub settled_at: String,
    #[serde(default)]
    pub created_at: String,
}

/// A payment destination chain configured on an organization for a given mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub id: String,
    #[serde(default)]
    pub org_id: String,
    pub mode: String,
    pub chain_id: String,
    pub pay_to_address: String,
    #[serde(default)]
    pub stablecoin_symbol: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Parameters for adding a chain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChainParams {
    pub chain_id: String,
    pub pay_to_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stablecoin_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Parameters for editing a chain; only non-None fields are sent so the rest stay at their
/// current values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChainParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_to_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stablecoin_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Parameters for creating a plan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePlanParams {
    pub slug: String,
    pub name: String,
    pub price_usdc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee_usdc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

/// Parameters for updating a plan. Only non-None fields are sent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_usdc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee_usdc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

/// Parameters for creating a feature on a plan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFeatureParams {
    pub feature_slug: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

/// Parameters for updating a feature. Only non-None fields are sent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFeatureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

/// Options for paginated list methods.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub limit: i64,
    pub offset: i64,
    pub wallet: String,
    pub status: String,
    pub plan_id: String,
    pub feature: String,
    pub after: String,
    pub before: String,
}

/// Options for the track method.
#[derive(Debug, Clone, Default)]
pub struct TrackOptions {
    pub idempotency_key: String,
}

/// Options for granting credits.
#[derive(Debug, Clone, Default)]
pub struct GrantOptions {
    pub description: String,
    pub expires_at: String,
}
