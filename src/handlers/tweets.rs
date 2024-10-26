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
    let access_token = {
        let token_gaurd = data.access_token.lock().unwrap();
        println!("Current token state: {:?}", token_gaurd.is_some());
        match token_gaurd.as_ref() {
            Some(token) => {
                println!("Found token: {}", token.secret());
                token.clone()
            }
            None => {
                println!("No access token found - user needs to authenticate");
                return format!("Not authenticated. Please authorize first.");
            }
        }
    };

    // Generate tweet
    let openai_service = OpenAIService::new();
    let message = match openai_service.generate_tweet().await {
        Ok(tweet) => tweet,
        Err(e) => {
            println!("Failed to generate tweet: {}", e);
            return format!("Failed to generate tweet: {}", e);
        }
    };

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
        .await
        .unwrap();

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        println!("Tweet posted successfully!");
        println!("Response: {}", body);
        format!("Posted tweet: {}", message)
    } else {
        println!("Failed to post tweet - Status: {}", status);
        println!("Error response: {}", body);
        format!("Failed to post tweet: {} - {}", status, body)
    }
}
