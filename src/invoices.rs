use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{Invoice, ListOptions, Page};

impl Rollover {
    /// Returns a paginated list of invoices.
    pub async fn list_invoices(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<Page<Invoice>, RolloverError> {
        let extra = opts.as_ref().map(build_list_query).unwrap_or_default();
        let extra_refs: Vec<(&str, &str)> = extra.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = self.admin_query(&extra_refs).await?;
        self.get("/v1/invoices", &q).await
    }
}
