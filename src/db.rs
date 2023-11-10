use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
pub(crate) struct User<'a> {
    pub(crate) id: &'a str,
    pub(crate) username: &'a str,
}
