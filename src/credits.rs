use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{CreditBalance, CreditTransaction, GrantOptions, GrantResult, ListOptions, Page};

impl Rollover {
    /// Returns the current credit balance for the given wallet.
    pub async fn get_credits(&self, wallet: &str) -> Result<CreditBalance, RolloverError> {
        let q = vec![("wallet".to_string(), wallet.to_string())];
        self.get("/v1/credits", &q).await
    }

    /// Adds credits to a wallet with an optional description and expiration.
    pub async fn grant_credits(
        &self,
        wallet: &str,
        amount: i64,
        opts: Option<&GrantOptions>,
    ) -> Result<GrantResult, RolloverError> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            wallet: &'a str,
            amount: i64,
            #[serde(skip_serializing_if = "str::is_empty")]
            description: &'a str,
            #[serde(skip_serializing_if = "str::is_empty")]
            expires_at: &'a str,
        }

        let (desc, exp) = opts
            .map(|o| (o.description.as_str(), o.expires_at.as_str()))
            .unwrap_or(("", ""));

        let body = Body {
            wallet,
            amount,
            description: desc,
            expires_at: exp,
        };

        self.post("/v1/credits", &[], &body).await
    }

    /// Returns a paginated list of credit ledger entries.
    pub async fn list_credit_transactions(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<Page<CreditTransaction>, RolloverError> {
        let extra = opts.as_ref().map(build_list_query).unwrap_or_default();
        let extra_refs: Vec<(&str, &str)> = extra.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = self.admin_query(&extra_refs).await?;
        self.get("/v1/credits/transactions", &q).await
    }
}
