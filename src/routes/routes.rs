use axum::Router;
use super::wellknown;

pub(crate) fn routes() -> Router {
    Router::new()
        .merge(wellknown::routes())
}
