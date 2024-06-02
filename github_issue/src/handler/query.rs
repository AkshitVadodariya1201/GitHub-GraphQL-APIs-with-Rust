use crate::schema::git_repo::Issue;
use crate::schema::git_repo::IssueQuery;
use crate::{config::response::response, schema::git_repo::Repository};
use async_graphql::{FieldResult, Object};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::env;

const GRAPHQL_ENDPOINT: &str = "https://api.github.com/graphql";

pub struct Query;

#[Object]
impl Query {
    async fn get_issue_id(&self, repository: Repository) -> FieldResult<Issue> {
        dotenv().ok();
        let github_token = env::var("TOKEN_KEY").expect("GitHub token not found in .env file");

        let client = reqwest::Client::new();
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

        let issue_query = IssueQuery {
            query: query.to_owned(),
        };

        let response = client
            .post(GRAPHQL_ENDPOINT)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("User-Agent", "graphql-rust-client")
            .json(&issue_query)
            .send()
            .await?;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
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

    async fn get_repository_issues(&self, owner: String, name: String) -> FieldResult<Vec<Value>> {
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
        let variables = json!({
            "owner": owner,
            "repoName": name,
        });
        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        let response = response(request_body).await.unwrap();
        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            let issues = response_json["data"]["repository"]["issues"]["nodes"]
                .as_array()
                .unwrap();

            Ok(issues.clone())
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }

    async fn get_labels(&self, owner: String, name: String) -> FieldResult<Value> {
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

        let variables = json!({
            "owner": owner,
            "repoName": name,
        });

        let request_body = json!({
            "query": query,
            "variables": variables,
        });

        let response = response(request_body).await.unwrap();

        if response.status().is_success() {
            let response_json: Value = response.json().await?;
            let labels = response_json["data"]["repository"]["labels"]["nodes"]
                .as_array()
                .unwrap();

            Ok(serde_json::Value::Array(labels.clone()))
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }
}
