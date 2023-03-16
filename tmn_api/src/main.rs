#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use uuid::Uuid;
use tmn_base::Tag;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tag")]
fn get_tag() -> Json<Tag> {
    Json(Tag::new(Uuid::new_v4(), "title".to_owned()))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_tag])
}