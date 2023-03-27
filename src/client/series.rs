use super::{OcApi, Result};
use crate::core::series::{AccessControlList, Count, Properties, Series};

impl OcApi {
    pub async fn series(&self) -> Result<Vec<Series>> {
        let url = format!("{}/api/series", self.base_url());
        let response = self.client.get(&url).send().await?;
        let series = response.json::<Vec<Series>>().await?;
        Ok(series)
    }

    pub async fn series_count(&self) -> Result<Count> {
        let url = format!("{}/series/count", self.base_url());
        let response = self.client.get(&url).send().await?;
        let count = response.json::<Count>().await?;
        Ok(count)
    }

    pub async fn series_access_control_list(&self, series_id: &str) -> Result<AccessControlList> {
        let url = format!("{}/series/{series_id}/acl.json", self.base_url());
        let response = self.client.get(&url).send().await?;
        let access_control_list = response.json::<AccessControlList>().await?;
        Ok(access_control_list)
    }

    // TODO implement DublinCore

    pub async fn series_properties(&self, series_id: &str) -> Result<Properties> {
        let url = format!("{}/series/{series_id}/properties.json", self.base_url());
        let response = self.client.get(&url).send().await?;
        let properties = response.json::<Properties>().await?;
        Ok(properties)
    }
}
