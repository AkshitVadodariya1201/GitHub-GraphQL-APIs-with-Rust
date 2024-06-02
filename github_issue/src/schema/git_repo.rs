use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(InputObject)]
pub struct Repository {
    pub owner: String,
    pub reponame: String,
    pub issuenumber: String,
}

#[derive(InputObject)]
pub struct RepositoryDetail {
    pub owner: String,
    pub reponame: String,
    pub last: usize,
}

#[derive(Serialize)]
pub struct IssueQuery {
    pub query: String,
}

#[derive(InputObject)]
pub struct CreateIssue {
    pub title: String,
    pub repositoryid: String,
    pub body: String,
}

#[derive(InputObject)]
pub struct UpdateIssue {
    pub newtitle: Option<String>,
    pub issueid: String,
    pub newbody: Option<String>,
}

#[derive(Serialize, SimpleObject, Deserialize)]
pub struct ClientMutationId {
    pub client_mutation_id: String,
}

#[derive(InputObject)]
pub struct DeleteIssue {
    pub issue_id: String,
}
#[derive(Serialize, SimpleObject, Deserialize, Debug)]
pub struct Issue {
    pub id: Option<String>,
    pub title: Option<String>,
    pub state: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IssueNode {
    pub nodes: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryIssue {
    pub issues: IssueNode,
}

#[derive(Debug, Serialize)]
pub struct GraphQLQuery {
    pub query: String,
}

#[derive(InputObject)]
pub struct FetchIssue {
    pub issue_id: String,
}

#[derive(InputObject, Serialize, Debug)]
pub struct AddLabelsToLabelable {
    pub issue_id: String,
    pub label_ids: Vec<String>,
}