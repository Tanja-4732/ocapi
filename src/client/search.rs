use super::{OcApi, Result};
use crate::core::search::{EpisodesData, SearchResults};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl OcApi {
    pub async fn search(&self, params: SearchParams) -> Result<EpisodesData> {
        let mut query = Vec::new();
        if let Some(limit) = &params.limit {
            query.push(("limit", limit));
        }
        if let Some(offset) = &params.offset {
            query.push(("offset", offset));
        }
        if let Some(series_uuid) = &params.series_uuid {
            query.push(("sid", series_uuid));
        }

        let url = format!("{}/search/episode.json", self.base_url());
        let response = self.client.get(&url).query(&query).send().await?;

        // println!("{}", response.text().await?);
        // panic!();

        let search = response.json::<EpisodesData>().await?;
        Ok(search)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    // TODO change these types to Option<u64> and Option<Uuid>
    pub limit: Option<String>,
    pub offset: Option<String>,
    pub series_uuid: Option<String>,
}
