use axum::{
    extract::Query,
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
    Extension,
};
use reqwest::header;
use serde::Deserialize;
use serde_json::json;

use regex::Regex;
use surrealdb::{engine::remote::ws::{Ws, Client}, Surreal};
use url::Url;

use std::{net::SocketAddr, sync::Arc};
use ld::string_to_jsonld_json;

mod ld;
mod db;

const DOMAIN: &str = "http://localhost:3001";

#[derive(Clone)]
#[derive(Debug)]
struct Db {
    db: Surreal<Client>,
}

impl Db {
    async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        db.use_db("activitypub").await.unwrap();
        Db {
            db,
        }
    }
}

#[tokio::main]
async fn main() {
    let db = Arc::new(
        Db::new().await
    );

    let app = Router::new()
        .route("/", get(root))
        .route("/.well-known/host-meta/", get(host_meta))
        .route("/.well-known/webfinger/", get(webfinger))
        .route("/_kokt", get(kokt))
        .route("/_ste", get(ste))
        // layer は最後
        .layer(Extension(db));
        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn host_meta() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xrd+xml".parse().unwrap());
    (
        headers,
        format!("<?xml version=\"1.0\"?><XRD xmlns=\"http://docs.oasis-open.org/ns/xri/xrd-1.0\"><Link rel=\"lrdd\" type=\"application/xrd+xml\" template=\"{}/.well-known/webfinger?resource={{uri}}\" /></XRD>", DOMAIN)
    )
}

#[derive(Deserialize, Debug)]
struct WebfingerQuery {
    resource: Option<String>,
}

    
pub(crate) async fn webfinger(db: Extension<Arc<Db>>, resource: Query<WebfingerQuery>) -> impl IntoResponse {
    // let user: Vec<db::User> = self.db.select("user").await.unwrap();
    // println!("{:#?}", self.db.select::<db::User>("user"));

    println!("{:#?}", db);
    match &resource.resource {
        Some(r) => {
            //regex to match acct:username@domain and @username@domain and extract
            let re = Regex::new(r"(acct:|@)(?<username>[\w]+)@(?<domain>[\w\-\.]+\.?[\w-]+)").unwrap();
            match re.captures(&r) {
                Some(cap) => {
                    println!("username: {}", &cap["username"]);
                    println!("domain: {}", &cap["domain"]);

                    if Url::parse(DOMAIN).unwrap().host().unwrap().to_string() != &cap["domain"] {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": "Error: not me"}))
                        )
                    }
                    let resp = json!({
                        "subject": r,
                        "aliases": [
                            format!("{}/users/{}", DOMAIN, &cap["username"])
                        ],
                        "links": [
                            {
                                "rel": "self",
                                "type": "application/activity+json",
                                "href": format!("{}/users/{}", DOMAIN, &cap["username"])
                            }
                        ]
                    });
                    return (
                        StatusCode::OK,
                        Json(resp)
                    );
                },
                None => {
                    println!("no match");
                },
            }
        }
        None => {
        }
    }
    
    (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Error"}))
    )
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
