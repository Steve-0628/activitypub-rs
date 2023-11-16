use crate::{Config, db::User, utils::signature};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse, Extension, Json};
use chrono::Utc;
use reqwest::{header, StatusCode};
use serde_json::{json, Value};
use std::{str::FromStr, sync::Arc};
use url::Url;

pub(crate) async fn inbox(
    config: Extension<Arc<Config>>,
    Path(userid): Path<String>,
    body: axum::extract::Json<Value>,
) -> impl IntoResponse {
    println!("inbox: {:?}", &userid);
    // println!("body: {:?}", &body.to_string());
    let mut users = config
        .db
        .query("select * from users where userid = $userid and host = $host")
        .bind(("userid", &userid))
        .bind(("host", &config.host))
        .await
        .unwrap();
    let request_type = &body.get("type").expect("Wrong ActivityPub Request");
    println!("request_type: {:?}", request_type.to_string().as_str());

    let user: Option<crate::db::User> = users.take(0).unwrap();

    if let Some(user) = user {
        // println!("user: {:?}", user);
        // let resp = json!(
        //     {
        //         "@context": [
        //             "https://www.w3.org/ns/activitystreams",
        //             "https://w3id.org/security/v1",
        //             {
        //                 "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
        //                 "sensitive": "as:sensitive",
        //                 "Hashtag": "as:Hashtag",
        //                 "quoteUrl": "as:quoteUrl",
        //                 "toot": "http://joinmastodon.org/ns#",
        //                 "Emoji": "toot:Emoji",
        //                 "featured": "toot:featured",
        //                 "discoverable": "toot:discoverable",
        //                 "schema": "http://schema.org#",
        //                 "PropertyValue": "schema:PropertyValue",
        //                 "value": "schema:value",
        //                 "misskey": "https://misskey-hub.net/ns#",
        //                 "_misskey_content": "misskey:_misskey_content",
        //                 "_misskey_quote": "misskey:_misskey_quote",
        //                 "_misskey_reaction": "misskey:_misskey_reaction",
        //                 "_misskey_votes": "misskey:_misskey_votes",
        //                 "_misskey_talk": "misskey:_misskey_talk",
        //                 "isCat": "misskey:isCat",
        //                 "vcard": "http://www.w3.org/2006/vcard/ns#"
        //             }
        //         ],
        //         "type": "Person",
        //         "id": format!("{}/users/{}", config.domain, user.userid),
        //         "inbox": format!("{}/users/{}/inbox", config.domain, user.userid),
        //         "outbox": format!("{}/users/{}/outbox", config.domain, user.userid),
        //         "preferredUsername": user.username
        //     }
        // );

        let remote_actor_uri = body.get("actor").unwrap();
        let mut remote_actor_q = config
            .db
            .query("select * from users where uri = $uri")
            .bind(("uri", remote_actor_uri))
            .await
            .unwrap();

        let remote_actor: Option<crate::db::User> = remote_actor_q.take(0).unwrap();

        let remote_actor = match remote_actor {
            Some(remote_actor) => remote_actor,
            None => {
                println!("no remote_actor match");
                let remote = reqwest::Client::new()
                    .get(remote_actor_uri.as_str().unwrap())
                    .header("Accept", "application/ld+json")
                    .send()
                    .await;

                match remote {
                    Ok(resp) => {
                        let resp_json: Value = resp.json().await.unwrap();
                        // let mut create_res = config.db.query("create users set userid = $userid, username = $username, host = $host, uri = $uri, inbox = $inbox, outbox = $outbox")
                        //     .bind(("userid", resp_json.get("id").unwrap()))
                        //     .bind(("username", resp_json.get("preferredUsername").unwrap()))
                        //     .bind(("host", "aaa"))
                        //     .bind(("uri", resp_json.get("id").unwrap()))
                        //     .bind(("inbox", resp_json.get("inbox").unwrap()))
                        //     .bind(("outbox", resp_json.get("outbox").unwrap()));
                            

                        let remote_user = User {
                            userid: resp_json.get("id").unwrap().as_str().unwrap().to_string(),
                            username: resp_json.get("preferredUsername").unwrap().as_str().unwrap().to_string(),
                            host: url::Url::from_str(resp_json.get("id").unwrap().as_str().unwrap()).unwrap().host_str().unwrap().to_string(),
                            uri: resp_json.get("id").unwrap().as_str().unwrap().to_string(),
                            inbox: resp_json.get("inbox").unwrap().as_str().unwrap().to_string(),
                            outbox: resp_json.get("outbox").unwrap().as_str().unwrap().to_string(),
                            pubkey: resp_json.get("publicKey").unwrap().get("publicKeyPem").unwrap().as_str().unwrap().to_string(),
                            privkey: "REMOTE".to_string(),
                        };
                        let _: Vec<User> = config.db.create("users")
                            .content(&remote_user)
                            
                            .await.unwrap();
                        // let remote_user: Option<crate::db::User> = create_res.first().unwrap();
                        remote_user
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Err((StatusCode::BAD_REQUEST, "Not Found"));
                    }
                }
            }
        };

        println!("remote_actor: {:?}", remote_actor);

        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "application/activity+json".parse().unwrap(),
        );

        match request_type.as_str().unwrap() {
            "Follow" => {
                println!("Follow: {:?}", request_type);

                let mut resp = json!(
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
                        "type": "Accept",
                        "actor": format!("{}/users/{}", config.domain, user.userid),
                        "object": json!(body.0)
                    }
                );
                let obj = resp.get_mut("object").unwrap().as_object_mut().unwrap();
                obj.remove("@context");                
                
                // resp["object"] = body.0.clone();
                // println!("resp: {:?}", resp);
                

                let signing_string = format!("(request-target): post {}\ndate: {}\nhost: {}\ncontent-type: application/activity+json", Url::from_str(&remote_actor.inbox).unwrap().path(), Utc::now().to_rfc3339(), config.host);
                let signature = signature::sign_string_with_privkey(&signing_string, &user.privkey);
                let signature_header = format!("keyId=\"{}#main-key\",algorithm=\"rsa-sha256\",headers=\"(request-target) date host content-type\",signature=\"{}\"", format!("{}/users/{}", config.domain, user.userid), signature);

                // send to remote users' inbox
                let remote_inbox = reqwest::Client::new()
                    .post(&remote_actor.inbox)
                    .header("Content-Type", "application/activity+json")
                    .header("date", Utc::now().to_rfc3339())
                    // .header("host", &config.host)
                    .header("Signature", &signature_header)
                    .header("Authorization", format!("Signature {}", &signature_header))
                    .json(&resp)
                    .send()
                    .await;

                println!("{:?}", &resp.to_string());

                println!("remote_inbox: {:?}", &remote_actor.inbox);
                println!("remote_inbox: {:?}", remote_inbox.unwrap());

                return Ok((headers, Json(json!({}))));
            }
            "Undo" => {
                println!("Undo: {:?}", request_type);
            }
            default => {
                println!("default: {:?}", default);
            }
        }

        return Ok((headers, Json(json!({}))));
    } else {
        println!("no user match: {:?}", &userid);
        return Err((StatusCode::NOT_FOUND, "Not Found"));
    }
}
