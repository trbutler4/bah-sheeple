use reqwest::Client;
use serde_json::json;

pub struct TwitterService {
    client: Client,
}

impl TwitterService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn post_tweet(
        &self,
        message: &str,
        access_token: &str,
    ) -> Result<String, reqwest::Error> {
        let twitter_api_url = "https://api.twitter.com/2/tweets";
        let tweet_data = json!({
            "text": message
        });

        let response = self
            .client
            .post(twitter_api_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .body(tweet_data.to_string())
            .send()
            .await?;

        response.text().await
    }
}
