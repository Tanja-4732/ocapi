use crate::core::api::events::Event;

use super::{OcApi, Result};

impl OcApi {
    pub async fn api_events(&self) -> Result<Vec<Event>> {
        let url = format!("{}/api/events", self.base_url());
        let response = reqwest::get(&url).await?;
        let events = response.json::<Vec<Event>>().await?;
        Ok(events)
    }
}
