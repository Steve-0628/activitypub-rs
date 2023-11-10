use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) userid: String,
    pub(crate) username: String,
}
