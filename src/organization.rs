use crate::client::Rollover;
use crate::errors::RolloverError;
use crate::types::{Chain, CreateChainParams, Organization, UpdateChainParams};

fn encode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

impl Rollover {
    /// Returns the organization associated with the API key.
    pub async fn get_organization(&self) -> Result<Organization, RolloverError> {
        self.get("/v1/organization", &[]).await
    }

    /// Lists every payment chain configured for the API key's org and mode, including
    /// disabled ones, ordered by priority so the first enabled chain is the one subscribers
    /// settle to.
    pub async fn list_chains(&self) -> Result<Vec<Chain>, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.get("/v1/organization/chains", &q).await
    }

    /// Adds a new payment destination chain. Use this when accepting payments on additional
    /// networks or when configuring your live mode payout address before issuing live API
    /// keys; the server returns 400 `unsupported_chain` for chains outside the catalog and
    /// 400 `mode_mismatch` if a testnet is added to live mode or vice versa.
    pub async fn create_chain(
        &self,
        params: &CreateChainParams,
    ) -> Result<Chain, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.post("/v1/organization/chains", &q, params).await
    }

    /// Edits a chain's address, stablecoin, enabled flag, or priority, sending only the
    /// fields set on `params` so the rest stay at their current values.
    pub async fn update_chain(
        &self,
        chain_id: &str,
        params: &UpdateChainParams,
    ) -> Result<Chain, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.put(
            &format!("/v1/organization/chains/{}", encode(chain_id)),
            &q,
            params,
        )
        .await
    }

    /// Removes a chain so subscribers can no longer pay on it; if this was the only enabled
    /// chain, paid flows fail with `no_chain_configured` until another is added.
    pub async fn delete_chain(&self, chain_id: &str) -> Result<(), RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.delete_empty(
            &format!("/v1/organization/chains/{}", encode(chain_id)),
            &q,
        )
        .await
    }
}
