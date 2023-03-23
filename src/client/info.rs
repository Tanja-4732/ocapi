use super::{OcApi, Result};
use crate::core::info::{Components, Health};

impl OcApi {
    pub async fn info_health(&self) -> Result<Health> {
        let url = format!("{}/info/health", self.base_url());
        let response = reqwest::get(&url).await?;
        let health = response.json::<Health>().await?;
        Ok(health)
    }

    pub async fn info_components(&self) -> Result<Components> {
        let url = format!("{}/info/components.json", self.base_url());
        let response = reqwest::get(&url).await?;
        let components = response.json::<Components>().await?;
        Ok(components)
    }
}
