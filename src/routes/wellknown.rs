use axum::{
    extract::Query,
    response::Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
    Extension,
    Router,
    routing::get
};
use reqwest::header;
use serde::Deserialize;
use serde_json::json;

use regex::Regex;
use url::Url;

use std::sync::Arc;

use crate::Config;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/.well-known/host-meta/", get(host_meta))
        .route("/.well-known/webfinger/", get(webfinger))
}

#[derive(Deserialize, Debug)]
pub(crate) struct WebfingerQuery {
    resource: Option<String>,
}

pub(crate) async fn webfinger(config: Extension<Arc<Config>>, resource: Query<WebfingerQuery>) -> impl IntoResponse {
    println!("{:#?}", config);
    match &resource.resource {
        Some(r) => {
            //regex to match acct:username@domain and @username@domain and extract
            let re = Regex::new(r"(acct:|@)(?<username>[\w]+)@(?<domain>[\w\-\.]+\.?[\w-]+)").unwrap();
            match re.captures(r) {
                Some(cap) => {
                    println!("username: {}", &cap["username"]);
                    println!("domain: {}", &cap["domain"]);

                    if Url::parse(&config.domain).unwrap().host().unwrap().to_string() != &cap["domain"] {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": "Error: not me"}))
                        )
                    }

                    let mut users = config.db.query("select * from users where userid = $userid")
                        .bind(("userid", &cap["username"])).await.unwrap();

                    let user: Option<crate::db::User> = users.take(0).unwrap();

                    match user {
                        Some(user) => {
                            println!("user: {:?}", user);
                            
                            let resp = json!({
                                "subject": r,
                                "aliases": [
                                    format!("{}/users/{}", &config.domain, &cap["username"])
                                ],
                                "links": [
                                    {
                                        "rel": "self",
                                        "type": "application/activity+json",
                                        "href": format!("{}/users/{}", &config.domain, &cap["username"])
                                    }
                                ]
                            });
                            return (
                                StatusCode::OK,
                                Json(resp)
                            );
                        },
                        None => {
                            println!("no user match");
                        },
                    }
                },
                None => {
                    println!("no regex user match");
                },
            }
        }
        None => {
            println!("no query resource");
        }
    }
    
    (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Error"}))
    )
}

pub(crate) async fn host_meta(config: Extension<Arc<Config>>,) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xrd+xml".parse().unwrap());
    (
        headers,
        format!("<?xml version=\"1.0\"?><XRD xmlns=\"http://docs.oasis-open.org/ns/xri/xrd-1.0\"><Link rel=\"lrdd\" type=\"application/xrd+xml\" template=\"{}/.well-known/webfinger?resource={{uri}}\" /></XRD>", config.domain)
    )
}
