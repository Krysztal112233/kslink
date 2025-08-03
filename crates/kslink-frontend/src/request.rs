use dioxus_logger::tracing;
use reqwest::Client;
use serde_json::Value;
use url::Url;

use crate::{common, error};

#[derive(Debug)]
pub struct Requester {
    client: Client,
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

    pub async fn create(&self, from: Url) -> error::Result<String> {
        let url = common::BASE_URL;
        let res = self
            .client
            .post(url)
            .query(&("url", from))
            .send()
            .await
            .inspect_err(|err| tracing::error!("{err}"))?;

        if res.status().is_success() {
            let res = res
                .json::<Value>()
                .await
                .inspect_err(|err| tracing::error!("{err}"))?;

            Ok(res["hash"]
                .as_str()
                .ok_or(error::Error::Unknown("unknow format".to_string()))?
                .to_string())
        } else {
            Err(error::Error::Unknown(format!(
                "server status {}",
                res.status().as_u16()
            )))
        }
    }

    pub async fn statistics(&self) -> error::Result<()> {
        todo!()
    }
}
