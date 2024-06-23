use async_graphql::Error;
use dotenv::dotenv;
use reqwest::Response;
use serde_json::Value;
use std::env;

const GRAPHQL_ENDPOINT: &str = "https://api.github.com/graphql";

// Send a request to the GitHub GraphQL API
pub async fn response(json_value: Value) -> Result<Response, Error> {
    // Load the environment variables
    dotenv().ok();
    // Get the GitHub token from the environment variables
    let token = env::var("TOKEN_KEY").expect("GITHUB_TOKEN environment variable not set");

    // Send a POST request to the GitHub GraphQL API
    let response = reqwest::Client::new()
        .post(GRAPHQL_ENDPOINT)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "graphql-rust-client")
        .json(&json_value)
        .send()
        .await?;
    Ok(response)
}
