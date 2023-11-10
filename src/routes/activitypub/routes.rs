use axum::Router;
use super::user;


pub(crate) fn routes() -> Router {
    Router::new()
        .merge(user::routes::routes())
}
