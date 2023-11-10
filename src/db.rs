use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct User<'a> {
    pub(crate) id: &'a str,
    pub(crate) username: &'a str,
}
