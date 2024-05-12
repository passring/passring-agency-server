mod voting;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Vote API", |rocket| async {
        rocket.attach(voting::stage())
    })
}
