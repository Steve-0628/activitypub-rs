use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
use struct_field_names_as_array::{self, FieldNamesAsArray};

#[derive(Debug, Serialize, Deserialize, struct_field_names_as_array::FieldNamesAsArray )]
pub(crate) struct User {
    pub(crate) userid: String,
    pub(crate) username: String,
    pub(crate) host: String,
    pub(crate) uri: String,
    pub(crate) inbox: String,
    pub(crate) outbox: String,
    pub(crate) pubkey: String,
    pub(crate) privkey: String,
}

// macro_rules! get_all_fields 

pub(crate) async fn check_schema(db: &Surreal<Client>) {
    db.query("define table users schemafull").await.unwrap();

    db.query("define field userid on table user type string;").await.unwrap();
    // User::FIELD_NAMES_AS_ARRAY.iter().for_each(|field| async {
    //     println!("field: {:?}", field);
    //     db.query(format!("define field {} on table user type string;", field)).await;
    // });
    let size = User::FIELD_NAMES_AS_ARRAY.len();
    for i in 0..size {
        println!("field: {:?}", User::FIELD_NAMES_AS_ARRAY[i].to_string());
        db.query(format!("define field {} on table users type string", User::FIELD_NAMES_AS_ARRAY[i].to_string()))
            .await.unwrap();
    }
    // get_all_fields!()
}
