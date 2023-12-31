use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::StatusCode,
    Extension,
};
use serde_json::json;
use surrealdb::{engine::remote::ws::{Ws, Client}, Surreal, /* opt::auth::Root */}; // TODO: SurrealDBの認証

use std::{net::SocketAddr, sync::Arc};
use utils::ld::string_to_jsonld_json;

mod utils;
mod db;
mod routes;

const DOMAIN: &str = "https://activitypub.local";

#[derive(Clone)]
#[derive(Debug)]
struct Config {
    db: Surreal<Client>,
    domain: String,
    host: String,
}

impl Config {
    async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        db.use_ns("activitypub").use_db("activitypub").await.unwrap();
        Config {
            db,
            domain: DOMAIN.to_string(),
            host: url::Url::parse(DOMAIN).unwrap().host_str().unwrap().to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Arc::new(
        Config::new().await
    );

    db::check_schema(&config.db).await;

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::routes::routes())

        .route("/_kokt", get(kokt))
        .route("/_ste", get(ste))
        // layer は最後
        .layer(Extension(config));
        let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn kokt() -> impl IntoResponse {
    //reqwest with header
    let response = reqwest::Client::new()
        .get("https://simkey.net/users/8rg6sbkjuv/")
        .header("Accept", "application/ld+json")
        .send()
        .await;

    let mut body = response.unwrap().text().await.unwrap();

    let a = string_to_jsonld_json(&mut "https://simkey.net/users/8rg6sbkjuv/".to_string(), &mut body).await;
    (
        StatusCode::OK,
        Json(a)
    )
}

async fn ste() -> impl IntoResponse {
    //https://fedibird.com/users/stesan
    let response = reqwest::Client::new()
        .get("https://fedibird.com/users/stesan")
        .header("Accept", "application/ld+json")
        .send()
        .await;

    let mut body = response.unwrap().text().await.unwrap();

    let a = string_to_jsonld_json(&mut "https://simkey.net/users/8rg6sbkjuv/".to_string(), &mut body).await;
    (
        StatusCode::OK,
        Json(a)
    )
    // (
    //     StatusCode::OK,
    //     Json("OK")
    // )
}

async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
    )
}
