#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod api;
mod db;
mod models;
mod schema;

struct ServerFairing;

#[rocket::async_trait]
impl Fairing for ServerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Server header change",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Server", "passring-agency-server"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(api::stage()).attach(ServerFairing)
}
