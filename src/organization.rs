use crate::client::Rollover;
use crate::errors::RolloverError;
use crate::types::Organization;

impl Rollover {
    /// Returns the organization associated with the API key.
    pub async fn get_organization(&self) -> Result<Organization, RolloverError> {
        self.get("/v1/organization", &[]).await
    }
}
