use crate::client::Rollover;
use crate::errors::RolloverError;
use crate::types::AnalyticsStats;

impl Rollover {
    /// Returns high-level analytics stats for the organization.
    pub async fn get_analytics(&self) -> Result<AnalyticsStats, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.get("/v1/analytics", &q).await
    }
}
