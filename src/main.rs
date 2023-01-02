mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::todo_api::{create_todo, delete_todo, get_all_todos, get_todo, update_todo}; //import the handler here
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user}; //import the handler here
use repository::mongodb_repo::MongoRepo;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
    .mount(
        "/",
        routes![
            create_user,
            get_user,
            update_user,
            delete_user,
            get_all_users
            ],
        )
        .mount(
            "/",
            routes![
                create_todo,
                get_todo,
                update_todo,
                delete_todo,
                get_all_todos
                ],
            )
            .manage(db)
            .manage(CORS)
            .attach(CORS)
        }
        