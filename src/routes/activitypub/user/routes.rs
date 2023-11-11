use axum::{Router, routing::{get, post}};
use super::{user, inbox};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/users/:userid", get(user::user))
        .route("/users/:userid/inbox", post(inbox::inbox))
}
