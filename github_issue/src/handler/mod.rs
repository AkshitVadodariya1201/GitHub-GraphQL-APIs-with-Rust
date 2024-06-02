use async_graphql::{Schema, EmptySubscription};

use self::{query::Query, mutation::Mutation};

pub mod mutation;
pub mod query;


pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;