//! Rust SDK for the Rollover subscription billing API.
//!
//! Rollover is a subscription billing platform built on x402 that manages
//! plans, usage, credits, and recurring billing, settling in USDC on-chain.
//!
//! # Usage
//!
//! ```no_run
//! use rollover::Rollover;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ro = Rollover::new("ro_test_...")?;
//!
//!     let result = ro.check("0xabc...", "api-calls").await?;
//!     if result.allowed {
//!         ro.track("0xabc...", "api-calls", 1, None).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```

mod analytics;
mod client;
mod credits;
mod errors;
mod invoices;
mod organization;
mod pagination;
mod plans;
mod subscriptions;
mod types;
mod usage;

pub use client::{Rollover, RolloverBuilder};
pub use errors::{error_code, is_error_code, RolloverError};
pub use pagination::{collect_all, pages, Iter};
pub use types::{
    AnalyticsStats, Atomicity, BatchCheckEntry, BatchCheckItem, BatchCheckResult, BatchTrackEntry,
    BatchTrackEvent, BatchTrackResult, Chain, CheckResult, CreateChainParams, CreatePlanParams,
    CreditBalance, CreditSummary, CreditTransaction, Feature, FeatureType, GrantOptions,
    GrantResult, Invoice, LinkFeatureParams, ListOptions, Organization, Page, Plan, PlanFeature,
    Policy, RecentEvent, Subscription, TopFeature, TrackOptions, TrackResult, UpdateChainParams,
    UpdatePlanFeatureParams, UpdatePlanParams, UsageEvent,
};
