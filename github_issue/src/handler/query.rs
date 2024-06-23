use crate::schema::git_repo::Issue;
use crate::schema::git_repo::IssueQuery;
use crate::{config::response::response, schema::git_repo::Repository};
use async_graphql::{FieldResult, Object};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::env;

// Define the GitHub GraphQL API endpoint
const GRAPHQL_ENDPOINT: &str = "https://api.github.com/graphql";

// Define the Query struct
pub struct Query;

// Implement the Query struct
#[Object]
impl Query {
    // Define the get_issue_id query
    async fn get_issue_id(&self, repository: Repository) -> FieldResult<Issue> {
        // Load the environment variables
        dotenv().ok();

        // Get the GitHub token from the environment variables
        let github_token = env::var("TOKEN_KEY").expect("GitHub token not found in .env file");

        // Send a POST request to the GitHub GraphQL API
        let client = reqwest::Client::new();

        // Define the query
        let query = format!(
            r#"
                query {{
                    repository(owner: "{}", name: "{}") {{
                        issue(number: {}) {{
                            id
                            body
                            title
                            state
                            url
                        }}
                    }}
                }}
            "#,
            repository.owner, repository.reponame, repository.issuenumber
        );

        // Define the issue query
        let issue_query = IssueQuery {
            query: query.to_owned(),
        };

        // Send the request to the GitHub GraphQL API
        let response = client
            .post(GRAPHQL_ENDPOINT)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("User-Agent", "graphql-rust-client")
            .json(&issue_query)
            .send()
            .await?;

        // Handle the response from the GitHub GraphQL API
        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;

            // Extract the issue ID from the response
            let issue_id = response_json["data"]["repository"]["issue"]["id"]
                .as_str()
                .unwrap_or("Unknown");

            println!("Issue ID: {}", issue_id);
            let new_issue: Issue =
                serde_json::from_value(response_json["data"]["repository"]["issue"].clone())?;
            Ok(new_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to get issue: {}", error_message).into())
        }
    }

    // Define the get_repository_issues query
    async fn get_repository_issues(&self, owner: String, name: String) -> FieldResult<Vec<Value>> {
        // Load the environment variables
        let query = r#"
        query GetIssues($owner: String!, $repoName: String!) {
            repository(owner: $owner, name: $repoName) {
                issues(last: 10) {
                    nodes {
                        id
                        title
                        body
                        state
                        createdAt
                        updatedAt
                        closedAt
                        url
                    }
                }
            }
        }"#;

        // Define the variables for the get_repository_issues query
        let variables = json!({
            "owner": owner,
            "repoName": name,
        });

        // Define the request body
        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        // Send the request to the GitHub GraphQL API
        let response = response(request_body).await.unwrap();

        // Handle the response from the GitHub GraphQL API
        if response.status().is_success() {
            let response_json: Value = response.json().await?;

            // Extract the issues from the response
            let issues = response_json["data"]["repository"]["issues"]["nodes"]
                .as_array()
                .unwrap();

            Ok(issues.clone())
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }

    // Define the get_labels query
    async fn get_labels(&self, owner: String, name: String) -> FieldResult<Value> {
        // Define the query for the get_labels query
        let query = r#"
            query GetLabels($owner: String!, $repoName: String!) {
                repository(owner: $owner, name: $repoName) {
                    labels(first: 20) {
                        nodes {
                            id
                            name
                            description
                        }
                    }
                }
            }
        "#;

        // Define the variables for the get_labels query
        let variables = json!({
            "owner": owner,
            "repoName": name,
        });

        // Define the request body
        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        // Send the request to the GitHub GraphQL API
        let response = response(request_body).await.unwrap();

        // Handle the response from the GitHub GraphQL API
        if response.status().is_success() {
            let response_json: Value = response.json().await?;

            // Extract the labels from the response
            let labels = response_json["data"]["repository"]["labels"]["nodes"]
                .as_array()
                .unwrap();

            Ok(serde_json::Value::Array(labels.clone()))
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }
}
