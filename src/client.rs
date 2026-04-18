use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::errors::{parse_error, RolloverError};
use crate::types::Organization;

const DEFAULT_BASE_URL: &str = "https://api.rollover.dev";

/// The Rollover API client.
pub struct Rollover {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) mode: String,
    slug: Mutex<Option<String>>,
}

/// Builder for constructing a Rollover client.
pub struct RolloverBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    http_client: Option<reqwest::Client>,
}

impl RolloverBuilder {
    /// Sets the API key.
    pub fn api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }

    /// Overrides the default API base URL.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    /// Sets a custom HTTP client.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Builds the Rollover client.
    pub fn build(self) -> Result<Rollover, RolloverError> {
        let api_key = self
            .api_key
            .or_else(|| std::env::var("ROLLOVER_API_KEY").ok())
            .unwrap_or_default();

        let mode = if api_key.starts_with("ro_test_") {
            "test"
        } else {
            "live"
        };

        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        let client = self.http_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to build HTTP client")
        });

        Ok(Rollover {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            mode: mode.to_string(),
            slug: Mutex::new(None),
        })
    }
}

impl Rollover {
    /// Creates a new client with the given API key.
    pub fn new(api_key: &str) -> Result<Rollover, RolloverError> {
        Rollover::builder().api_key(api_key).build()
    }

    /// Creates a new client from the ROLLOVER_API_KEY environment variable.
    pub fn from_env() -> Result<Rollover, RolloverError> {
        Rollover::builder().build()
    }

    /// Returns a new builder for configuring a client.
    pub fn builder() -> RolloverBuilder {
        RolloverBuilder {
            api_key: None,
            base_url: None,
            http_client: None,
        }
    }

    /// Returns the mode (test or live) detected from the API key.
    pub fn mode(&self) -> &str {
        &self.mode
    }

    pub(crate) async fn resolve_slug(&self) -> Result<String, RolloverError> {
        let mut slug = self.slug.lock().await;
        if let Some(ref s) = *slug {
            return Ok(s.clone());
        }

        let org: Organization = self.get("/v1/organization", &[]).await?;
        *slug = Some(org.slug.clone());
        Ok(org.slug)
    }

    pub(crate) async fn admin_query(
        &self,
        extra: &[(&str, &str)],
    ) -> Result<Vec<(String, String)>, RolloverError> {
        let slug = self.resolve_slug().await?;
        let mut q = vec![
            ("slug".to_string(), slug),
            ("mode".to_string(), self.mode.clone()),
        ];
        for (k, v) in extra {
            if !v.is_empty() {
                q.push((k.to_string(), v.to_string()));
            }
        }
        Ok(q)
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<T, RolloverError> {
        self.do_request(reqwest::Method::GET, path, query, None::<&()>, None)
            .await
    }

    pub(crate) async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &B,
    ) -> Result<T, RolloverError> {
        self.do_request(reqwest::Method::POST, path, query, Some(body), None)
            .await
    }

    pub(crate) async fn post_with_headers<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &B,
        headers: HeaderMap,
    ) -> Result<T, RolloverError> {
        self.do_request(reqwest::Method::POST, path, query, Some(body), Some(headers))
            .await
    }

    pub(crate) async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        body: &B,
    ) -> Result<T, RolloverError> {
        self.do_request(reqwest::Method::PUT, path, query, Some(body), None)
            .await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<T, RolloverError> {
        self.do_request(reqwest::Method::DELETE, path, query, None::<&()>, None)
            .await
    }

    pub(crate) async fn delete_empty(
        &self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<(), RolloverError> {
        let resp = self
            .do_request_raw(reqwest::Method::DELETE, path, query, None::<&()>, None)
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let body = resp.bytes().await.unwrap_or_default();
            Err(parse_error(status, &body))
        }
    }

    async fn do_request<B: Serialize, T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&B>,
        headers: Option<HeaderMap>,
    ) -> Result<T, RolloverError> {
        let resp = self
            .do_request_raw(method, path, query, body, headers)
            .await?;

        let status = resp.status().as_u16();
        let resp_body = resp.bytes().await?;

        if status < 200 || status >= 300 {
            return Err(parse_error(status, &resp_body));
        }

        serde_json::from_slice(&resp_body).map_err(|e| RolloverError::Config(format!("parsing response: {}", e)))
    }

    async fn do_request_raw<B: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&B>,
        headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response, RolloverError> {
        let url = format!("{}{}", self.base_url, path);

        let mut req = self.client.request(method, &url);

        if !query.is_empty() {
            req = req.query(query);
        }

        req = req.header("X-API-Key", &self.api_key);

        if let Some(body) = body {
            let data = serde_json::to_vec(body)
                .map_err(|e| RolloverError::Config(format!("marshaling request body: {}", e)))?;
            req = req
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .body(data);
        }

        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        Ok(req.send().await?)
    }
}

pub(crate) fn build_list_query(opts: &crate::types::ListOptions) -> Vec<(&str, String)> {
    let mut q = Vec::new();
    if opts.limit > 0 {
        q.push(("limit", opts.limit.to_string()));
    }
    if opts.offset > 0 {
        q.push(("offset", opts.offset.to_string()));
    }
    if !opts.wallet.is_empty() {
        q.push(("wallet", opts.wallet.clone()));
    }
    if !opts.status.is_empty() {
        q.push(("status", opts.status.clone()));
    }
    if !opts.plan_id.is_empty() {
        q.push(("plan_id", opts.plan_id.clone()));
    }
    if !opts.feature.is_empty() {
        q.push(("feature", opts.feature.clone()));
    }
    if !opts.after.is_empty() {
        q.push(("after", opts.after.clone()));
    }
    if !opts.before.is_empty() {
        q.push(("before", opts.before.clone()));
    }
    q
}

