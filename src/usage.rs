use reqwest::header::{HeaderMap, HeaderValue};

use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{CheckResult, ListOptions, Page, TrackOptions, TrackResult, UsageEvent};

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

        if let Some(opts) = opts {
            if !opts.idempotency_key.is_empty() {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Idempotency-Key",
                    HeaderValue::from_str(&opts.idempotency_key)
                        .map_err(|e| RolloverError::Config(format!("invalid idempotency key: {}", e)))?,
                );
                return self.post_with_headers("/v1/track", &[], &body, headers).await;
            }
        }

        self.post("/v1/track", &[], &body).await
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
