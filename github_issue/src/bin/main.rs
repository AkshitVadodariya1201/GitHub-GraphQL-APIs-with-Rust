use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_rocket::GraphQLQuery;
use async_graphql_rocket::GraphQLRequest;
use async_graphql_rocket::GraphQLResponse;
use github_issue::handler::{mutation::Mutation, query::Query, ProjectSchema};
use rocket::{response::content, routes, State};

/// Create a route for the GraphQL query
#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<ProjectSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

// Create a route for the GraphQL mutation
#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_mutation(
    schema: &State<ProjectSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

// Create a route for the GraphQL playground
#[rocket::get("/")]
pub async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

// Launch the Rocket server
#[rocket::launch]
fn rocket() -> _ {
    // Create a new schema
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    // Mount the schema on the Rocket server
    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_mutation, graphql_playground],
    )
}
