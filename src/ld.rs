use json_ld::{self, RemoteDocumentReference, JsonLdProcessor, Print, RemoteDocument, syntax::Parse,};
use rdf_types::{self, vocabulary::IriIndex, IriVocabularyMut};
use locspan::{Location, Span};
use iref::{IriBuf, Iri};
use contextual::WithContext;
use serde_json::json;

#[derive(Clone)]
#[derive(Debug)]
enum Source {
    Nowhere,
	Iri(IriIndex),
}

pub(crate) async fn string_to_jsonld_json(url: &mut String, body: &mut String) -> serde_json::Value {
    let mut vocabulary: rdf_types::IndexVocabulary = rdf_types::IndexVocabulary::new();
    let mut loader: json_ld::ReqwestLoader<IriIndex, locspan::Location<Source, Span>>
        = json_ld::loader::ReqwestLoader::
            new_with_metadata_map(|_, url, span| {
                Location::new(Source::Iri(*url), span)
            });
    println!("{:?}", url);
    let url = vocabulary.insert(IriBuf::new(url).unwrap().as_iri());
    let remote_doc = RemoteDocumentReference::Loaded(
        RemoteDocument::new(
            Some(url),
            Some("application/activity+json".parse().unwrap()),
            json_ld::syntax::Value::parse_str(body, |span| {Location::new(Source::Nowhere, span)}).unwrap(),
        )
    );
    let options: json_ld::Options<IriIndex, locspan::Location<Source>> = json_ld::Options {
        ..Default::default()
    };
    let result = remote_doc.expand_with_using(&mut vocabulary, &mut loader, options).await;
    match result {
        Ok(mut expanded) => {
            expanded.canonicalize();
            return serde_json::from_str(&expanded.with(&vocabulary).inline_print().to_string()).unwrap()
        }
        Err(e) => {
            println!("Error: {:?}", e);
            json!(format!("{:?}", e))
        }
    }
}


pub(crate) async fn compact_jsonld(url: &mut String, body: &mut String) -> serde_json::Value {
    let mut vocabulary: rdf_types::IndexVocabulary = rdf_types::IndexVocabulary::new();
    let mut loader: json_ld::ReqwestLoader<IriIndex, locspan::Location<Source, Span>>
        = json_ld::loader::ReqwestLoader::
            new_with_metadata_map(|_, url, span| {
                Location::new(Source::Iri(*url), span)
            });
    println!("{:?}", url);
    let url_v = vocabulary.insert(IriBuf::new(url).unwrap().as_iri());
    let remote_doc = RemoteDocumentReference::Loaded(
        RemoteDocument::new(
            Some(url_v),
            Some("application/activity+json".parse().unwrap()),
            json_ld::syntax::Value::parse_str(body, |span| {Location::new(Source::Nowhere, span)}).unwrap(),
        )
    );
    let options: json_ld::Options<IriIndex, locspan::Location<Source>> = json_ld::Options {
        ..Default::default()
    };

    // let iris = Iri::new("https://www.w3.org/ns/activitystreams#").unwrap();
    let myiri = Iri::new(&url).unwrap();
    let context_iri = vocabulary.insert(myiri);

    let context: RemoteDocumentReference<IriIndex, Location<Source>, json_ld_syntax::context::Value<_>> = 
        RemoteDocumentReference::context_iri(context_iri);

    let res = remote_doc.compact_with_using(&mut vocabulary, context, &mut loader, options).await;
    match res {
        Ok(res) => {
            return serde_json::from_str(&res.with(&vocabulary).inline_print().to_string()).unwrap()
        },
        Err(e) => {
            println!("Error: {:?}", e);
            json!(format!("{:?}", e))
        },
    }
}