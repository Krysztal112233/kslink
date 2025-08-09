use std::collections::HashMap;

use dioxus_logger::tracing::error;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::{
    common,
    error::{Error, Result},
};

#[derive(Debug)]
pub struct Requester {
    client: Client,
}

#[derive(Debug, Default, Deserialize)]
pub struct Statistics {
    pub count: u64,
    pub visit: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct MakeHash {
    pub hash: String,
    pub trimmed: HashMap<String, String>,
}

#[allow(unused)]
impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("kslink/frontend")
                .build()
                .unwrap(),
        }
    }

    pub async fn create(&self, from: Url) -> Result<MakeHash> {
        let res = self
            .client
            .post(common::BASE_URL)
            .query(&[("url", from.as_str())])
            .send()
            .await
            .inspect_err(|err| error!("{err}"))?;

        if res.status().is_success() {
            Ok(res.json().await.inspect_err(|err| error!("{err}"))?)
        } else {
            Err(Error::Unknown(format!(
                "server status {}",
                res.status().as_u16()
            )))
        }
    }

    pub async fn statistics(&self) -> Result<Statistics> {
        let url = Url::parse(common::BASE_URL)
            .unwrap()
            .join("statistics")
            .unwrap();

        let res = self
            .client
            .get(url)
            .send()
            .await
            .inspect_err(|err| error!("{err}"))?;

        if res.status().is_success() {
            Ok(res
                .json::<Statistics>()
                .await
                .inspect_err(|err| error!("{err}"))?)
        } else {
            Err(Error::Unknown(format!(
                "server status {}",
                res.status().as_u16()
            )))
        }
    }
}
