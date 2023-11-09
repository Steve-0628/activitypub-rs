use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap}
};
use reqwest::header;
use serde_json::json;

use std::net::SocketAddr;
use json_ld::{self, RemoteDocumentReference, JsonLdProcessor, Print, RemoteDocument, syntax::Parse,};
use rdf_types::{self, vocabulary::IriIndex, IriVocabularyMut};
use locspan::{Location, Span};
use iref::IriBuf;
use contextual::WithContext;

const DOMAIN: &str = "http://localhost:3001/";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/.well-known/host-meta/", get(host_meta))
        .route("/.well-known/webfinger", get(webfinger))
        .route("/_kokt", get(kokt))
        .route("/_ste", get(ste));
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

async fn webfinger() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
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
    let body2 = body.clone();
    // println!("{:#?}", serde_json::from_str::<serde_json::Value>(&body2));

    let body_str = serde_json::to_string(&serde_json::from_str::<serde_json::Value>(&body2).unwrap()).unwrap();
    println!("{:#?}", body_str);

    let a = string_to_jsonld_json(&mut "https://simkey.net/users/8rg6sbkjuv/".to_string(), &mut body).await;
    (
        StatusCode::OK,
        Json(a)
    )
}

async fn ste() -> impl IntoResponse {
    // let a = string_to_jsonld_json(&mut "https://honi.stesan.dev/users/93h73zkvw3".to_string()).await;
    (
        StatusCode::OK,
        // Json(a)
        Json("OK")
    )
}

#[derive(Clone)]
#[derive(Debug)]
enum Source {
    Nowhere,
	Iri(IriIndex),
}

async fn string_to_jsonld_json(url: &mut String, body: &mut String) -> serde_json::Value {
    let mut vocabulary: rdf_types::IndexVocabulary = rdf_types::IndexVocabulary::new();
    let mut loader: json_ld::ReqwestLoader<IriIndex, locspan::Location<Source, Span>>
        = json_ld::loader::ReqwestLoader::
            new_with_metadata_map(|_, url, span| {
                Location::new(Source::Iri(*url), span)
            });
    // let mut loader = json_ld::NoLoader::<IriIndex, Span>::new();
    println!("{:?}", url);
    let url = vocabulary.insert(IriBuf::new(url).unwrap().as_iri());
    // let remote_doc: RemoteDocumentReference<IriIndex, Location<Source>> = RemoteDocumentReference::iri(url);
    let remote_doc = RemoteDocumentReference::Loaded(
        RemoteDocument::new(
            Some(url), 
            Some("application/activity+json".parse().unwrap()),
            json_ld::syntax::Value::parse_str(&body, |span| {Location::new(Source::Nowhere, span)}).unwrap(),
        )
    );
    let options: json_ld::Options<IriIndex, locspan::Location<Source>> = json_ld::Options {
        // expansion_policy: json_ld::expansion::Policy::Strictest,
        ..Default::default()
    };
    let result = remote_doc.expand_with_using(&mut vocabulary, &mut loader, options).await;
    println!("aaaaa");
    match result {
        Ok(mut expanded) => {
            expanded.canonicalize();
            return serde_json::from_str(&expanded.with(&vocabulary).inline_print().to_string()).unwrap()
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return json!(format!("{:?}", e))
        }
    }

    // let remote_local_doc = RemoteDocument::new(
    //     Some(iri!("https://simkey.net/users/8rg6sbkjuv/").to_owned()),
    //     Some("application/activity+json".parse().unwrap()),
    //     json_ld::syntax::Value::parse_str(r#""#, |span| span).unwrap()
    // );
    // let mut local_loader = json_ld::NoLoader::<IriBuf, Span>::new();
    // let expanded = remote_local_doc.expand(&mut local_loader).await;
    // match expanded {
    //     Ok(mut expanded) => {
    //         expanded.canonicalize();
    //         // expanded
    //         // return serde_json::from_str(&expanded.with(&vocabulary).inline_print().to_string()).unwrap()
    //         // serde_json::from_value(expanded);
    //         println!("{:?}", expanded);
    //         expanded.objects();

    //         return serde_json::from_str("{}").unwrap()
    //     }
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //         return json!(e.to_string())
    //     }
    // }
}


async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
    )
}
