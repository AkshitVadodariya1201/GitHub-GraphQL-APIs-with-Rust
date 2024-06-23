use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

// Define the Repository struct
#[derive(InputObject)]
pub struct Repository {
    pub owner: String,
    pub reponame: String,
    pub issuenumber: String,
}

// Define the RepositoryDetail struct
#[derive(InputObject)]
pub struct RepositoryDetail {
    pub owner: String,
    pub reponame: String,
    pub last: usize,
}

// Define the IssueQuery struct
#[derive(Serialize)]
pub struct IssueQuery {
    pub query: String,
}

// Define the CreateIssue struct
#[derive(InputObject)]
pub struct CreateIssue {
    pub title: String,
    pub repositoryid: String,
    pub body: String,
}

// Define the UpdateIssue struct
#[derive(InputObject)]
pub struct UpdateIssue {
    pub newtitle: Option<String>,
    pub issueid: String,
    pub newbody: Option<String>,
}

// Define the DeleteIssue struct
#[derive(Serialize, SimpleObject, Deserialize)]
pub struct ClientMutationId {
    pub client_mutation_id: String,
}

// Define the DeleteIssue struct
#[derive(InputObject)]
pub struct DeleteIssue {
    pub issue_id: String,
}

// Define the FetchIssue struct
#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct Issue {
    pub id: Option<String>,
    pub title: Option<String>,
    pub state: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
}

// Define the IssueNode struct
#[derive(Debug, Deserialize)]
pub struct IssueNode {
    pub nodes: Vec<Issue>,
}

// Define the RepositoryIssue struct
#[derive(Debug, Deserialize)]
pub struct RepositoryIssue {
    pub issues: IssueNode,
}

// Define the GraphQLQuery struct
#[derive(Debug, Serialize)]
pub struct GraphQLQuery {
    pub query: String,
}

// Define the FetchIssue struct
#[derive(InputObject)]
pub struct FetchIssue {
    pub issue_id: String,
}

// Define the AddLabelsToLabelable struct
#[derive(InputObject, Serialize, Debug)]
pub struct AddLabelsToLabelable {
    pub issue_id: String,
    pub label_ids: Vec<String>,
}
