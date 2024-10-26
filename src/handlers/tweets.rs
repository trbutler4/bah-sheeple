use actix_web::{get, post, web, Responder};

use crate::services::openai::OpenAIService;
use crate::AppState;

#[get("/bah")]
pub async fn bah() -> impl Responder {
    let mut result = String::from("ba");
    let num_h = rand::random::<u8>() % 15 + 4;
    result.extend(std::iter::repeat('h').take(num_h as usize));
    format!("{}", result)
}

#[post("/tweet")]
pub async fn post_tweet(data: web::Data<AppState>) -> impl Responder {
    match generate_and_post_tweet(data).await {
        Ok(result) => result,
        Err(e) => format!("Error: {}", e),
    }
}

pub async fn generate_and_post_tweet(
    data: web::Data<AppState>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Extract the existing tweet posting logic from post_tweet handler
    let access_token = {
        let token_guard = data.access_token.lock().unwrap();
        match token_guard.as_ref() {
            Some(token) => token.clone(),
            None => return Err("Not authenticated".into()),
        }
    };

    let openai_service = OpenAIService::new();
    let message = openai_service.generate_tweet().await?;

    // Post to Twitter
    let twitter_api_url = "https://api.twitter.com/2/tweets";
    let tweet_data = serde_json::json!({
        "text": message
    });

    let client = reqwest::Client::new();
    let response = client
        .post(twitter_api_url)
        .header("Authorization", format!("Bearer {}", access_token.secret()))
        .header("Content-Type", "application/json")
        .body(tweet_data.to_string())
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        Ok(format!("Posted tweet: {}", message))
    } else {
        Err(format!("Failed to post tweet: {} - {}", status, body).into())
    }
}
