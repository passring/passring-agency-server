use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use serde::Serializer;

#[derive(AsChangeset, Queryable, Selectable, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FullSignature {
    pub id: uuid::Uuid,
    #[serde(with = "hex::serde")]
    pub challenge: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub responses: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub encrypted: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub key_image: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub nonce: Vec<u8>,
    #[serde(serialize_with = "serialize_to_hex_or_null")]
    pub key: Option<Vec<u8>>,
    pub voting_id: uuid::Uuid,
}

fn serialize_to_hex_or_null<S>(data: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match data {
        Some(data) => hex::serialize(data, serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PartialSignature {
    #[serde(with = "hex::serde")]
    pub challenge: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub responses: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub key_image: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub encrypted: Vec<u8>,
    #[serde(with = "hex::serde")]
    pub nonce: Vec<u8>,
    pub voting_id: uuid::Uuid,
}
