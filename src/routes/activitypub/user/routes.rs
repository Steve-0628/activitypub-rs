use axum::{Router, routing::get};
use super::user;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/users/:username", get(user::user))
}
