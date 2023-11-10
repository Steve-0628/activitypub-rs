use axum::Router;
use super::wellknown;
use super::activitypub;

pub(crate) fn routes() -> Router {
    Router::new()
        .merge(wellknown::routes())
        .merge(activitypub::routes::routes())
}
