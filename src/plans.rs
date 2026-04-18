use crate::client::{build_list_query, Rollover};
use crate::errors::RolloverError;
use crate::types::{
    CreateFeatureParams, CreatePlanParams, Feature, ListOptions, Page, Plan, UpdateFeatureParams,
    UpdatePlanParams,
};

fn encode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

impl Rollover {
    /// Returns a paginated list of plans for the organization.
    pub async fn list_plans(
        &self,
        opts: Option<ListOptions>,
    ) -> Result<Page<Plan>, RolloverError> {
        let extra = opts.as_ref().map(build_list_query).unwrap_or_default();
        let extra_refs: Vec<(&str, &str)> = extra.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = self.admin_query(&extra_refs).await?;
        self.get("/v1/plans", &q).await
    }

    /// Returns a single plan by slug, including its features.
    pub async fn get_plan(&self, plan_slug: &str) -> Result<Plan, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.get(&format!("/v1/plans/{}", encode(plan_slug)), &q)
            .await
    }

    /// Creates a new plan.
    pub async fn create_plan(&self, params: &CreatePlanParams) -> Result<Plan, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.post("/v1/plans", &q, params).await
    }

    /// Updates an existing plan's metadata.
    pub async fn update_plan(
        &self,
        plan_slug: &str,
        params: &UpdatePlanParams,
    ) -> Result<Plan, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.put(&format!("/v1/plans/{}", encode(plan_slug)), &q, params)
            .await
    }

    /// Archives a plan by slug, hiding it from new subscribers while existing subscribers
    /// keep their current subscription on the revision they signed up on.
    pub async fn archive_plan(&self, plan_slug: &str) -> Result<(), RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.delete_empty(&format!("/v1/plans/{}", encode(plan_slug)), &q)
            .await
    }

    /// Hard deletes a plan and all of its revisions; the server returns 409
    /// `plan_has_subscriptions` when any subscription past or present references the plan, so
    /// reach for [`Self::archive_plan`] whenever the plan has ever had a subscriber.
    pub async fn delete_plan(&self, plan_slug: &str) -> Result<(), RolloverError> {
        let q = self.admin_query(&[("hard", "true")]).await?;
        self.delete_empty(&format!("/v1/plans/{}", encode(plan_slug)), &q)
            .await
    }

    /// Adds a feature to a plan.
    pub async fn create_feature(
        &self,
        plan_slug: &str,
        params: &CreateFeatureParams,
    ) -> Result<Feature, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.post(
            &format!("/v1/plans/{}/features", encode(plan_slug)),
            &q,
            params,
        )
        .await
    }

    /// Updates an existing feature on a plan.
    pub async fn update_feature(
        &self,
        plan_slug: &str,
        feature_slug: &str,
        params: &UpdateFeatureParams,
    ) -> Result<Feature, RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.put(
            &format!(
                "/v1/plans/{}/features/{}",
                encode(plan_slug),
                encode(feature_slug)
            ),
            &q,
            params,
        )
        .await
    }

    /// Removes a feature from a plan.
    pub async fn delete_feature(
        &self,
        plan_slug: &str,
        feature_slug: &str,
    ) -> Result<(), RolloverError> {
        let q = self.admin_query(&[]).await?;
        self.delete_empty(
            &format!(
                "/v1/plans/{}/features/{}",
                encode(plan_slug),
                encode(feature_slug)
            ),
            &q,
        )
        .await
    }

    /// Returns the active plans for a given org slug (public, no auth required).
    pub async fn list_pricing(&self, org_slug: &str) -> Result<Vec<Plan>, RolloverError> {
        self.get(&format!("/v1/pricing/{}", encode(org_slug)), &[])
            .await
    }
}
