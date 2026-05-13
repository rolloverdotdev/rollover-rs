use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{
    Atomicity, BatchCheckItem, BatchCheckResult, BatchTrackEvent, BatchTrackResult, CheckResult,
    ListOptions, Page, TrackOptions, TrackResult, UsageEvent,
};

impl Rollover {
    /// Returns whether a wallet is allowed to use a feature.
    pub async fn check(&self, wallet: &str, feature: &str) -> Result<CheckResult, RolloverError> {
        let q = vec![
            ("wallet".to_string(), wallet.to_string()),
            ("feature".to_string(), feature.to_string()),
        ];
        self.get("/v1/check", &q).await
    }

    /// Records a usage event for the given wallet and feature.
    pub async fn track(
        &self,
        wallet: &str,
        feature: &str,
        amount: i64,
        opts: Option<&TrackOptions>,
    ) -> Result<TrackResult, RolloverError> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            wallet: &'a str,
            feature: &'a str,
            amount: i64,
        }

        let body = Body {
            wallet,
            feature,
            amount,
        };

        let key = opts
            .and_then(|o| (!o.idempotency_key.is_empty()).then(|| o.idempotency_key.clone()))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let mut headers = HeaderMap::new();
        headers.insert(
            "Idempotency-Key",
            HeaderValue::from_str(&key)
                .map_err(|e| RolloverError::Config(format!("invalid idempotency key: {}", e)))?,
        );
        self.post_with_headers("/v1/track", &[], &body, headers).await
    }

    /// Checks multiple features in one call, optionally preflighting per-entry `amount` and returning a `credit_summary` when the batch touches credit features.
    pub async fn check_batch(
        &self,
        wallet: &str,
        features: &[BatchCheckItem],
    ) -> Result<BatchCheckResult, RolloverError> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            wallet: &'a str,
            features: &'a [BatchCheckItem],
        }
        self.post("/v1/check/batch", &[], &Body { wallet, features }).await
    }

    /// Records every event in one call, tagging each `usage_events` row with the returned `batch_id` and using `atomicity` to decide whether a per-event failure rolls back the whole batch.
    pub async fn track_batch(
        &self,
        wallet: &str,
        events: &[BatchTrackEvent],
        atomicity: Atomicity,
        opts: Option<&TrackOptions>,
    ) -> Result<BatchTrackResult, RolloverError> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            wallet: &'a str,
            events: &'a [BatchTrackEvent],
            atomicity: Atomicity,
        }
        let body = Body { wallet, events, atomicity };
        let key = opts
            .and_then(|o| (!o.idempotency_key.is_empty()).then(|| o.idempotency_key.clone()))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let mut headers = HeaderMap::new();
        headers.insert(
            "Idempotency-Key",
            HeaderValue::from_str(&key)
                .map_err(|e| RolloverError::Config(format!("invalid idempotency key: {}", e)))?,
        );
        self.post_with_headers("/v1/track/batch", &[], &body, headers).await
    }

    /// Returns a paginated list of usage events.
    pub async fn list_usage(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<Page<UsageEvent>, RolloverError> {
        let extra = opts.as_ref().map(build_list_query).unwrap_or_default();
        let extra_refs: Vec<(&str, &str)> = extra.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = self.admin_query(&extra_refs).await?;
        self.get("/v1/usage", &q).await
    }
}
