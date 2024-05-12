use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::voting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Voting {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub options: Vec<Option<String>>,
    pub active: bool,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::voting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewVoting {
    title: String,
    description: String,
    options: Vec<Option<String>>,
    active: bool,
}
