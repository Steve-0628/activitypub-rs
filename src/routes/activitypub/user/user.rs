use std::sync::Arc;
use axum::{Extension, http::HeaderMap, response::IntoResponse, extract::Path, Json};
use reqwest::{header, StatusCode};
use serde_json::json;
use crate::Config;

pub(crate) async fn user(config: Extension<Arc<Config>>, Path(username): Path<String>) -> impl IntoResponse {
    let mut users = config.db.query("select * from users where username = $username")
        .bind(("username", &username)).await.unwrap();

    let user: Option<crate::db::User> = users.take(0).unwrap();

    if let Some(user) = user {
        println!("user: {:?}", user);
        let resp = json!(
            {
                "@context": [
                    "https://www.w3.org/ns/activitystreams",
                    "https://w3id.org/security/v1",
                    {
                        "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
                        "sensitive": "as:sensitive",
                        "Hashtag": "as:Hashtag",
                        "quoteUrl": "as:quoteUrl",
                        "toot": "http://joinmastodon.org/ns#",
                        "Emoji": "toot:Emoji",
                        "featured": "toot:featured",
                        "discoverable": "toot:discoverable",
                        "schema": "http://schema.org#",
                        "PropertyValue": "schema:PropertyValue",
                        "value": "schema:value",
                        "misskey": "https://misskey-hub.net/ns#",
                        "_misskey_content": "misskey:_misskey_content",
                        "_misskey_quote": "misskey:_misskey_quote",
                        "_misskey_reaction": "misskey:_misskey_reaction",
                        "_misskey_votes": "misskey:_misskey_votes",
                        "_misskey_talk": "misskey:_misskey_talk",
                        "isCat": "misskey:isCat",
                        "vcard": "http://www.w3.org/2006/vcard/ns#"
                    }
                ],
                "type": "Person",
                "id": format!("{}/users/{}", config.domain, user.userid),
                "inbox": format!("{}/users/{}/inbox", config.domain, user.userid),
                "outbox": format!("{}/users/{}/outbox", config.domain, user.userid),
                "preferredUsername": user.username
            }
        );
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/activity+json".parse().unwrap());

        return Ok((
            headers,
            Json(json!(
                resp
            )),
        ));
        
    } else {
        println!("no user match: {:?}", &username);
        return Err((StatusCode::NOT_FOUND, "Not Found"));
    }
}
