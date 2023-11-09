use axum::{
    routing::get,
    Router,
    response::Json,
    response::IntoResponse,
    http::StatusCode
};
use serde_json::json;

use std::net::SocketAddr;
use reqwest;

use json_ld::{self, RemoteDocumentReference, JsonLdProcessor, Print, print};
use rdf_types::{self, vocabulary::IriIndex, IriVocabularyMut};
use locspan::{Location, Span};
use iref::IriBuf;
use contextual::WithContext;

mod ld;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/test", get(test));
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn test() -> impl IntoResponse {
    // GET json-ld https://simkey.net/users/8rg6sbkjuv/ with Accept: application/ld+json

    let response = reqwest::Client::new()
        .get("https://simkey.net/users/8rg6sbkjuv/")
        .header("Accept", "application/ld+json")
        .send()
        .await;
    // let r2 = response;
    // &response.unwrap().text().await.unwrap();
    let body = response.unwrap().text().await;
    let mut body2 = body.unwrap().to_string();
    let a = string_to_jsonld_json(&mut "https://simkey.net/users/8rg6sbkjuv/".to_string()).await;

    (
        StatusCode::OK,
        Json(json!("aaa"))
    )
}

#[derive(Clone)]
#[derive(Debug)]
enum Source {
	Iri(IriIndex),
}

async fn string_to_jsonld_json(str: &mut String) -> () {
    let mut vocabulary: rdf_types::IndexVocabulary = rdf_types::IndexVocabulary::new();
    let mut loader: json_ld::ReqwestLoader<_, Location<Source, Span>> 
        = json_ld::loader::ReqwestLoader::
            new_with_metadata_map(|_, url, span| {
                Location::new(Source::Iri(*url), span)
            });
    println!("{:?}", str);
    let url = vocabulary.insert(IriBuf::new(str).unwrap().as_iri());
    let remote_doc: RemoteDocumentReference<IriIndex, Location<Source>> = RemoteDocumentReference::iri(url);
    let options: json_ld::Options<IriIndex, Location<Source>> = json_ld::Options {
        // expansion_policy: json_ld::expansion::Policy::Strictest,
        ..Default::default()
    };
    let result = remote_doc.expand_with_using(&mut vocabulary, &mut loader, options).await;
    match result {
        Ok(mut expanded) => {
            expanded.canonicalize();
            println!("{}", expanded.with(&vocabulary).pretty_print());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    // serde_json::from_str(str).unwrap()
    // expanded.with
}


async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!("OK"))
    )
}
