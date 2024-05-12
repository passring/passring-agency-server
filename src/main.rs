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

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    crate::db::run_migrations(&mut crate::db::establish_connection()).unwrap();
    rocket::build()
        .attach(api::stage())
        .attach(ServerFairing)
        .attach(cors.to_cors().unwrap())
}
