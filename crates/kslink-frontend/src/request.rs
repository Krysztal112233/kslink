use reqwest::Client;
use url::Url;

use crate::common;

#[derive(Debug)]
pub struct Requester {
    client: Client,
}

impl Requester {
    async fn create(&self, from: Url) {
        let url = common::BASE_URL;

        let result = self.client.post(url).query(&("url", from)).send().await;
    }
}
