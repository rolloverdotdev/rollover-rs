use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{ListOptions, Page, Subscription};

fn encode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

impl Rollover {
    /// Returns a paginated list of subscriptions.
    pub async fn list_subscriptions(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<Page<Subscription>, RolloverError> {
        let extra = opts.as_ref().map(build_list_query).unwrap_or_default();
        let extra_refs: Vec<(&str, &str)> = extra.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = self.admin_query(&extra_refs).await?;
        self.get("/v1/subscriptions", &q).await
    }

    /// Returns a single subscription by ID.
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<Subscription, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.get(
            &format!("/v1/subscriptions/{}", encode(subscription_id)),
            &q,
        )
        .await
    }

    /// Creates an admin-initiated subscription for a wallet.
    pub async fn create_subscription(
        &self,
        wallet: &str,
        plan_slug: &str,
    ) -> Result<Subscription, RolloverError> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            wallet_address: &'a str,
            plan_slug: &'a str,
        }

        let q = self.admin_query(&[]).await?;
        let body = Body {
            wallet_address: wallet,
            plan_slug,
        };
        self.post("/v1/subscriptions", &q, &body).await
    }

    /// Cancels a subscription, marking it to expire at the end of the billing period.
    pub async fn cancel_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<Subscription, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.delete(
            &format!("/v1/subscriptions/{}", encode(subscription_id)),
            &q,
        )
        .await
    }
}
