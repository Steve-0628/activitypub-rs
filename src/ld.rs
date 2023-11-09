use json_ld::{self, RemoteDocumentReference, JsonLdProcessor, Print, RemoteDocument, syntax::Parse,};
use rdf_types::{self, vocabulary::IriIndex, IriVocabularyMut};
use locspan::{Location, Span};
use iref::IriBuf;
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