//! Functions for invoking the API endpoints, returning the data types defined in the [`crate::core`] module.

pub mod api;
pub mod info;
pub mod search;
pub mod series;

use anyhow::Result;
use reqwest::{cookie::Jar, Url};

/// A struct for invoking the API endpoints, holds the base URL
#[derive(Default, Debug, Clone)]
pub struct OcApi {
    base_url: String,
    client: reqwest::Client,
}

impl OcApi {
    pub fn new(base_url: String, token: AuthToken) -> Self {
        Self {
            client: make_reqwest_client(token, &base_url).unwrap(),
            base_url,
        }
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_ref()
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

#[derive(Clone)]
pub struct AuthToken {
    name: String,
    token: String,
}

impl AuthToken {
    pub fn new(name: String, token: String) -> Self {
        Self { name, token }
    }
}

fn make_reqwest_client(
    AuthToken { name, token }: AuthToken,
    base_url: &str,
) -> Result<reqwest::Client> {
    let url = Url::parse(base_url)?;

    let jar = Jar::default();
    jar.add_cookie_str(&format!("{name}={token}"), &url);

    Ok(reqwest::Client::builder()
        .cookie_provider(jar.into())
        .build()?)
}
