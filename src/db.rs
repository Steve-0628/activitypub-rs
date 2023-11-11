use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) userid: String,
    pub(crate) username: String,
    pub(crate) host: String,
    pub(crate) uri: String,
    pub(crate) inbox: String,
    pub(crate) outbox: String,
}
