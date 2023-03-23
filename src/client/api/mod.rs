pub mod events;

use super::{OcApi, Result};
use crate::core::api::{Api, DefaultVersion, MyRoles, Versions};

impl OcApi {
    pub async fn api(&self) -> Result<Api> {
        let url = format!("{}/api", self.base_url());
        let response = self.client.get(&url).send().await?;
        let api = response.json::<Api>().await?;
        Ok(api)
    }

    pub async fn api_info_me(&self) -> Result<Api> {
        // FIXME this will need cookies somehow

        let url = format!("{}/api/info/me", self.base_url());
        let response = self.client.get(&url).send().await?;
        let api = response.json::<Api>().await?;
        Ok(api)
    }

    pub async fn api_info_me_roles(&self) -> Result<MyRoles> {
        // FIXME this will need cookies somehow

        let url = format!("{}/api/info/me/roles", self.base_url());
        let response = self.client.get(&url).send().await?;
        let my_roles = response.json::<MyRoles>().await?;
        Ok(my_roles)
    }

    pub async fn api_version(&self) -> Result<Versions> {
        let url = format!("{}/api/version", self.base_url());
        let response = self.client.get(&url).send().await?;
        let versions = response.json::<Versions>().await?;
        Ok(versions)
    }

    pub async fn api_version_default(&self) -> Result<DefaultVersion> {
        let url = format!("{}/api/version/default", self.base_url());
        let response = self.client.get(&url).send().await?;
        let default_version = response.json::<DefaultVersion>().await?;
        Ok(default_version)
    }
}
