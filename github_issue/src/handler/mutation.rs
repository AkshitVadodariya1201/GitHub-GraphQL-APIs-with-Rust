use crate::config::response::response;
use crate::schema::git_repo::AddLabelsToLabelable;
use crate::schema::git_repo::ClientMutationId;
use crate::schema::git_repo::CreateIssue;
use crate::schema::git_repo::DeleteIssue;
use crate::schema::git_repo::FetchIssue;
use crate::schema::git_repo::Issue;
use crate::schema::git_repo::UpdateIssue;
use async_graphql::{Context, FieldResult, Object};
use serde_json::json;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_issue(&self, input: CreateIssue) -> FieldResult<Issue> {
        let mutation = r#"
            mutation CreateIssue($input: CreateIssueInput!) {
              createIssue(input: $input) {
                issue {
                  id
                  title
                  body
                  state
                  url
                }
              }
            }
        "#;

        let variables = json!({
            "input": {
                "repositoryId": input.repositoryid,
                "title": input.title,
                "body": input.body,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(request_body).await.unwrap();

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let new_issue: Issue =
                serde_json::from_value(response_json["data"]["createIssue"]["issue"].clone())
                    .unwrap();
            Ok(new_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to create issue: {}", error_message).into())
        }
    }

    async fn update_issue(&self, input: UpdateIssue) -> FieldResult<Issue> {
        let mutation = r#"
            mutation UpdateIssue($input: UpdateIssueInput!) {
              updateIssue(input: $input) {
                issue {
                  id
                  title
                  body
                  state
                  url
                }
              }
            }
        "#;

        let variables = json!({
            "input": {
                "id": input.issueid,
                "title": input.newtitle,
                "body": input.newbody,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(request_body).await.unwrap();

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let updated_issue: Issue =
                serde_json::from_value(response_json["data"]["updateIssue"]["issue"].clone())
                    .unwrap();
            Ok(updated_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to update issue: {}", error_message).into())
        }
    }

    async fn delete_issue(
        &self,
        _ctx: &Context<'_>,
        input: DeleteIssue,
    ) -> FieldResult<ClientMutationId> {
        let mutation = json!(format!(
            r#"
            mutation {{
                deleteIssue(input: {{
                    issueId: "{}"
                    }}) {{
                        clientMutationId
                    }}
                }}
            }}
            "#,
            input.issue_id
        ));

        let response = response(mutation).await.unwrap();
        if response.status().is_success() {
            println!("Issue deleted successfully");
            Ok(ClientMutationId {
                client_mutation_id: "deleted".to_string(),
            })
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to delete issue: {}", error_message).into())
        }
    }

    async fn close_issue(&self, input: FetchIssue) -> FieldResult<String> {
        let mutation = r#"
            mutation CloseIssue($input: CloseIssueInput!) {
                closeIssue(input: $input) {
                    clientMutationId
                }
            }
        "#;

        let variables = json!({
            "input" : {
                "issueId" : input.issue_id,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = response(request_body).await.unwrap();

        if response.status().is_success() {
            //let response_json: serde_json::Value = response.json().await?;
            // Handle the response and return the appropriate result
            Ok("Issue closed successfully".to_string())
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to close issue: {}", error_message).into())
        }
    }

    async fn add_label_to_issue(
        &self,
        input: AddLabelsToLabelable,
    ) -> FieldResult<String> {
        let mutation = format!(
            r#"mutation {{
                addLabelsToLabelable(input: {{
                    labelableId: "{}",
                    labelIds: ["{}"]
                    }}) {{
                    labelable {{
                    ... on Issue {{
                        id
                        title
                        labels(first: 5) {{
                        nodes {{
                            name
                        }}
                        }}
                    }}
                    }}
                }}
            }}"#,
            input.issue_id, input.label_ids[0]
        );
        let response = response(serde_json::Value::String(mutation)).await.unwrap();

        if response.status().is_success() {
            let _response_json: serde_json::Value = response.json().await?;
          //  Handle the response and return the appropriate result
            Ok("Add label successfully".to_string())
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to add label in issue: {}", error_message).into())
        }
    }
}
