use async_openai::{config::OpenAIConfig, types::CreateCompletionRequestArgs, Client};

pub struct OpenAIService {
    client: Client<OpenAIConfig>,
}

impl OpenAIService {
    pub fn new() -> Self {
        // Create a OpenAI client with api key from env var OPENAI_API_KEY
        let config = OpenAIConfig::default();
        let client = Client::with_config(config);

        Self { client }
    }

    pub async fn generate_tweet(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateCompletionRequestArgs::default()
            .model("gpt-3.5-turbo-instruct")
            .prompt("Generate a funny sheep-themed tweet that includes 'bah' or 'baah'. Keep it under 280 characters.")
            .max_tokens(40_u32)
            .build()?;

        let response = self.client.completions().create(request).await?;

        let tweet_content = response
            .choices
            .first()
            .map(|choice| choice.text.clone())
            .ok_or("No response generated")?;

        Ok(tweet_content)
    }
}
