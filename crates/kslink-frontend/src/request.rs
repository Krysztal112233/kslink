use reqwest::Client;
use url::Url;

#[derive(Debug)]
pub struct Requester {
    base_url: Url,
    client: Client,
}

impl Requester {
    async fn create(&self, from: Url) {
        let result = self
            .client
            .post(self.base_url.clone())
            .query(&("url", from))
            .send()
            .await;
    }
}
