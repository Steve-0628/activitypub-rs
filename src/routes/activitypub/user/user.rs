use std::sync::Arc;
use axum::{Extension, http::HeaderMap, response::IntoResponse, extract::Path, Json};
use reqwest::{header, StatusCode};
use serde_json::json;
use crate::Config;
use crate::ld::compact_jsonld;

pub(crate) async fn user(config: Extension<Arc<Config>>, Path(userid): Path<String>) -> impl IntoResponse {
    let mut users = config.db.query("select * from users where userid = $userid")
        .bind(("userid", userid)).await.unwrap();

    let user: Option<crate::db::User> = users.take(0).unwrap();

    if let Some(user) = user {
        println!("user: {:?}", user);
        let json = json!(
            [
                {
                    "@id": format!("{}/users/{}", config.domain, user.userid),
                    "@type": [
                        "https://www.w3.org/ns/activitystreams#Person"
                    ],
                    "https://www.w3.org/ns/activitystreams#following": [
                        {
                            "@id": format!("{}/users/{}/following", config.domain, user.userid)
                        }
                    ],
                    "https://www.w3.org/ns/activitystreams#outbox": [
                        {
                            "@id": format!("{}/users/{}/outbox", config.domain, user.userid)
                        }
                    ],
                    "http://www.w3.org/ns/ldp#inbox": [
                        {
                            "@id": format!("{}/users/{}/inbox", config.domain, user.userid)
                        }
                    ],
                    "https://www.w3.org/ns/activitystreams#name": [
                        {
                            "@value": ""
                        }
                    ]
                }
            ]
        );
        let res = compact_jsonld(&mut "http://example.com/".to_string(), &mut json.to_string()).await;
    
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/activity+json".parse().unwrap());

        return Ok((
            headers,
            Json(json!(
                res
            )),
        ));
        
    } else {
        println!("no user match");
        return Err((StatusCode::NOT_FOUND, "Not Found"));
    }
}
