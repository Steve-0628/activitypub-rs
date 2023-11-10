use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::StatusCode,
    Extension,
};
use serde_json::json;
use surrealdb::{engine::remote::ws::{Ws, Client}, Surreal, opt::auth::Root};

use std::{net::SocketAddr, sync::Arc};
use ld::string_to_jsonld_json;

mod ld;
mod db;
mod routes;

const DOMAIN: &str = "http://localhost:3001";

#[derive(Clone)]
#[derive(Debug)]
struct Config {
    db: Surreal<Client>,
    domain: String,
}

impl Config {
    async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        // db.signin(
        //     Root {
        //         username: "root",
        //         password: "root",
        //     }
        // ).await.unwrap();
        db.use_ns("activitypub").use_db("activitypub").await.unwrap();
        // println!("{:#?}", db.version().await.unwrap());
        Config {
            db,
            domain: DOMAIN.to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Arc::new(
        Config::new().await
    );

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::routes::routes())

        .route("/_kokt", get(kokt))
        .route("/_ste", get(ste))
        // layer は最後
        .layer(Extension(config));
        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

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
    (
        StatusCode::OK,
        // Json(a)
        Json("OK")
    )
}

async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
    )
}
