use crate::db::establish_connection;
use crate::models::vote::{self, FullSignature, PartialSignature};
use crate::models::votings::{NewVoting, Voting};
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/")]
async fn get_votings() -> Option<Json<Vec<Voting>>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    voting::table.load(connection).map(Json).ok()
}

#[post("/", data = "<voting>")]
async fn create_voting(voting: Json<NewVoting>) -> Option<Json<Voting>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    diesel::insert_into(voting::table)
        .values(voting.into_inner())
        .get_result(connection)
        .map(Json)
        .ok()
}

#[get("/<id>")]
async fn get_voting(id: uuid::Uuid) -> Option<Json<Voting>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    voting::table.find(id).first(connection).map(Json).ok()
}

#[put("/<id>", data = "<voting>")]
async fn update_voting(id: uuid::Uuid, voting: Json<NewVoting>) -> Option<Json<Voting>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    diesel::update(voting::table.find(id))
        .set(voting.into_inner())
        .get_result(connection)
        .map(Json)
        .ok()
}

#[get("/<id>/vote")]
async fn get_votes(id: uuid::Uuid) -> Option<Json<Vec<FullSignature>>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    let _voting: Voting = voting::table.find(id).first(connection).ok()?;

    crate::schema::vote::table
        .filter(crate::schema::vote::voting_id.eq(id))
        .load(connection)
        .map(Json)
        .ok()
}

#[post("/<id>/vote", data = "<vote>")]
async fn create_vote(id: uuid::Uuid, vote: Json<PartialSignature>) -> Option<Json<FullSignature>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    let voting: Voting = voting::table.find(id).first(connection).ok()?;

    if !voting.active {
        return None;
    }

    if vote.voting_id != id {
        return None;
    }

    diesel::insert_into(crate::schema::vote::table)
        .values(vote.into_inner())
        .get_result(connection)
        .map(Json)
        .ok()
}

#[get("/<id>/vote/<vote_id>")]
async fn get_vote(id: uuid::Uuid, vote_id: uuid::Uuid) -> Option<Json<FullSignature>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    let _voting: Voting = voting::table.find(id).first(connection).ok()?;

    crate::schema::vote::table
        .find(vote_id)
        .first(connection)
        .map(Json)
        .ok()
}

#[derive(Deserialize)]
struct Key {
    #[serde(with = "hex::serde")]
    key: Vec<u8>,
}

#[post("/<id>/vote/<vote_id>/key", data = "<key>")]
async fn update_vote(
    id: uuid::Uuid,
    vote_id: uuid::Uuid,
    key: Json<Key>,
) -> Option<Json<FullSignature>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    let voting: Voting = voting::table
        .find(id)
        .first(connection)
        .expect("Voting not found");

    if !voting.active {
        return None;
    }

    let mut vote: FullSignature = crate::schema::vote::table
        .find(vote_id)
        .first(connection)
        .expect("Vote not found");

    if vote.key.is_some() {
        return None;
    }

    // check if the key is valid
    let passring_sig = ::passring::signature::FullSignature {
        voting_id: vote.voting_id,
        challenge: vote.challenge.clone(),
        responses: vote.responses.clone(),
        key_image: vote.key_image.clone(),
        encrypted: vote.encrypted.clone(),
        nonce: vote.nonce.clone(),
        key: key.key.clone(),
    };

    passring_sig.decrypt().expect("Invalid key");

    vote.key = Some(key.key.clone());

    diesel::update(crate::schema::vote::table.find(vote_id))
        .set(&vote)
        .get_result(connection)
        .map(Json)
        .ok()
}

#[get("/<id>/vote/count")]
async fn count_votes(id: uuid::Uuid) -> Option<Json<Vec<u32>>> {
    use crate::schema::voting;
    let connection = &mut establish_connection();

    let _voting: Voting = voting::table.find(id).first(connection).ok()?;

    let votes: Vec<FullSignature> = crate::schema::vote::table
        .filter(crate::schema::vote::voting_id.eq(id))
        .load(connection)
        .ok()?;

    let mut choices_count: Vec<u32> = vec![0; 256];

    for vote in votes {
        if vote.key.is_none() {
            continue;
        }
        let passring_sig = ::passring::signature::FullSignature {
            voting_id: vote.voting_id,
            challenge: vote.challenge.clone(),
            responses: vote.responses.clone(),
            key_image: vote.key_image.clone(),
            encrypted: vote.encrypted.clone(),
            nonce: vote.nonce.clone(),
            key: vote.key.clone().unwrap(),
        };

        let payload = passring_sig.decrypt().expect("Invalid key");

        choices_count[payload.choice as usize] += 1;
    }

    Some(Json(choices_count))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Voting", |rocket| async {
        rocket.mount(
            "/voting",
            routes![
                get_votings,
                create_voting,
                get_voting,
                update_voting,
                get_votes,
                create_vote,
                get_vote,
                update_vote,
                count_votes
            ],
        )
    })
}
